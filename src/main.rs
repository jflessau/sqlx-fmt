use anyhow::{Result, bail};
use clap::{Parser, Subcommand};
use log::{error, info, warn};
use sqlx_fmt::fs::find_rust_files;
use std::fs;

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

        /// Literal indentation level
        #[arg(long, default_value = "4", env = "SQLX_FMT_LITERAL_INDENTATION")]
        literal_indentation: usize,

        /// Macros to format, comma separated
        #[arg(long, env = "SQLX_FMT_MACROS")]
        macros: Option<String>,
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

        /// Literal indentation level
        #[arg(long, default_value = "4", env = "SQLX_FMT_LITERAL_INDENTATION")]
        literal_indentation: usize,

        /// Macros to format, comma separated, e.g. "query, query_as, sqlx::query, sqlx::query_as"
        #[arg(long, env = "SQLX_FMT_MACROS")]
        macros: Option<String>,
    },
}

fn main() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("warn,sqlx_fmt=info"),
    )
    .init();
    let cli = Cli::parse();

    match &cli.command {
        Commands::Format {
            path,
            config,
            literal_indentation,
            macros,
        } => {
            if let Err(err) = format_files(path, config, *literal_indentation, macros) {
                error!("error: {err:?}");
                std::process::exit(1);
            }
        }
        Commands::Check {
            path,
            config,
            fail_on_unformatted,
            literal_indentation,
            macros,
        } => {
            if let Err(err) = check_files(
                path,
                config,
                *fail_on_unformatted,
                *literal_indentation,
                macros,
            ) {
                error!("error: {err:?}",);
                std::process::exit(1);
            }
        }
    }
}

fn format_files(
    path: &str,
    config: &str,
    literal_indentation: usize,
    _macros: &Option<String>,
) -> Result<()> {
    info!("formatting files in {path}, with config at {config}");

    let rust_files = find_rust_files(path)?;

    if rust_files.is_empty() {
        println!("no rust files found in {}", path);
        return Ok(());
    }

    let mut formatted_count = 0;

    for file_path in rust_files {
        let content = fs::read_to_string(&file_path)?;
        let formatted_content = sqlx_fmt::format(&content, config, literal_indentation, _macros)?;

        if content != formatted_content {
            fs::write(&file_path, formatted_content)?;
            info!("formatted: {}", file_path);
            formatted_count += 1;
        }
    }

    if formatted_count == 0 {
        info!("all files are already formatted correctly");
    } else {
        info!(
            "formatted {} file{}",
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
    literal_indentation: usize,
    macros: &Option<String>,
) -> Result<()> {
    info!("checking files in {path}, with config at {config}");
    let rust_files = find_rust_files(path)?;

    if rust_files.is_empty() {
        warn!("no rust files found in {}", path);
        return Ok(());
    }

    let mut unformatted_count = 0;

    for file_path in rust_files {
        let content = fs::read_to_string(&file_path)?;
        let formatted = sqlx_fmt::format(&content, config, literal_indentation, macros)?;
        let is_formatted = content == formatted;

        if !is_formatted {
            info!("unformatted: {file_path}");
            unformatted_count += 1;
        }
    }

    if unformatted_count == 0 {
        info!("all files are formatted correctly");
    } else if fail_on_unformatted {
        bail!(
            "{unformatted_count} unformatted file{} found",
            if unformatted_count > 1 { "s" } else { "" }
        );
    }

    Ok(())
}
