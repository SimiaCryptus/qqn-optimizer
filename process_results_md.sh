#!/usr/bin/env bash
# Enable error handling and detailed logging
set +e  # Exit on error
set +x  # Enable debug output (remove this line to reduce verbosity)

# Function to log messages with timestamp
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Function to log errors
log_error() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ERROR: $1" >&2
}

# Cleanup function
cleanup() {
    if [ -n "$temp_file" ] && [ -f "$temp_file" ]; then
        rm -f "$temp_file"
        log "Cleaned up temporary file: $temp_file"
    fi
}

# Set up signal handlers
trap cleanup EXIT
trap 'log_error "Script interrupted by signal"; exit 130' INT TERM

# Check if pandoc is available
if ! command -v pandoc >/dev/null 2>&1; then
    log_error "pandoc is not installed or not in PATH"
    exit 1
fi

# Process all markdown files in benchmark results
log "Starting processing of benchmark result files..."

# Initialize counters
processed_count=0
error_count=0

# Use find for recursive search of markdown files
while IFS= read -r -d '' md_file; do
  if [ -f "$md_file" ]; then
    log "Processing file: $md_file"
    md_dir=$(dirname "$md_file")
    base_name=$(basename "$md_file" .md)
    output_file="${md_dir}/${base_name}.html"

    # Skip if output file already exists
    if [ -f "$output_file" ]; then
      log "Skipping $md_file - output file $output_file already exists"
      continue
    fi

    log "Converting $md_file to $output_file"

    # Check file sizes for debugging
    original_size=$(wc -c < "$md_file")
    log "File sizes - Original: ${original_size} bytes"

    # Run pandoc with timeout and better error handling
    log "Running pandoc conversion..."
    if pandoc "$md_file" -f markdown -t html -o "$output_file" 2>&1; then

        sed -e 's/\(href="[^"]*\)\.md"/\1.html"/g' \
            -e "s/\(href='[^']*\)\.md'/\1.html'/g" \
            -i "$output_file"

        log "Successfully converted: $md_file -> $output_file"
        ((processed_count++))
    else
        pandoc_exit_code=$?
        log_error "Pandoc failed with exit code $pandoc_exit_code for file: $md_file"

        # Check if output file was partially created
        if [ -f "$output_file" ]; then
            log "Removing partially created output file: $output_file"
            rm -f "$output_file"
        fi

        ((error_count++))
    fi

  else
    log "Skipping non-file or non-existent: $md_file"
  fi
done < <(find results/ -name "*.md" -print0 2>/dev/null)

# Final summary
log "Processing complete!"
log "Files processed successfully: $processed_count"
log "Files with errors: $error_count"

if [ $error_count -gt 0 ]; then
    log_error "Some files failed to process. Check the logs above for details."
    exit 1
else
    log "All files processed successfully!"
fi