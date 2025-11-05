# sqlx-fmt

A CLI tool to format SQL code within sqlx macros in Rust files using sqruff.

## Overview

This tool automatically finds sqlx macros in Rust source files, extracts the SQL code, formats it using sqruff, and writes the result back to a new file while preserving the original Rust code structure and indentation.

## Supported sqlx Macros

- `query!`
- `query_unchecked!`
- `query_as!`
- `query_as_unchecked!`
- `query_scalar!`
- `query_scalar_unchecked!`
- `migrate!`

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

## How it Works

1. **Parse**: The tool uses regex to find sqlx macros with raw string literals (`r#"..."`#)
2. **Extract**: SQL content is extracted from each macro
3. **Format**: Each SQL snippet is written to a temporary file and formatted using sqruff
4. **Replace**: The formatted SQL is inserted back into the Rust code with proper indentation
5. **Write**: The result is written to the output file

## Configuration

The tool uses sqruff for SQL formatting. Create a `.sqruff` configuration file in your project root:

```toml
[sqruff]
dialect = postgres
rules = all

[sqruff:indentation]
indent_unit = space
tab_space_size = 4
indented_joins = True
```

## Example

### Before (input)
```rust
sqlx::query!(
    r#"
        select exists (
      		select "id"
      		from "event"
      		where
     			"reason" = 'fall_detection'
                and "deviceID" = $1
     			and (
                    ("hasApprovalByHumanInTheLoop" or (not "needsApprovalByHumanInTheLoop")) and "createdAt" > now() - interval '5 minutes'
                    or
                    ("needsApprovalByHumanInTheLoop" and "createdAt" > now() - interval '120 seconds')
                )
        ) as "exists!"
    "#,
    self.id
)
```

### After (output)
```rust
sqlx::query!(
    r#"
        select exists(
            select "id"
            from "event"
            where
                "reason" = 'fall_detection'
                and "deviceID" = $1
                and (
                    (
                        "hasApprovalByHumanInTheLoop"
                        or (not "needsApprovalByHumanInTheLoop")
                    )
                    and "createdAt" > now() - interval '5 minutes'
                    or
                    (
                        "needsApprovalByHumanInTheLoop"
                        and "createdAt" > now() - interval '120 seconds'
                    )
                )
        ) as "exists!"
    "#,
    self.id
)
```

## Features

- **Preserves Rust Code**: Only SQL within sqlx macros is modified
- **Maintains Indentation**: The base indentation level of the Rust code is preserved
- **Multiple Macros**: Handles multiple sqlx macros in a single file
- **Safe Processing**: Creates new output file instead of modifying input file
- **Error Handling**: Provides clear error messages for common issues

## Development

### Running Tests

```bash
cargo test
```

### Building

```bash
cargo build --release
```

## License

This project is open source. Please check the repository for license details.
