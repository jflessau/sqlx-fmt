# sqlx-fmt

A CLI tool and GitHub Action to format SQL code within [sqlx](https://github.com/launchbadge/sqlx) macros in Rust files using [sqruff](https://github.com/quarylabs/sqruff?tab=readme-ov-file).  
Supports raw string literals (`r#"..."#`) only.

**Supported sqlx Macros**: `query!`, `query_unchecked!`, `query_as!`, `query_as_unchecked!`, `query_scalar!`, `query_scalar_unchecked!`, `migrate!`

## Prerequisites

- [sqruff](https://github.com/quarylabs/sqruff) installed and available in PATH
- A `.sqruff` configuration file

<details>
<summary><b>Demo .sqruff</b></summary>
```toml
[sqruff]
dialect = sqlite
exclude_rules = AM01,AM02
rules = all

[sqruff:indentation]
indent_unit = space
tab_space_size = 4
indented_joins = True

````
</details>

## Usage

Build or install it with cargo:

```bash
# build
cargo build --release

# install
cargo install --path .
````

Then run the formatter like this:

```bash
# format files
sqlx-fmt format --path path_to_files_to_be_formatted --config path_to_sqruff_config

# check formatting
sqlx-fmt check --path path_to_files_to_be_formatted --config path_to_sqruff_config --check
```

### Commands

- `format`: Formats SQL code within sqlx macros in the specified Rust files.
- `check`: Checks if the SQL is formatted correctly without making changes.

### Example

```bash
sqlx-fmt format --path ./src --config .sqruff
```

## Development

TDD is encouraged! To run the tests, use:

```bash
cargo test
```

## GitHub Action

Use the format checker as a step in a GitHub Actions like so:

```yaml
steps:
  - name: checkout code
    uses: actions/checkout@v4

  - name: run format checker
    uses: jflessau/sqlx-fmt@v1
    with:
      context: "./code_to_format"
      config-file: "./code_to_format/.sqruff"
      fail-on-unformatted: "false"
```
