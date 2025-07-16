#!/bin/bash
# run_academic_study.sh

# Create output directory
OUTPUT_DIR="results/study_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

# 1. Run comprehensive benchmarks
echo "Running comprehensive benchmarks..."
cargo test --release test_comprehensive_benchmarks -- --nocapture 2>&1 | tee "$OUTPUT_DIR/benchmark_log.txt"

# 2. Run performance analysis tests to generate LaTeX tables
echo "Generating LaTeX tables..."
cargo test --release test_latex_table_generation -- --nocapture 2>&1 | tee "$OUTPUT_DIR/latex_tables_log.txt"

# 3. Export academic formats
echo "Exporting academic formats..."
cargo test --release test_export_academic_formats -- --nocapture 2>&1 | tee "$OUTPUT_DIR/export_log.txt"

# 4. Run integration tests for additional validation
echo "Running integration tests..."
cargo test --release test_qqn_rosenbrock_optimization test_qqn_vs_lbfgs_sphere_function -- --nocapture 2>&1 | tee "$OUTPUT_DIR/integration_log.txt"

# 5. Create paper directory structure
echo "Creating paper directory structure..."
mkdir -p paper/tables paper/figures paper/analysis

# 6. Find and copy generated artifacts
echo "Collecting generated artifacts..."

    
    
# Look for any benchmark/results directories (with timestamps)
for dir in results/benchmark/results_*; do
    if [ -d "$dir" ]; then
        echo "Found benchmark results directory: $dir"
        # Copy all contents to output directory
        cp -r "$dir"/* "$OUTPUT_DIR/" 2>/dev/null || echo "No files to copy from $dir"
        
        # Copy specific files to paper directories
        [ -f "$dir/performance_table.tex" ] && cp "$dir/performance_table.tex" paper/tables/
        [ -f "$dir/significance_table.tex" ] && cp "$dir/significance_table.tex" paper/tables/
        find "$dir" -name "*.png" -exec cp {} paper/figures/ \; 2>/dev/null || echo "No PNG files found"
        find "$dir" -name "*.csv" -exec cp {} paper/analysis/ \; 2>/dev/null || echo "No CSV files found"
    fi
done

# Look for academic/exports directories
for dir in results/academic/exports_*; do
    if [ -d "$dir" ]; then
        echo "Found academic exports directory: $dir"
        cp -r "$dir"/* "$OUTPUT_DIR/" 2>/dev/null || echo "No files to copy from $dir"
        
        # Copy LaTeX files to paper directories
        find "$dir" -name "*.tex" -exec cp {} paper/tables/ \; 2>/dev/null || echo "No TEX files found"
        find "$dir" -name "*.csv" -exec cp {} paper/analysis/ \; 2>/dev/null || echo "No CSV files found"
    fi
done
# Look for citation/test directories
for dir in results/citation/test_*; do
    if [ -d "$dir" ]; then
        echo "Found citation test directory: $dir"
        cp -r "$dir"/* "$OUTPUT_DIR/" 2>/dev/null || echo "No files to copy from $dir"
    fi
done
# Look for latex_output directories
for dir in results/latex_output_*; do
    if [ -d "$dir" ]; then
        echo "Found LaTeX output directory: $dir"
        cp -r "$dir"/* "$OUTPUT_DIR/" 2>/dev/null || echo "No files to copy from $dir"
        find "$dir" -name "*.tex" -exec cp {} paper/tables/ \; 2>/dev/null || echo "No TEX files found in $dir"
    fi
done

# List what was actually generated
echo ""
echo "Generated files:"
find . -maxdepth 2 -name "results/benchmark/results_*" -o -name "results/academic/exports_*" -o -name "results/citation/test_*" -o -name "results/latex/output_*" | head -20
echo ""
echo "Contents of output directory:"
ls -la "$OUTPUT_DIR" 2>/dev/null || echo "Output directory is empty"
echo ""
echo "Contents of paper directories:"
echo "Tables:"
ls -la paper/tables/ 2>/dev/null || echo "No tables directory"
echo "Figures:"
ls -la paper/figures/ 2>/dev/null || echo "No figures directory"
echo "Analysis:"
ls -la paper/analysis/ 2>/dev/null || echo "No analysis directory"
echo ""

# 7. Generate reproducibility package
echo "Creating reproducibility package..."
zip -r reproducibility.zip \
    Cargo.toml \
    src/ \
    tests/ \
    "$OUTPUT_DIR" \
    paper/ \
    README.md \
    run_academic_study.sh

echo "Academic artifacts generated successfully!"
echo "Results directory: $OUTPUT_DIR"
echo "LaTeX tables: paper/tables/ (if generated)"
echo "Figures: paper/figures/ (if generated)"
echo "Analysis: paper/analysis/ (if generated)"
echo "Reproducibility package: reproducibility.zip"
echo ""
echo "Note: Some artifacts are generated in temporary directories during tests."
echo "Check the log files in $OUTPUT_DIR for specific paths."