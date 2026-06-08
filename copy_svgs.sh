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

# Clean up target subdirectories in the destination directory to avoid stale files
for cat in bitsets hashsets trees intervals_roaring vectors_lists; do
    rm -rf "$DEST_DIR/$cat"
done

echo "Copying files from '$SRC_DIR' to '$DEST_DIR'..."
echo "Excluding '*avg_time*' and '*capacity_breakdown*' files..."

count=0
while read -r file; do
    # Extract the relative path from the source directory
    rel_path="${file#$SRC_DIR/}"
    filename="$(basename "$file")"
    
    if [ "$filename" = "4_all_scenarios_time_scaling.svg" ] || [ "$filename" = "5_all_scenarios_time_distribution.svg" ]; then
        # Skip the global category-level charts for 4 and 5
        continue
    elif [ "$filename" = "5_time_distribution.svg" ]; then
        # Chart 5 per scenario: copy PNG instead of SVG
        png_file="${file%.svg}.png"
        rel_png_path="${rel_path%.svg}.png"
        dest_file="$DEST_DIR/$rel_png_path"
        
        # Create target subdirectory and copy the file
        mkdir -p "$(dirname "$dest_file")"
        cp "$png_file" "$dest_file"
        count=$((count + 1))
    else
        # All other charts (including chart 4 per scenario, 1_global_avg_time, etc.): copy SVG
        dest_file="$DEST_DIR/$rel_path"
        
        # Create target subdirectory and copy the file
        mkdir -p "$(dirname "$dest_file")"
        cp "$file" "$dest_file"
        count=$((count + 1))
    fi
done < <(find "$SRC_DIR" -type f -name "*.svg" \( ! -name "*avg_time*" -o -name "*1_global_avg_time.svg" \) ! -name "*capacity_breakdown*")

echo "Done! Successfully copied $count files to '$DEST_DIR'."
