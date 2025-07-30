// Integration test for complete report generator functionality
// Tests the entire report generation pipeline using generated data for fast execution
use std::fs;
use std::path::Path;
use tempfile::TempDir;

use qqn_optimizer::benchmarks::evaluation::BenchmarkConfig;
use qqn_optimizer::experiment_runner::{ReportGenerator, UnifiedReportTestSuite};

#[tokio::test]
async fn test_report_generator_complete_pipeline() -> anyhow::Result<()> {
    // Create a temporary directory for test output
    let temp_dir = TempDir::new()?;
    let output_path = temp_dir.path().to_str().unwrap().to_string();

    // Create test data using the existing test suite
    let test_data = UnifiedReportTestSuite::create_test_data();
    
    // Verify we have test data
    assert!(!test_data.is_empty(), "Test data should not be empty");
    assert!(test_data.len() >= 3, "Should have at least 3 test problems");

    // Create ReportGenerator with test configuration
    let config = BenchmarkConfig::default();
    let report_generator = ReportGenerator::new(output_path.clone(), config);

    // Convert test data to the format expected by generate_main_report
    let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

    // Run the complete report generation pipeline
    println!("Generating complete report with generated data...");
    report_generator
        .generate_main_report(&data_refs, false)
        .await?;

    // Verify that the main output directory structure was created
    let output_dir = Path::new(&output_path);
    assert!(output_dir.exists(), "Output directory should exist");

    // Check for expected subdirectories
    let expected_dirs = ["reports", "data", "convergence", "plots", "latex"];
    for dir_name in &expected_dirs {
        let dir_path = output_dir.join(dir_name);
        assert!(
            dir_path.exists(),
            "Directory '{}' should be created",
            dir_name
        );
    }

    // List all generated files for inspection
    fn list_files(dir: &std::path::Path, indent: usize) -> anyhow::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                println!("{}{}", "  ".repeat(indent), path.file_name().unwrap().to_string_lossy());
                if path.is_dir() {
                    list_files(&path, indent + 1)?;
                }
            }
        }
        Ok(())
    }

    println!("Generated files structure:");
    list_files(output_dir, 0)?;

    // Verify unified reports were generated in multiple formats
    let unified_dir = output_dir.join("unified_reports");
    assert!(
        unified_dir.exists(),
        "Unified reports directory should exist"
    );

    // Check that at least some unified reports exist
    let html_dir = unified_dir.join("html");
    let csv_dir = unified_dir.join("csv");
    let latex_dir = unified_dir.join("latex");
    let markdown_dir = unified_dir.join("markdown");
    
    assert!(html_dir.exists(), "HTML reports directory should exist");
    assert!(csv_dir.exists(), "CSV reports directory should exist");
    assert!(latex_dir.exists(), "LaTeX reports directory should exist");
    assert!(markdown_dir.exists(), "Markdown reports directory should exist");

    // Check specific report files exist in HTML format
    let html_summary = html_dir.join("summary_statistics.html");
    assert!(html_summary.exists(), "HTML summary statistics should exist");
    
    let html_performance = html_dir.join("performance_table.html");
    assert!(html_performance.exists(), "HTML performance table should exist");

    // Check CSV format files
    let csv_summary = csv_dir.join("summary_statistics.csv");
    assert!(csv_summary.exists(), "CSV summary statistics should exist");

    // Validate HTML content
    let html_content = fs::read_to_string(html_summary)?;
    assert!(
        html_content.contains("<!DOCTYPE html>"),
        "HTML should contain proper DOCTYPE"
    );
    assert!(
        html_content.len() > 500,
        "HTML should have substantial content"
    );

    // Validate CSV content
    let csv_content = fs::read_to_string(csv_summary)?;
    assert!(
        csv_content.contains("Problem_Family") || csv_content.contains("Optimizer"),
        "CSV should contain expected headers"
    );
    assert!(
        csv_content.lines().count() > 1,
        "CSV should have header plus data rows"
    );

    // Verify report index was created
    let index_file = output_dir.join("report_index.html");
    assert!(index_file.exists(), "Report index.html should be created");

    // Check that the index file has reasonable content
    let index_content = fs::read_to_string(index_file)?;
    assert!(
        index_content.contains("<!DOCTYPE html>"),
        "Index should be valid HTML"
    );
    assert!(
        index_content.len() > 500,
        "Index should have substantial content"
    );

    // Verify legacy reports exist
    let benchmark_report = output_dir.join("benchmark_report.md");
    assert!(benchmark_report.exists(), "Benchmark report should exist");

    // Verify data files exist
    let data_dir = output_dir.join("data");
    let detailed_results = data_dir.join("detailed_results.csv");
    assert!(detailed_results.exists(), "Detailed results CSV should exist");

    let summary_stats = data_dir.join("summary_statistics.csv");
    assert!(summary_stats.exists(), "Summary statistics CSV should exist");

    // Verify LaTeX files exist
    let latex_dir = output_dir.join("latex");
    let comprehensive_tex = latex_dir.join("comprehensive_benchmark_report.tex");
    assert!(comprehensive_tex.exists(), "Comprehensive LaTeX report should exist");

    println!("✓ All expected directories created");
    println!("✓ Report files generated successfully");
    println!("✓ Directory structure created correctly");
    println!("✓ Complete report generation pipeline test passed");

    Ok(())
}

#[tokio::test]
async fn test_report_generator_with_family_mode() -> anyhow::Result<()> {
    // Test the family optimization mode as well
    let temp_dir = TempDir::new()?;
    let output_path = temp_dir.path().to_str().unwrap().to_string();

    let test_data = UnifiedReportTestSuite::create_test_data();
    let config = BenchmarkConfig::default();
    let report_generator = ReportGenerator::new(output_path.clone(), config);
    let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

    // Run with family optimization enabled
    report_generator
        .generate_main_report(&data_refs, true)
        .await?;

    let output_dir = Path::new(&output_path);
    assert!(output_dir.exists(), "Output directory should exist");

    // Verify the same basic structure exists
    let index_file = output_dir.join("report_index.html");
    assert!(index_file.exists(), "Report index.html should be created");

    // Verify unified reports directory exists
    let unified_dir = output_dir.join("unified_reports");
    assert!(unified_dir.exists(), "Unified reports directory should exist");

    println!("✓ Family mode report generation test passed");

    Ok(())
}

#[test]
fn test_report_generator_creation() {
    // Test basic ReportGenerator creation
    let config = BenchmarkConfig::default();
    let _report_generator = ReportGenerator::new("test_output".to_string(), config);
    
    // This is a simple validation that the struct can be created
    // More detailed testing happens in the async tests above
    // Note: output_dir field is private, so we just test successful creation
}