pub mod formatter;
pub mod fs;
pub mod tree_sitter;

use anyhow::{Result, bail};

const DEFAULT_MACROS: [&str; 14] = [
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

pub fn format(
    content: &str,
    config: &str,
    literal_indentation: usize,
    macros: &Option<String>,
) -> Result<String> {
    let macros = macros.clone().unwrap_or(DEFAULT_MACROS.join(", "));

    let macros: Vec<String> = macros
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    if macros.is_empty() {
        bail!("no macros like 'query_as, sqlx::query, migrate' specified for formatting");
    }

    if macros.is_empty() {
        bail!("no macros like 'query_as, sqlx::query, migrate' specified for formatting");
    }

    let res = tree_sitter::format_query_macros_literals(
        content,
        literal_indentation,
        macros,
        |sql, _is_raw| formatter::sqruff(sql, config),
    );

    Ok(res)
}
