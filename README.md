# sqlx-fmt

A CLI tool and GitHub Action to format SQL code within [sqlx](https://github.com/launchbadge/sqlx) macros in Rust files using [sqruff](https://github.com/quarylabs/sqruff?tab=readme-ov-file).

## Prerequisites

You need [Rust](https://rust-lang.org/tools/install/) and [sqruff](https://github.com/quarylabs/sqruff) installed and available in PATH.

## Installation

Install it with cargo: `cargo install --path .`

## Run

```bash
# format files
sqlx-fmt format --path path_to_files --config path_to_sqruff_config

# check formatting
sqlx-fmt check --path path_to_files --config path_to_sqruff_config
```

### Example

```bash
sqlx-fmt format --path ./src --config .sqruff
```

## Development

TDD is encouraged! To run the tests, use `cargo test`.

## GitHub Action

Use the format checker as a step in a GitHub Actions:

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

## License

This project is licensed under the MIT License - see LICENSE file.

This project uses [sqruff](https://github.com/quarylabs/sqruff), which is licensed under
the Apache License 2.0.
