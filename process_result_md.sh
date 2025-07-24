
# Process all markdown files in benchmark results
echo "Processing benchmark result files..."
for md_file in results/benchmark/**/*.md; do
  if [ -f "$md_file" ]; then
    # Get the directory of the markdown file
    md_dir=$(dirname "$md_file")

    # Get the base filename without extension
    base_name=$(basename "$md_file" .md)
    # Get the directory structure for unique naming
    rel_path=$(dirname "${md_file#results/benchmark/}")

    # Create output filename
    if [ "$rel_path" = "." ]; then
      output_name="${md_dir}/${base_name}_tables.tex"
    else
      # Replace slashes with underscores for valid filename
      clean_path=$(echo "$rel_path" | tr '/' '_')
      output_name="${md_dir}/${clean_path}_${base_name}_tables.tex"
    fi

    echo "Processing $md_file -> $output_name"

    # Convert markdown to HTML
    pandoc "$md_file" -o "${md_dir}/${base_name}.html"

    # Extract tables from HTML
    python html2tex.py "${md_dir}/${base_name}.html" -o "$output_name" \
      --caption-prefix "Results from ${base_name}" \
      --label-prefix "tab:${base_name}"
  fi
done
