# sqlx-fmt

![CI](https://github.com/jflessau/sqlx-fmt/actions/workflows/ci.yml/badge.svg)


A CLI and GitHub Action to format SQL code within [sqlx](https://github.com/launchbadge/sqlx) macros in Rust files using [sqruff](https://github.com/quarylabs/sqruff?tab=readme-ov-file).

This is not an official sqlx project, just something I always wanted to have.

This project is a WIP.

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
rules = ambiguous,capitalisation,convention,layout,references

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

### Inputs

| Input                 | Required | Default   | Description                                                                      |
| --------------------- | -------- | --------- | -------------------------------------------------------------------------------- |
| `context`             | no       | `.`       | Path to the directory/file to format/check, e.g. `./src`                         |
| `config-file`         | no       | `.sqruff` | Path to the sqruff config file. Default config is used if the file is not found. |
| `fail-on-unformatted` | no       | `true`    | If 'true', the action will fail if any unformatted files are found.              |

## Development

TDD is encouraged! To run the tests, use `cargo test`.

## License

This project is licensed under the MIT License - see LICENSE file.

This project uses [sqruff](https://github.com/quarylabs/sqruff), which is licensed under
the Apache License 2.0.
