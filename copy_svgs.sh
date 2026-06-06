#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Define source directory based on the workspace path
SRC_DIR="benchmark_charts/categories"
DEST_DIR="$1"

# Print usage if no destination directory is specified
if [ -z "$DEST_DIR" ]; then
    echo "Usage: $0 <destination_directory>"
    exit 1
fi

# Check if the source directory exists
if [ ! -d "$SRC_DIR" ]; then
    echo "Error: Source directory '$SRC_DIR' does not exist."
    exit 1
fi

# Ensure the destination directory exists
mkdir -p "$DEST_DIR"

echo "Copying SVG files from '$SRC_DIR' to '$DEST_DIR'..."
echo "Excluding '*avg_time*' and '*capacity_breakdown*' files..."

count=0
while read -r file; do
    # Extract the relative path from the source directory
    rel_path="${file#$SRC_DIR/}"
    dest_file="$DEST_DIR/$rel_path"
    
    # Create target subdirectory and copy the file
    mkdir -p "$(dirname "$dest_file")"
    cp "$file" "$dest_file"
    
    count=$((count + 1))
done < <(find "$SRC_DIR" -type f -name "*.svg" \( ! -name "*avg_time*" -o -name "*1_global_avg_time.svg" \) ! -name "*capacity_breakdown*")

echo "Done! Successfully copied $count SVG files to '$DEST_DIR'."
