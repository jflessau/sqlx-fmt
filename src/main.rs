use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "sqlx-fmt")]
#[command(about = "A CLI tool to format SQL in sqlx macros")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Format SQL in sqlx macros in Rust files
    Format {
        /// Directory or file path to format
        #[arg(long, default_value = ".", env = "SQLX_FMT_PATH")]
        path: String,

        /// Path to sqruff config file
        #[arg(long, default_value = ".sqruff", env = "SQLX_FMT_SQRUFF_CONFIG")]
        config: String,
    },
    /// Check SQL formatting in sqlx macros in Rust files
    Check {
        /// Directory or file path to check
        #[arg(long, default_value = ".", env = "SQLX_FMT_PATH")]
        path: String,

        /// Path to sqruff config file
        #[arg(long, default_value = ".sqruff", env = "SQLX_FMT_SQRUFF_CONFIG")]
        config: String,

        /// Fail if any unformatted files are found (default is true)
        #[arg(long, default_value = "true", env = "SQLX_FMT_FAIL_ON_UNFORMATTED")]
        fail_on_unformatted: bool,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Format { path, config } => {
            if let Err(err) = format_files(path, config) {
                eprintln!("Error: {err:?}");
                std::process::exit(1);
            }
        }
        Commands::Check {
            path,
            config,
            fail_on_unformatted,
        } => {
            if let Err(err) = check_files(path, config, *fail_on_unformatted) {
                eprintln!("Error: {err:?}",);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn format_files(path: &str, config: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Formatting files in {path}, with config at {config}");

    let rust_files = find_rust_files(path)?;

    if rust_files.is_empty() {
        println!("No Rust files found in {}", path);
        return Ok(());
    }

    let mut formatted_count = 0;

    for file_path in rust_files {
        let content = fs::read_to_string(&file_path)?;
        let formatted_content = sqlx_fmt::format(&content, config)?;

        if content != formatted_content {
            fs::write(&file_path, formatted_content)?;
            println!("Formatted: {}", file_path);
            formatted_count += 1;
        }
    }

    if formatted_count == 0 {
        println!("All files are already formatted correctly.");
    } else {
        println!(
            "Formatted {} file{}",
            formatted_count,
            if formatted_count > 1 { "s" } else { "" }
        );
    }

    Ok(())
}

fn check_files(
    path: &str,
    config: &str,
    fail_on_unformatted: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Checking files in {path}, with config at {config}");
    let rust_files = find_rust_files(path)?;

    if rust_files.is_empty() {
        println!("No Rust files found in {}", path);
        return Ok(());
    }

    let mut unformatted_count = 0;

    for file_path in rust_files {
        let content = fs::read_to_string(&file_path)?;
        let formatted = sqlx_fmt::format(&content, config)?;
        let is_formatted = content == formatted;

        if !is_formatted {
            println!("Unformatted: {file_path}");
            unformatted_count += 1;
        }
    }

    if unformatted_count == 0 {
        println!("All files are formatted correctly.");
    } else if fail_on_unformatted {
        return Err(format!(
            "{unformatted_count} unformatted file{} found",
            if unformatted_count > 1 { "s" } else { "" }
        )
        .into());
    }

    Ok(())
}

fn find_rust_files(path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = Path::new(path);
    let mut rust_files = Vec::new();

    if path.is_file() {
        if let Some(extension) = path.extension() {
            if extension == "rs" {
                rust_files.push(path.to_string_lossy().to_string());
            }
        }
    } else if path.is_dir() {
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(extension) = file_path.extension() {
                    if extension == "rs" {
                        rust_files.push(file_path.to_string_lossy().to_string());
                    }
                }
            }
        }
    } else {
        return Err(format!("Path '{}' does not exist", path.display()).into());
    }

    Ok(rust_files)
}
