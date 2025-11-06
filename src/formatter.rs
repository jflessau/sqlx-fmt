use anyhow::{Result, bail};
use log::info;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn sqruff(content: &str, config: &str) -> Result<String> {
    let config_exits = std::path::Path::new(config).exists();
    let mut child = if config_exits {
        Command::new("sqruff")
            .arg("--config")
            .arg(config)
            .arg("fix")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    } else {
        info!("sqruff config file not found at {config}, using default sqruff config");
        Command::new("sqruff")
            .arg("fix")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?
    };

    if let Some(stdin) = child.stdin.as_mut() {
        stdin.write_all(content.trim().as_bytes())?;
    }
    let output = child.wait_with_output()?;
    let formatted = String::from_utf8_lossy(&output.stdout);

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
