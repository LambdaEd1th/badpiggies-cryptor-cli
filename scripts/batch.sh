#!/bin/bash

# ==============================================================================
# Bad Piggies Cryptor CLI - Interactive Wrapper Script
# File: scripts/batch.sh
#
# This script is designed to be placed in the "scripts/" directory.
# It automatically searches for the binary in common locations.
# ==============================================================================

# Define the binary name
BIN_NAME="badpiggies-cryptor-cli"

# Attempt to find the executable in the following order:
# 1. Current directory (if moved here or inside release folder)
# 2. Cargo release target directory (development path)
if [ -f "./$BIN_NAME" ]; then
    TOOL="./$BIN_NAME"
elif [ -f "../target/release/$BIN_NAME" ]; then
    TOOL="../target/release/$BIN_NAME"
    echo "Running in development mode (using target/release binary)"
else
    echo "Error: Could not find '$BIN_NAME'."
    echo "Please ensure the project is built (cargo build --release) or the binary is in the current directory."
    exit 1
fi

echo "=========================================="
echo "   Bad Piggies Cryptor - Interactive Tool"
echo "=========================================="
echo "Please select an operation mode:"
echo "1) Decrypt (Binary -> XML)"
echo "2) Encrypt (XML -> Binary)"
echo "3) Generate Template (Create Progress.dat.xml)"
echo "=========================================="
read -p "Enter number (1-3): " MODE_CHOICE

case $MODE_CHOICE in
    1) CMD="decrypt";;
    2) CMD="encrypt";;
    3)
        echo "Generating template file..."
        $TOOL generate
        echo "Template file generated successfully."
        exit 0
        ;;
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

echo ""
read -e -p "Enter Input File Path (drag & drop allowed): " INPUT_FILE
# Remove quotes if present (common when dragging files in terminal)
INPUT_FILE=$(echo "$INPUT_FILE" | tr -d "'\"")

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: File '$INPUT_FILE' does not exist."
    exit 1
fi

echo ""
read -e -p "Enter Output File Path (leave empty for auto-naming): " OUTPUT_FILE
OUTPUT_FILE=$(echo "$OUTPUT_FILE" | tr -d "'\"")

echo ""
echo "Select File Type:"
echo "1) Game Save (Progress.dat)"
echo "2) Vehicle Blueprint (.contraption)"
read -p "Enter number (1-2): " TYPE_CHOICE

case $TYPE_CHOICE in
    1) FTYPE="progress";;
    2) FTYPE="contraption";;
    *)
        echo "Invalid choice. Exiting."
        exit 1
        ;;
esac

# Construct arguments array
CMD_ARGS=("$CMD" "-i" "$INPUT_FILE" "$FTYPE")

if [ -n "$OUTPUT_FILE" ]; then
    CMD_ARGS+=("-o" "$OUTPUT_FILE")
fi

echo ""
echo "Executing: $TOOL ${CMD_ARGS[*]}"
echo "------------------------------------------"

# Execute the tool
$TOOL "${CMD_ARGS[@]}"

# Check exit status
if [ $? -eq 0 ]; then
    echo "------------------------------------------"
    echo "Operation completed successfully!"
else
    echo "------------------------------------------"
    echo "Operation failed. Please check the file or logs."
fi