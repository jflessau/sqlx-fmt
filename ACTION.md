# SQLx Format GitHub Action

A GitHub Action that checks if SQL code within sqlx macros in Rust files needs formatting using sqruff.

## Usage

```yaml
jobs:
  sql-format-check:
    name: Check SQL Formatting
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: actions-rust-lang/setup-rust-toolchain@v1

      - name: Setup Python for sqruff
        uses: actions/setup-python@v4
        with:
          python-version: "3.11"

      - name: Check SQL formatting
        id: sql-check
        uses: your-org/sqlx-fmt@v1
        with:
          context: "."
          config-file: ".sqruff"
          fail-on-diff: "false"

      - name: Comment on PR if formatting needed
        if: steps.sql-check.outputs.needs-formatting == 'true' && github.event_name == 'pull_request'
        uses: actions/github-script@v7
        with:
          script: |
            const files = `${{ steps.sql-check.outputs.changed-files }}`.split('\n').filter(f => f.trim());
            const comment = `
            ## SQL Formatting Required

            The following files need SQL formatting:
            ${files.map(f => `- \`${f}\``).join('\n')}

            Please run \`sqlx-fmt\` on these files and commit the changes.
            `;

            github.rest.issues.createComment({
              issue_number: context.issue.number,
              owner: context.repo.owner,
              repo: context.repo.repo,
              body: comment
            });

      - name: Fail if formatting needed
        if: steps.sql-check.outputs.needs-formatting == 'true'
        run: |
          echo "SQL formatting is required"
          exit 1
```

## Inputs

| Input          | Description                                                 | Required | Default   |
| -------------- | ----------------------------------------------------------- | -------- | --------- |
| `context`      | Directory containing Rust files and sqruff config           | Yes      | `.`       |
| `config-file`  | Path to sqruff configuration file relative to context       | No       | `.sqruff` |
| `fail-on-diff` | Whether to fail the action if formatting changes are needed | No       | `true`    |

## Outputs

| Output             | Description                                            |
| ------------------ | ------------------------------------------------------ |
| `needs-formatting` | Whether any files need formatting (`true`/`false`)     |
| `changed-files`    | List of files that need formatting (newline separated) |

## Configuration

### Sqruff Configuration

Create a `.sqruff` file in your repository root (or specify a different path):

```toml
[sqruff]
dialect = postgres
rules = all

[sqruff:indentation]
indent_unit = space
tab_space_size = 4
indented_joins = True

[sqruff:layout]
type = postgres
```

### Directory Structure

```
your-repo/
├── .sqruff                 # Sqruff configuration
├── src/
│   ├── main.rs            # Rust files with sqlx macros
│   ├── database.rs
│   └── models/
│       └── user.rs
└── .github/
    └── workflows/
        └── sql-check.yml   # Your workflow file
```

## Local Development

You can also run the formatting check locally using the provided scripts:

### Check formatting (without changes)

```bash
./scripts/check-format.sh ./src .sqruff
```

### Format all files

```bash
./scripts/format-all.sh ./src .sqruff
```

## Examples

### Before Formatting

```rust
sqlx::query!(
    r#"
        select exists (
      		select "id"
      		from "event"
      		where
     			"reason" = 'fall_detection'
                and "deviceID" = $1
    "#,
    device_id
)

sqlx::migrate!("select 1 from    test")
```

### After Formatting

```rust
sqlx::query!(
    r#"
        select exists(
            select "id"
            from "event"
            where
                "reason" = 'fall_detection'
                and "deviceID" = $1
    "#,
    device_id
)

sqlx::migrate!("select 1 from test")
```

## Supported SQLx Macros

- `query!`
- `query_unchecked!`
- `query_as!`
- `query_as_unchecked!`
- `query_scalar!`
- `query_scalar_unchecked!`
- `migrate!`

## Requirements

- Rust toolchain (for building sqlx-fmt)
- Python 3.11+ (for sqruff)
- sqruff configuration file

## Troubleshooting

### Common Issues

1. **"Config file not found"**
   - Ensure your `.sqruff` file exists in the specified context directory
   - Check the `config-file` input parameter

2. **"No Rust files found"**
   - Verify the `context` directory contains `.rs` files
   - Check file permissions

3. **"sqruff command failed"**
   - Verify your `.sqruff` configuration is valid
   - Check sqruff installation and version

### Debug Mode

Enable debug output by setting the `ACTIONS_STEP_DEBUG` secret to `true` in your repository.

## Contributing

This action is part of the sqlx-fmt project. See the main README for contribution guidelines.

## License

See the main project license.
