use clap::Parser;
use std::fs;

mod formatter;

#[derive(Parser)]
#[command(name = "sqlx-fmt")]
#[command(about = "A CLI tool to format SQL in sqlx macros")]
struct Cli {
    /// Input Rust file containing sqlx macros
    #[arg(short, long)]
    input: String,

    /// Output Rust file to write formatted result
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Read the input file
    let input_content = fs::read_to_string(&cli.input)?;

    // Format the SQL in the content
    let formatted_content = formatter::format_sqlx_file(&input_content)?;

    // Write to output file
    fs::write(&cli.output, formatted_content)?;

    println!("Successfully formatted {} to {}", cli.input, cli.output);
    Ok(())
}
