#!/bin/bash

# SQLx Format Check - Check if SQL in Rust files needs formatting
# Usage: ./check-format.sh [directory] [config-file]

set -e

# Default values
DIRECTORY="${1:-.}"
CONFIG_FILE="${2:-.sqruff}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SQLX_FMT="$SCRIPT_DIR/../target/release/sqlx-fmt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}SQLx Format Check${NC}"
echo "Directory: $DIRECTORY"
echo "Config: $CONFIG_FILE"
echo "Tool: $SQLX_FMT"

# Check if sqlx-fmt exists
if [ ! -f "$SQLX_FMT" ]; then
    echo -e "${RED}Error: sqlx-fmt not found at $SQLX_FMT${NC}"
    echo "Please build the project first: cargo build --release"
    exit 1
fi

# Check if directory exists
if [ ! -d "$DIRECTORY" ]; then
    echo -e "${RED}Error: Directory '$DIRECTORY' does not exist${NC}"
    exit 1
fi

# Change to target directory
cd "$DIRECTORY"

# Check if config file exists
if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${RED}Error: Config file '$CONFIG_FILE' not found in $(pwd)${NC}"
    exit 1
fi

# Find all Rust files
RUST_FILES=$(find . -name "*.rs" -type f)

if [ -z "$RUST_FILES" ]; then
    echo -e "${YELLOW}No Rust files found in $(pwd)${NC}"
    exit 0
fi

echo -e "${BLUE}Found Rust files:${NC}"
echo "$RUST_FILES"
echo

# Process each file
CHECKED=0
NEEDS_FORMATTING=0
ERRORS=0
FILES_NEEDING_FORMAT=""

for file in $RUST_FILES; do
    echo -n "Checking $file... "

    # Create temporary file for formatted version
    TEMP_FILE=$(mktemp)

    # Run formatter
    if "$SQLX_FMT" --input "$file" --output "$TEMP_FILE"; then
        # Check if file would change
        if cmp -s "$file" "$TEMP_FILE"; then
            echo -e "${GREEN}✓ formatted${NC}"
        else
            echo -e "${RED}✗ needs formatting${NC}"
            FILES_NEEDING_FORMAT="$FILES_NEEDING_FORMAT$file"$'\n'
            NEEDS_FORMATTING=$((NEEDS_FORMATTING + 1))

            # Show diff
            echo -e "${YELLOW}Diff for $file:${NC}"
            diff -u "$file" "$TEMP_FILE" || true
            echo "---"
        fi
        CHECKED=$((CHECKED + 1))
    else
        echo -e "${RED}✗ error${NC}"
        ERRORS=$((ERRORS + 1))
    fi

    # Clean up temp file
    rm -f "$TEMP_FILE"
done

echo
echo -e "${BLUE}Summary:${NC}"
echo "Files checked: $CHECKED"

if [ $NEEDS_FORMATTING -gt 0 ]; then
    echo -e "Files needing formatting: ${RED}$NEEDS_FORMATTING${NC}"
    echo
    echo -e "${YELLOW}Files that need formatting:${NC}"
    echo -n "$FILES_NEEDING_FORMAT"
    echo
    echo "Run ./scripts/format-all.sh to fix formatting issues."
fi

if [ $ERRORS -gt 0 ]; then
    echo -e "Files with errors: ${RED}$ERRORS${NC}"
fi

# Exit with error code if files need formatting or there were errors
if [ $NEEDS_FORMATTING -gt 0 ] || [ $ERRORS -gt 0 ]; then
    exit 1
else
    echo -e "${GREEN}✓ All files are properly formatted!${NC}"
    exit 0
fi
