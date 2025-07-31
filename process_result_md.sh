#!/usr/bin/env bash
# Enable error handling and detailed logging
set +e
set +x

# Function to log messages with timestamp
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Function to log errors
log_error() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] ERROR: $1" >&2
}

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
    
   # Create a temporary file with updated links
   temp_file=$(mktemp)
   # Replace .md links with .html links in the markdown content
   # Handle both markdown links [text](link.md) and HTML links <a href="link.md">
   sed -e 's/\(\[[^]]*\]([^)]*\)\.md\()\)/\1.html\2/g' \
       -e 's/\(href="[^"]*\)\.md"/\1.html"/g' \
       -e "s/\(href='[^']*\)\.md'/\1.html'/g" "$md_file" > "$temp_file"
   
   if pandoc "$temp_file" -o "$output_file"; then
      log "Successfully converted: $md_file -> $output_file"
      rm "$temp_file"
      ((processed_count++))
    else
      log_error "Failed to convert: $md_file"
      rm "$temp_file"
      ((error_count++))
    fi
  else
    log "Skipping non-file or non-existent: $md_file"
  fi
done < <(find results -name "*.md" -type f -print0 2>/dev/null)
# Process all TeX files (table exports) in benchmark results
log "Starting processing of TeX table export files..."
# Use find for recursive search of TeX files
while IFS= read -r -d '' tex_file; do
  if [ -f "$tex_file" ]; then
    log "Processing TeX file: $tex_file"
    tex_dir=$(dirname "$tex_file")
    base_name=$(basename "$tex_file" .tex)
    output_file="${tex_dir}/${base_name}.pdf"
    # Skip if output file already exists
    if [ -f "$output_file" ]; then
      log "Skipping $tex_file - output file $output_file already exists"
      continue
    fi
    
    log "Converting $tex_file to $output_file"
    # Use pdflatex to compile TeX to PDF
    # Run in the directory containing the TeX file to handle relative paths
    if (cd "$tex_dir" && pdflatex -interaction=nonstopmode "${base_name}.tex" > /dev/null 2>&1); then
      log "Successfully converted: $tex_file -> $output_file"
      # Clean up auxiliary files created by pdflatex
      rm -f "${tex_dir}/${base_name}.aux" "${tex_dir}/${base_name}.log"
      ((processed_count++))
    else
      log_error "Failed to convert: $tex_file"
      ((error_count++))
    fi
  else
    log "Skipping non-file or non-existent: $tex_file"
  fi
done < <(find results -name "*.tex" -type f -print0 2>/dev/null)


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