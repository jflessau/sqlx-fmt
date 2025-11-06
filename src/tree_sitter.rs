use anyhow::Result;
use tree_sitter::{Node, Parser, Range};

pub fn format_query_macros_literals<F>(source: &str, mut replacer: F) -> String
where
    F: FnMut(&str, bool) -> Result<String>, // (literal_content, is_raw) -> replacement
{
    let language: tree_sitter::Language = tree_sitter_rust::LANGUAGE.into();

    let mut parser = Parser::new();
    parser
        .set_language(&language)
        .expect("Error loading Rust grammar");

    let tree = parser
        .parse(source.as_bytes(), None)
        .expect("Failed to parse code");
    let root_node = tree.root_node();

    let mut replacements: Vec<(Range, String)> = Vec::new();

    find_and_collect(
        root_node,
        source.as_bytes(),
        &mut replacer,
        &mut replacements,
    );

    let mut result = source.to_string();
    for (range, replacement) in replacements.into_iter().rev() {
        println!(
            "Applying replacement at range {:?}---\n{}---",
            range, replacement
        );
        let start = range.start_byte;
        let end = range.end_byte;
        result.replace_range(start..end, &replacement);
    }

    result
}

fn unquote_raw_string_literal(lit: &str) -> (&str, usize) {
    // r#"..."#, r##"..."##, etc.
    // Find how many # there are
    let og_hashes = lit[1..].find('"').unwrap();
    println!("og_hashes: {og_hashes}");
    let hashes = &lit[1..=og_hashes];
    let content_start = og_hashes + 2; // hashes + r + opening quote
    let content_end = lit.len() - (og_hashes + 1); // hashes + closing quote
    (&lit[content_start..content_end], hashes.len())
}

fn find_and_collect<'a, F>(
    node: Node<'a>,
    source: &'a [u8],
    replacer: &mut F,
    replacements: &mut Vec<(Range, String)>,
) where
    F: FnMut(&str, bool) -> Result<String>,
{
    let macro_names = vec![
        "migrate",
        "sqlx::migrate",
        "query",
        "sqlx::query",
        "query_unchecked",
        "sqlx::query_unchecked",
        "query_as",
        "sqlx::query_as",
        "query_as_unchecked",
        "sqlx::query_as_unchecked",
        "query_scalar",
        "sqlx::query_scalar",
        "query_scalar_unchecked",
        "sqlx::query_scalar_unchecked",
    ];

    let mut cursor = node.walk();

    for child in node.children(&mut cursor) {
        if child.kind() == "macro_invocation" {
            if let Some(macro_node) = child.child_by_field_name("macro") {
                let macro_name = macro_node.utf8_text(source).unwrap();
                if macro_names.contains(&macro_name) {
                    let mut cursor = child.walk();
                    for macro_child in child.children(&mut cursor) {
                        let mut cursor = &mut macro_child.walk();
                        let Some(raw_string_literal) = macro_child
                            .children(&mut cursor)
                            .find(|n| n.kind() == "raw_string_literal")
                        else {
                            continue;
                        };

                        let literal_text = raw_string_literal.utf8_text(source).unwrap().trim();
                        let literal_text_lines_count = literal_text.lines().count();
                        let (unquoted, hash_count) = unquote_raw_string_literal(literal_text);
                        println!("unquoted: {unquoted}---");

                        let col: usize = raw_string_literal.start_position().column;
                        println!("col: {col}, hash_count: {hash_count}");

                        let replacement = replacer(unquoted, true).unwrap();
                        let replacement_line_count = replacement.lines().count();
                        println!("replacement:--\n{replacement}---");

                        println!(
                            "literal_lines: {literal_text_lines_count}, replacement_lines_count: {replacement_line_count}"
                        );

                        let new_literal =
                            if literal_text_lines_count <= 1 && replacement_line_count > 1 {
                                println!("🚨 RAW_SINGLE_TO_MANY detected");
                                format!(
                                    "{quote}{replacement}\n{unquote}",
                                    quote = format!("r{}\"\n", "#".repeat(hash_count)),
                                    replacement = replacement
                                        .lines()
                                        .map(|line| format!(
                                            "{}{}",
                                            " ".repeat(col.saturating_add(4)),
                                            line
                                        ))
                                        .collect::<Vec<String>>()
                                        .join("\n")
                                        .trim_end(),
                                    unquote =
                                        format!("{}\"{}", " ".repeat(col), "#".repeat(hash_count))
                                )
                            } else if replacement.lines().count() <= 1 {
                                println!("🚨 RAW_SINGLE detected");
                                format!(
                                    "{quote}{reappearance}{unquote}",
                                    quote = format!("r{}\"", "#".repeat(hash_count)),
                                    reappearance = replacement.trim(),
                                    unquote = format!("\"{}", "#".repeat(hash_count))
                                )
                            } else {
                                println!("🚨 RAW_MANY detected");
                                format!(
                                    "{quote}{replacement}\n{unquote}",
                                    quote = format!("r{}\"\n", "#".repeat(hash_count)),
                                    replacement = replacement
                                        .lines()
                                        .map(|line| format!(
                                            "{}{}",
                                            " ".repeat(col.saturating_add(4)),
                                            line
                                        ))
                                        .collect::<Vec<String>>()
                                        .join("\n")
                                        .trim_end(),
                                    unquote =
                                        format!("{}\"{}", " ".repeat(col), "#".repeat(hash_count))
                                )
                            };

                        replacements.push((raw_string_literal.range(), new_literal));
                    }
                }
            }
        }
        find_and_collect(child, source, replacer, replacements);
    }
}
