# sqlx-fmt

A CLI tool and GitHub Action to format SQL code within sqlx macros in Rust files using sqruff. Supports both raw string literals (`r#"..."#`) and regular string literals (`"..."`).

**Supported sqlx Macros**: `query!`, `query_unchecked!`, `query_as!`, `query_as_unchecked!`, `query_scalar!`, `query_scalar_unchecked!`, `migrate!`

## Prerequisites

- Rust toolchain
- [sqruff](https://github.com/quarylabs/sqruff) installed and available in PATH
- A `.sqruff` configuration file in the current directory

## Installation

```bash
cargo build --release
```

## Usage

```bash
sqlx-fmt --input <INPUT_FILE> --output <OUTPUT_FILE>
```

### Arguments

- `--input`, `-i`: Input Rust file containing sqlx macros
- `--output`, `-o`: Output Rust file to write formatted result

### Example

```bash
# Format SQL in a Rust file
./target/release/sqlx-fmt --input src/database.rs --output src/database_formatted.rs
```

## Configuration File

The tool uses sqruff for SQL formatting. Create a `.sqruff` configuration file according to their [documentation](https://github.com/quarylabs/sqruff/tree/main/docs)

## Development

### Running Tests

```bash
cargo test
```

## GitHub Action

This project also provides a GitHub Action for checking SQL formatting in CI/CD pipelines. See the [ACTION.md](ACTION.md) file for details.
