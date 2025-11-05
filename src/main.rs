use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

mod formatter;

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
        #[arg(value_name = "PATH")]
        path: String,
    },
    /// Check if SQL in sqlx macros is properly formatted
    Check {
        /// Directory or file path to check
        #[arg(value_name = "PATH")]
        path: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Format { path } => {
            format_files(path)?;
        }
        Commands::Check { path } => {
            check_files(path)?;
        }
    }

    Ok(())
}

fn format_files(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rust_files = find_rust_files(path)?;

    if rust_files.is_empty() {
        println!("No Rust files found in {}", path);
        return Ok(());
    }

    let mut formatted_count = 0;

    for file_path in rust_files {
        let content = fs::read_to_string(&file_path)?;
        let formatted_content = formatter::format_sqlx_file(&content)?;

        if content != formatted_content {
            fs::write(&file_path, formatted_content)?;
            println!("Formatted: {}", file_path);
            formatted_count += 1;
        }
    }

    if formatted_count == 0 {
        println!("All files are already formatted correctly");
    } else {
        println!("Formatted {} file(s)", formatted_count);
    }

    Ok(())
}

fn check_files(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let rust_files = find_rust_files(path)?;

    if rust_files.is_empty() {
        println!("No Rust files found in {}", path);
        return Ok(());
    }

    let mut unformatted_files = Vec::new();

    for file_path in rust_files {
        let content = fs::read_to_string(&file_path)?;
        let formatted_content = formatter::format_sqlx_file(&content)?;

        if content != formatted_content {
            unformatted_files.push(file_path);
        }
    }

    if unformatted_files.is_empty() {
        println!("All files are properly formatted");
        Ok(())
    } else {
        println!("The following files need formatting:");
        for file in &unformatted_files {
            println!("  {}", file);
        }
        std::process::exit(1);
    }
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
