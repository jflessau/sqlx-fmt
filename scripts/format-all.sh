#!/bin/bash

# SQLx Format All - Format SQL in all Rust files in a directory
# Usage: ./format-all.sh [directory] [config-file]

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

echo -e "${BLUE}SQLx Format All${NC}"
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
PROCESSED=0
CHANGED=0
ERRORS=0

for file in $RUST_FILES; do
    echo -n "Processing $file... "

    # Create backup
    BACKUP_FILE="${file}.sqlx-fmt-backup"
    cp "$file" "$BACKUP_FILE"

    # Run formatter
    if "$SQLX_FMT" --input "$file" --output "$file.formatted"; then
        # Check if file changed
        if cmp -s "$file" "$file.formatted"; then
            echo -e "${GREEN}unchanged${NC}"
            rm -f "$file.formatted" "$BACKUP_FILE"
        else
            echo -e "${YELLOW}formatted${NC}"
            mv "$file.formatted" "$file"
            rm -f "$BACKUP_FILE"
            CHANGED=$((CHANGED + 1))
        fi
        PROCESSED=$((PROCESSED + 1))
    else
        echo -e "${RED}error${NC}"
        rm -f "$file.formatted"
        # Restore from backup
        mv "$BACKUP_FILE" "$file"
        ERRORS=$((ERRORS + 1))
    fi
done

echo
echo -e "${BLUE}Summary:${NC}"
echo "Files processed: $PROCESSED"
echo -e "Files changed: ${YELLOW}$CHANGED${NC}"
if [ $ERRORS -gt 0 ]; then
    echo -e "Files with errors: ${RED}$ERRORS${NC}"
fi

if [ $ERRORS -gt 0 ]; then
    exit 1
else
    echo -e "${GREEN}All done!${NC}"
fi
