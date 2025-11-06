pub mod tree_sitter;

use anyhow::{Result, bail};
use std::io::Write;
use std::process::{Command, Stdio};

pub fn format(content: &str, config: &str) -> Result<String> {
    let res =
        tree_sitter::format_query_macros_literals(content, |sql, _is_raw| format_sql(sql, config));

    Ok(res)
}

pub fn format_sql(content: &str, config: &str) -> Result<String> {
    println!("formatting this:\n{content}---");

    // use sqruff to format the SQL

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
        stdin.write_all(content.trim().as_bytes())?;
    }
    let output = child.wait_with_output()?;
    let formatted = String::from_utf8_lossy(&output.stdout);

    // return stderr if formatting failed

    if formatted.trim().is_empty() {
        bail!(
            "failed to format sql, error: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let formatted = formatted.trim_end();
    let formatted = format!("{formatted}\n");

    Ok(formatted.to_string())
}
