use regex::Regex;
use std::fs;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

const NEWLINE: &str = "\n";
const CONFIG_FLAG: &str = "--config";
const SQRUFF_CONFIG: &str = ".sqruff";
const FIX_COMMAND: &str = "fix";
const CHECK_COMMAND: &str = "check";

pub fn format_sqlx_file(input_content: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut result = input_content.to_string();

    // Process raw string literals first
    result = process_raw_string_macros(&result)?;

    Ok(result)
}

fn process_raw_string_macros(content: &str) -> Result<String, Box<dyn std::error::Error>> {
    // use regex to get sqlx macros with raw string literals

    let pattern = r###"(?s)([ \t]*)((query!|query_unchecked!|query_as!|query_as_unchecked!|query_scalar!|query_scalar_unchecked!|migrate!)\()([\w\s]+,)*\s*(r#")((?s).*?)("#)"###;
    let macro_regex = Regex::new(pattern)?;
    let mut result = content.to_string();

    // Process matches in reverse order to avoid offset issues
    let mut matches: Vec<_> = macro_regex.find_iter(content).collect();
    matches.reverse();

    for m in matches {
        if let Some(captures) = macro_regex.captures(m.as_str()) {
            for (n, c) in captures.iter().enumerate() {
                println!("cap {n}: '{}'", c.map_or("", |m| m.as_str()));
            }
            let base_indent = captures.get(1).unwrap().as_str();
            let mac = captures.get(2).unwrap().as_str();
            let query_type = captures.get(4).and_then(|m| Some(m.as_str())).unwrap_or("");
            let literal_prefix = captures.get(5).unwrap().as_str();
            let full_macro_prefix = format!("{mac}{query_type}");
            let sql_content = captures.get(6).unwrap().as_str();
            let macro_suffix = captures.get(7).unwrap().as_str();

            println!("captures: {}", captures.len());
            println!("base_indent: '{}'", base_indent);
            println!("macro_prefix: '{}'", mac);
            println!("query_type: '{}'", query_type);
            println!("literal_prefix: '{}'", literal_prefix);
            println!("full_macro_prefix: '{}'", full_macro_prefix);
            println!("Processing SQL content:\n{}", sql_content);
            println!("macro_suffix: '{}'", macro_suffix);
            println!("6: '{}'", captures.get(6).map_or("", |m| m.as_str()));

            // Format the SQL
            let formatted_sql = format_sql_content(sql_content)?;

            // Apply proper indentation
            let indented_sql = indent_sql(&formatted_sql, base_indent);

            // Reconstruct the macro
            let new_macro =
                format!("{base_indent}{full_macro_prefix}{NEWLINE}{base_indent}{literal_prefix}{NEWLINE}{indented_sql}{NEWLINE}{base_indent}{macro_suffix}");

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
