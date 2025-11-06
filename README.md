# sqlx-fmt

A CLI and GitHub Action to format SQL code within [sqlx](https://github.com/launchbadge/sqlx) macros in Rust files using [sqruff](https://github.com/quarylabs/sqruff?tab=readme-ov-file).

This is not an official sqlx project, just something I always wanted to have.

This project is a WiP. 

## Installation

Install [sqruff](https://github.com/quarylabs/sqruff) if you haven't already: `cargo install sqruff`

Then install sqlx-fmt with:

```bash
git clone https://github.com/jflessau/sqlx-fmt.git
cd sqlx-fmt
cargo install --path .
```

## Usage

```bash
# format files
sqlx-fmt format --path path_to_files

# check formatting
sqlx-fmt check --path path_to_files
```

### Example

```bash
sqlx-fmt format --path ./src --config .sqruff
```

<details>
<summary><b>Example .sqruff config</b></summary>
  
<a href="https://github.com/quarylabs/sqruff/blob/main/docs/rules.md">sqruff config docs</a>

<pre>
[sqruff]
dialect = postgres
rules = all

[sqruff:indentation]
indent_unit = space
tab_space_size = 4
indented_joins = True
</pre>
</details>

## GitHub Action

Use the format checker as a step in GitHub Actions:

```yaml
steps:
  - name: checkout code
    uses: actions/checkout@v4

  - name: run format checker
    uses: jflessau/sqlx-fmt@main
    with:
      context: "./code_to_format"
      config-file: "./code_to_format/.sqruff"
      fail-on-unformatted: "false"
```

## Development

TDD is encouraged! To run the tests, use `cargo test`.

## License

This project is licensed under the MIT License - see LICENSE file.

This project uses [sqruff](https://github.com/quarylabs/sqruff), which is licensed under
the Apache License 2.0.
