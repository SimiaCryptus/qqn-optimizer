#!/bin/bash
# run_academic_study.sh

# Create output directory
OUTPUT_DIR="results/study_$(date +%Y%m%d_%H%M%S)"
mkdir -p "$OUTPUT_DIR"

# 1. Run comprehensive benchmarks
echo "Running comprehensive benchmarks..."
cargo test --release benchmark_experiments::test_comprehensive_benchmarks -- --nocapture 2>&1 | tee "$OUTPUT_DIR/benchmark_log.txt"

# 2. Run performance analysis tests to generate LaTeX tables
echo "Generating LaTeX tables..."
cargo test --release performance_analysis::test_latex_table_generation -- --nocapture 2>&1 | tee "$OUTPUT_DIR/latex_tables_log.txt"

# 3. Export academic formats
echo "Exporting academic formats..."
cargo test --release performance_analysis::test_export_academic_formats -- --nocapture 2>&1 | tee "$OUTPUT_DIR/export_log.txt"

# 4. Run integration tests for additional validation
echo "Running integration tests..."
cargo test --release integration_tests -- --nocapture 2>&1 | tee "$OUTPUT_DIR/integration_log.txt"

# 5. Create paper directory structure
echo "Creating paper directory structure..."
mkdir -p paper/tables paper/figures paper/analysis

# 6. Find and copy generated artifacts
echo "Collecting generated artifacts..."

    
# Look for benchmark_results directory in current working directory
if [ -d "benchmark_results" ]; then
    echo "Found benchmark_results directory, copying files..."
    cp -r benchmark_results/* "$OUTPUT_DIR/" 2>/dev/null || true
    
    # Copy specific files to paper directories
    [ -f "benchmark_results/performance_table.tex" ] && cp "benchmark_results/performance_table.tex" paper/tables/
    [ -f "benchmark_results/significance_table.tex" ] && cp "benchmark_results/significance_table.tex" paper/tables/
    [ -f "benchmark_results"/*.png ] && cp "benchmark_results"/*.png paper/figures/ 2>/dev/null || true
    [ -f "benchmark_results"/*.csv ] && cp "benchmark_results"/*.csv paper/analysis/ 2>/dev/null || true
else
    echo "No benchmark_results directory found in current directory"
fi

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