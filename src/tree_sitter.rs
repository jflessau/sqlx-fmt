#![allow(clippy::format_in_format_args)]

use anyhow::{Result, bail};
use log::{debug, error};
use tree_sitter::{Node, Parser, Range};

pub fn format_query_macros_literals<F>(
    source: &str,
    literal_indentation: usize,
    macros_names: Vec<String>,
    mut formatter: F,
) -> String
where
    F: FnMut(&str, bool) -> Result<String>,
{
    // setup rust parser

    let language: tree_sitter::Language = tree_sitter_rust::LANGUAGE.into();
    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("Error loading Rust grammar");
    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("Failed to parse code");
    let root_node = tree.root_node();

    // find and collect replacements

    let mut replacements: Vec<(Range, String)> = Vec::new();

    find_and_collect(
        root_node,
        source.as_bytes(),
        &macros_names,
        literal_indentation,
        &mut formatter,
        &mut replacements,
    );

    // repace unformatted with formatted sql

    let mut result = source.to_string();
    for (range, replacement) in replacements.into_iter().rev() {
        let start = range.start_byte;
        let end = range.end_byte;
        result.replace_range(start..end, &replacement);
    }

    result
}

fn find_and_collect<'a, F>(
    node: Node<'a>,
    source: &'a [u8],
    macro_names: &Vec<String>,
    _literal_indentation: usize,
    formatter: &mut F,
    replacements: &mut Vec<(Range, String)>,
) where
    F: FnMut(&str, bool) -> Result<String>,
{
    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "macro_invocation"
            && let Some(macro_node) = child.child_by_field_name("macro")
        {
            let macro_name = macro_node
                .utf8_text(source)
                .expect("failed to get macro name as utf8");
            if macro_names.contains(&macro_name.to_string()) {
                let mut cursor = child.walk();
                for macro_child in child.children(&mut cursor) {
                    // raw string literal

                    let cursor = &mut macro_child.walk();
                    if let Some(raw_string_literal) = macro_child
                        .children(cursor)
                        .find(|n| n.kind() == "raw_string_literal")
                    {
                        match format_raw_string_literal(source, &raw_string_literal, formatter) {
                            Ok(v) => replacements.push((raw_string_literal.range(), v)),
                            Err(e) => {
                                error!(
                                    "failed to format raw string literal: {:?}, error: {:?}",
                                    raw_string_literal.utf8_text(source),
                                    e
                                );
                            }
                        }
                    }

                    // string literal

                    if let Some(string_literal) = macro_child
                        .children(cursor)
                        .find(|n| n.kind() == "string_literal")
                    {
                        match format_string_literal(source, &string_literal, formatter) {
                            Ok(v) => replacements.push((string_literal.range(), v)),
                            Err(e) => {
                                error!(
                                    "failed to format string literal: {:?}, error: {:?}",
                                    string_literal.utf8_text(source),
                                    e
                                );
                            }
                        }
                    }
                }
            }
        }
        find_and_collect(
            child,
            source,
            macro_names,
            _literal_indentation,
            formatter,
            replacements,
        );
    }
}

fn format_raw_string_literal<'a>(
    source: &'a [u8],
    raw_string_literal: &Node<'a>,
    formatter: &mut impl FnMut(&str, bool) -> Result<String>,
) -> Result<String> {
    let literal = raw_string_literal
        .utf8_text(source)
        .expect("failed to get raw string literal as utf8")
        .trim();

    let literal_text_lines_count = literal.lines().count();
    let (unquoted, hash_count) = unquote_raw_string_literal(literal);

    let col: usize = raw_string_literal.start_position().column;

    let formatter_res = formatter(unquoted, true);
    let Ok(replacement) = formatter_res else {
        bail!(
            "formatter failed to format sql {unquoted}, error: {:?}",
            formatter_res.err()
        );
    };

    let replacement_line_count = replacement.lines().count();

    debug!(
        "raw string literal => col: {col}, literal_lines: {literal_text_lines_count}, replacement_lines_count: {replacement_line_count}"
    );

    let new_literal = if literal_text_lines_count <= 1 && replacement_line_count > 1 {
        debug!("RAW_SINGLE_TO_MANY detected");
        format!(
            "{quote}{replacement}\n{unquote}",
            quote = format!("r{}\"\n", "#".repeat(hash_count)),
            replacement = replacement
                .lines()
                .map(|line| format!("{}{}", " ".repeat(col.saturating_add(4)), line))
                .collect::<Vec<String>>()
                .join("\n")
                .trim_end(),
            unquote = format!("{}\"{}", " ".repeat(col), "#".repeat(hash_count))
        )
    } else if replacement.lines().count() <= 1 {
        debug!("RAW_SINGLE detected");
        format!(
            "{quote}{reappearance}{unquote}",
            quote = format!("r{}\"", "#".repeat(hash_count)),
            reappearance = replacement.trim(),
            unquote = format!("\"{}", "#".repeat(hash_count))
        )
    } else {
        debug!("RAW_MANY detected");
        format!(
            "{quote}{replacement}\n{unquote}",
            quote = format!("r{}\"\n", "#".repeat(hash_count)),
            replacement = replacement
                .lines()
                .map(|line| format!("{}{}", " ".repeat(col.saturating_add(4)), line))
                .collect::<Vec<String>>()
                .join("\n")
                .trim_end(),
            unquote = format!("{}\"{}", " ".repeat(col), "#".repeat(hash_count))
        )
    };

    Ok(new_literal)
}

fn format_string_literal<'a>(
    source: &'a [u8],
    string_literal: &Node<'a>,
    formatter: &mut impl FnMut(&str, bool) -> Result<String>,
) -> Result<String> {
    let literal = string_literal
        .utf8_text(source)
        .expect("failed to get string literal as utf8")
        .trim();

    let literal_text_lines_count = literal.lines().count();
    let unquoted = &literal[1..literal.len() - 1];

    let col: usize = string_literal.start_position().column;

    let formatter_res = formatter(unquoted, true);
    let Ok(replacement) = formatter_res else {
        bail!(
            "formatter failed to format sql {unquoted}, error: {:?}",
            formatter_res.err()
        );
    };

    let replacement_line_count = replacement.lines().count();

    debug!(
        "string literal => col: {col}, literal_lines: {literal_text_lines_count}, replacement_lines_count: {replacement_line_count}"
    );

    let new_literal = format!(
        "\"{replacement}\"",
        replacement = replacement
            .lines()
            .map(|l| l.trim())
            .collect::<Vec<_>>()
            .join(" ")
    );

    Ok(new_literal)
}

fn unquote_raw_string_literal(lit: &str) -> (&str, usize) {
    // r#"..."#, r##"..."##, etc.
    let og_hashes = lit[1..].find('"').expect("invalid raw string literal");
    debug!("og_hashes: {og_hashes}");
    let hashes = &lit[1..=og_hashes];
    let content_start = og_hashes + 2; // 'r' + hashes + opening quote
    let content_end = lit.len() - (og_hashes + 1); // hashes + closing quote
    (&lit[content_start..content_end], hashes.len())
}
