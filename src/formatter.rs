use regex::Regex;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

const NEWLINE: &str = "\n";
const CONFIG_FLAG: &str = "--config";
const SQRUFF_CONFIG: &str = ".sqruff";
const FIX_COMMAND: &str = "fix";

pub fn format_sqlx_file(input_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = input_content.to_string();

    // Process raw string literals first
    result = process_raw_string_macros(&result)?;

    // Then process regular string literals
    result = process_regular_string_macros(&result)?;

    Ok(result)
}

fn process_raw_string_macros(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Pattern to match sqlx macros with raw string literals
    let pattern = r###"(?s)([ \t]*)(sqlx::(query!|query_unchecked!|query_as!|query_as_unchecked!|query_scalar!|query_scalar_unchecked!|migrate!)\s*\(\s*r#")(.+?)([ \t]*"#)"###;
    let macro_regex = Regex::new(pattern)?;

    let mut result = content.to_string();

    // Process matches in reverse order to avoid offset issues
    let mut matches: Vec<_> = macro_regex.find_iter(content).collect();
    matches.reverse();

    for m in matches {
        if let Some(captures) = macro_regex.captures(m.as_str()) {
            let base_indent = captures.get(1).unwrap().as_str();
            let macro_prefix = captures.get(2).unwrap().as_str();
            let sql_content = captures.get(4).unwrap().as_str();
            let macro_suffix = captures.get(5).unwrap().as_str();

            // Format the SQL
            let formatted_sql = format_sql_content(sql_content)?;

            // Apply proper indentation
            let indented_sql = indent_sql(&formatted_sql, base_indent);

            // Reconstruct the macro
            let new_macro = format!(
                "{}{}{}{}{}{}",
                base_indent, macro_prefix, NEWLINE, indented_sql, NEWLINE, macro_suffix
            );

            // Replace in result
            let start = m.start();
            let end = m.end();
            result.replace_range(start..end, &new_macro);
        }
    }

    Ok(result)
}

fn process_regular_string_macros(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Pattern to match sqlx macros with regular string literals
    let pattern = r###"([ \t]*)(sqlx::(query!|query_unchecked!|query_as!|query_as_unchecked!|query_scalar!|query_scalar_unchecked!|migrate!)\s*\(\s*)"([^"]+)"(.*)"###;
    let macro_regex = Regex::new(pattern)?;

    let mut result = content.to_string();

    // Process matches in reverse order to avoid offset issues
    let mut matches: Vec<_> = macro_regex.find_iter(content).collect();
    matches.reverse();

    for m in matches {
        if let Some(captures) = macro_regex.captures(m.as_str()) {
            let base_indent = captures.get(1).unwrap().as_str();
            let macro_part = captures.get(2).unwrap().as_str();
            let sql_content = captures.get(4).unwrap().as_str();
            let closing_part = captures.get(5).unwrap().as_str();

            // Format the SQL
            let formatted_sql = format_sql_content(sql_content)?;

            // For regular strings, keep them simple - just replace the SQL content
            let new_macro = if formatted_sql.contains('\n') {
                // Multi-line: use indented format
                let indented_sql = indent_sql(&formatted_sql, base_indent);
                format!(
                    "{}{}\"{}{}{}\"\n{}{}",
                    base_indent,
                    macro_part.trim_start(),
                    NEWLINE,
                    indented_sql,
                    NEWLINE,
                    base_indent,
                    closing_part
                )
            } else {
                // Single line: keep it simple
                format!(
                    "{}{}\"{}\"{}",
                    base_indent,
                    macro_part.trim_start(),
                    formatted_sql,
                    closing_part
                )
            };

            // Replace in result
            let start = m.start();
            let end = m.end();
            result.replace_range(start..end, &new_macro);
        }
    }

    Ok(result)
}

fn format_sql_content(sql: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Create temporary file
    let mut temp_file = NamedTempFile::new()?;
    temp_file.write_all(sql.trim().as_bytes())?;

    // Get the path as string
    let temp_path = temp_file.path().to_string_lossy().to_string();

    // Run sqruff fix - it modifies the file in place
    let output = Command::new("sqruff")
        .arg(CONFIG_FLAG)
        .arg(SQRUFF_CONFIG)
        .arg(FIX_COMMAND)
        .arg(&temp_path)
        .output()?;

    // sqruff fix returns non-zero exit code even on successful fixes
    // so we check if the file exists and read it regardless
    let formatted = fs::read_to_string(temp_file.path())?;

    // Only report error if we couldn't read the formatted file
    if formatted.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("sqruff warning: {}", stderr);
    }

    Ok(formatted.trim().to_string())
}

fn indent_sql(sql: &str, base_indent: &str) -> String {
    let lines: Vec<&str> = sql.lines().collect();
    let mut result = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if line.trim().is_empty() {
            result.push(String::new());
        } else if i == 0 {
            // First line gets base indent + 8 spaces
            result.push(format!("{}        {}", base_indent, line.trim()));
        } else {
            // Subsequent lines preserve relative indentation
            let trimmed = line.trim_start();
            let original_spaces = line.len() - trimmed.len();
            let indent_level = (original_spaces + 3) / 4; // Convert to 4-space units
            let extra_indent = "    ".repeat(indent_level);
            result.push(format!(
                "{}        {}{}",
                base_indent, extra_indent, trimmed
            ));
        }
    }

    result.join(NEWLINE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indent_sql() {
        let sql = "SELECT id,\n    name\nFROM users\nWHERE active = true";
        let base_indent = "    ";

        let result = indent_sql(&sql, base_indent);
        println!("Result: '{}'", result);

        // Check that first line has correct indentation
        assert!(result.contains("        SELECT"));
        // Check that indented line has extra indentation
        assert!(result.contains("            name"));
    }

    #[test]
    fn test_raw_string_regex_matching() {
        let content = "    sqlx::query!(\n        r#\"\n            select id from users\n        \"#,\n        user_id\n    )";

        let pattern = r###"(?s)([ \t]*)(sqlx::(query!|query_unchecked!|query_as!|query_as_unchecked!|query_scalar!|query_scalar_unchecked!|migrate!)\s*\(\s*r#")(.+?)([ \t]*"#)"###;
        let regex = Regex::new(pattern).unwrap();

        let captures = regex.captures(content).unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "    ");
        assert!(captures.get(2).unwrap().as_str().contains("sqlx::query!"));
        let users_check = "select id from users";
        assert!(captures.get(4).unwrap().as_str().contains(users_check));
    }

    #[test]
    fn test_regular_string_regex_matching() {
        let content = "sqlx::migrate!(\"select 1 from    test\")";

        let pattern = r###"([ \t]*)(sqlx::(query!|query_unchecked!|query_as!|query_as_unchecked!|query_scalar!|query_scalar_unchecked!|migrate!)\s*\(\s*)"([^"]+)"(.*)"###;
        let regex = Regex::new(pattern).unwrap();

        let captures = regex.captures(content).unwrap();
        assert_eq!(captures.get(1).unwrap().as_str(), "");
        assert!(captures.get(2).unwrap().as_str().contains("sqlx::migrate!"));
        let test_check = "select 1 from    test";
        assert_eq!(captures.get(4).unwrap().as_str(), test_check);
    }
}
