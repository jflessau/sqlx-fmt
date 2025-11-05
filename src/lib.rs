use regex::Regex;
use std::io::Write;
use std::process::{Command, Stdio};

const NEWLINE: &str = "\n";

pub fn format(content: &str, config: &str) -> Result<String, Box<dyn std::error::Error>> {
    // use regex to get sqlx macros with raw string literals

    let pattern = r###"(?s)((query!|query_unchecked!|query_as!|query_as_unchecked!|query_scalar!|query_scalar_unchecked!|migrate!)\()\n*(\s*)([\w\s]+,)*(\s*)(r#")((?s).*?)("#)"###;
    let macro_regex = Regex::new(pattern)?;
    let mut result = content.to_string();

    // Process matches in reverse order to avoid offset issues
    let mut matches: Vec<_> = macro_regex.find_iter(content).collect();
    matches.reverse();

    for m in matches {
        if let Some(captures) = macro_regex.captures(m.as_str()) {
            let base_indent = captures.get(5).unwrap().as_str();
            let base_indent =
                if base_indent.is_empty() || base_indent.chars().all(|c| c.is_whitespace()) {
                    captures.get(3).unwrap().as_str()
                } else {
                    ""
                };
            let mac = captures.get(1).unwrap().as_str();
            let query_type = captures.get(4).map(|m| m.as_str()).unwrap_or("");
            let query_type = if query_type.contains(',') {
                format!("{NEWLINE}{base_indent}{query_type}")
            } else {
                query_type.to_string()
            };
            let literal_prefix = captures.get(6).unwrap().as_str();
            let full_macro_prefix = format!("{mac}{query_type}");
            let sql_content = captures.get(7).unwrap().as_str();
            let macro_suffix = captures.get(8).unwrap().as_str();

            // Format the SQL
            let formatted_sql = format_sql_content(sql_content, config)?;

            // Apply proper indentation
            let indented_sql = indent_sql(&formatted_sql, base_indent);

            // Reconstruct the macro
            let new_macro =
                format!("{full_macro_prefix}{NEWLINE}{base_indent}{literal_prefix}{NEWLINE}{indented_sql}{NEWLINE}{base_indent}{macro_suffix}");

            // Replace in result
            let start = m.start();
            let end = m.end();
            result.replace_range(start..end, &new_macro);
        }
    }

    Ok(result)
}

fn format_sql_content(sql: &str, config: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut child = Command::new("sqruff")
        .arg("--config")
        .arg(config)
        .arg("fix")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(sql.trim().as_bytes())?;
    }

    let output = child.wait_with_output()?;

    let formatted = String::from_utf8_lossy(&output.stdout);

    if formatted.trim().is_empty() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("sqruff warning: {}", stderr);
        return Ok(sql.trim().to_string());
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
            result.push(format!("{}    {}", base_indent, line.trim()));
        } else {
            let trimmed = line.trim_start();
            let original_spaces = line.len() - trimmed.len();
            let indent_level = original_spaces.div_ceil(4);
            let extra_indent = "    ".repeat(indent_level);
            result.push(format!("{}    {}{}", base_indent, extra_indent, trimmed));
        }
    }

    result.join(NEWLINE)
}
