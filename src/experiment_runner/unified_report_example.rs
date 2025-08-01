//! Example demonstrating how to use the unified reporting system.
//!
//! This example shows how to create reports using the unified trait,
//! configure them, and generate outputs in multiple formats.
//! 
//! # Examples
//! 
//! See the individual methods for specific usage examples.

use crate::experiment_runner::reports::unified_performance_table::PerformanceTableReport;
use crate::experiment_runner::reports::unified_summary_statistics::SummaryStatisticsReport;
use crate::experiment_runner::{
    Report, ReportCollection, ReportConfig, ReportFormat, UnifiedReportTestSuite,
};
use anyhow::Result;
use std::path::Path;
use std::collections::HashMap;
use tempfile::TempDir;

/// Example usage of the unified reporting system
pub struct UnifiedReportingExample;

impl UnifiedReportingExample {
    /// Generate all reports in multiple formats
    /// 
    /// # Arguments
    /// 
    /// * `output_dir` - Directory where reports will be saved
    /// 
    /// # Returns
    /// 
    /// * `Result<()>` - Success or error if report generation fails
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use std::path::Path;
    /// use qqn_optimizer::experiment_runner::UnifiedReportingExample;
    /// 
    /// UnifiedReportingExample::generate_comprehensive_reports(Path::new("./reports")).unwrap();
    /// ```
    pub fn generate_comprehensive_reports(output_dir: &Path) -> Result<()> {
        // Create test data (in practice, this would come from actual benchmark results)
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        // Create report collection with multiple report types
        let reports = ReportCollection::new()
            .add_report(SummaryStatisticsReport::new())
            .add_report(PerformanceTableReport::new());

        // Generate reports in different formats
        let formats = vec![
            ReportFormat::Html,
            ReportFormat::Markdown,
            ReportFormat::Latex,
            ReportFormat::Csv,
        ];

        for format in formats {
            let format_dir = output_dir.join(format!("{:?}", format).to_lowercase());
            std::fs::create_dir_all(&format_dir)
                .map_err(|e| anyhow::anyhow!("Failed to create directory {:?}: {}", format_dir, e))?;
            
            println!("Generating reports in {:?} format...", format);

            let config = ReportConfig {
                format,
                include_detailed_stats: true,
                include_plots: false, // We don't have plotting in this example
                ..Default::default()
            };

            let metadata_list = reports.generate_all(&data_refs, &config, &format_dir)?;
            println!(
                "Generated {} reports in {:?} format",
                metadata_list.len(),
                config.format
            );
            for metadata in metadata_list {
                println!("  - {} ({})", metadata.report_type, metadata.data_points);
            }
        }

        Ok(())
    }

    /// Example of using a single report with custom configuration
    /// 
    /// This demonstrates how to:
    /// - Create a single report instance
    /// - Configure custom style options
    /// - Generate content programmatically
    /// - Validate the report data
    /// 
    /// # Returns
    /// 
    /// * `Result<String>` - The generated HTML content or error
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use qqn_optimizer::experiment_runner::UnifiedReportingExample;
    /// 
    /// let html_content = UnifiedReportingExample::generate_custom_report().unwrap();
    /// ```
    pub fn generate_custom_report() -> Result<String> {
        let report = SummaryStatisticsReport::new();
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        let mut style_options = std::collections::HashMap::new();
        style_options.insert("theme".to_string(), "dark".to_string());
        style_options.insert("font_size".to_string(), "12px".to_string());
        style_options.insert("highlight_best".to_string(), "true".to_string());
        style_options.insert("color_scheme".to_string(), "performance".to_string());

        let config = ReportConfig {
            format: ReportFormat::Html,
            include_detailed_stats: true,
            include_plots: false,
            style_options,
        };

        let content = report.generate_content(&data_refs, &config)?;

        // Validate the report
        report.validate_data(&data_refs)?;

        // Get metadata
        let metadata = report.get_metadata(&data_refs);
        println!(
            "Generated {} report with {} data points",
            metadata.report_type, metadata.data_points
        );

        Ok(content)
    }

    /// Example of validating reports using the unified test suite
    /// 
    /// This runs comprehensive validation tests on all report types
    /// to ensure they correctly implement the Report trait.
    /// 
    /// # Returns
    /// 
    /// * `Result<()>` - Success if all validations pass, error otherwise
    /// 
    /// # Example
    /// 
    /// ```no_run
    /// use qqn_optimizer::experiment_runner::UnifiedReportingExample;
    /// 
    /// // Run validation in CI/CD or during development
    /// match UnifiedReportingExample::validate_all_reports() {
    ///     Ok(_) => println!("All reports validated successfully"),
    ///     Err(e) => eprintln!("Report validation failed: {}", e),
    /// }
    /// ```
    pub fn validate_all_reports() -> Result<()> {
        println!("Starting comprehensive report validation...\n");
        
        let reports: Vec<Box<dyn Report>> = vec![
            Box::new(SummaryStatisticsReport::new()),
            Box::new(PerformanceTableReport::new()),
        ];

        for report in reports {
            println!("Validating {} report...", report.name());

            // Run comprehensive tests
            UnifiedReportTestSuite::test_basic_functionality(report.as_ref())?;
            UnifiedReportTestSuite::test_content_generation(report.as_ref())?;
            UnifiedReportTestSuite::test_data_validation(report.as_ref())?;
            UnifiedReportTestSuite::test_metadata_generation(report.as_ref())?;
            UnifiedReportTestSuite::test_file_export(report.as_ref())?;
            UnifiedReportTestSuite::test_all_formats(report.as_ref())?;

            println!("✓ {} report validation passed", report.name());
        }
        println!("\n✓ All report validations passed successfully!");
        Ok(())
    }
    /// Example of generating a report with specific problem filtering
    /// 
    /// # Arguments
    /// 
    /// * `problem_filter` - Optional filter to include only specific problems
    /// * `output_path` - Path where the report should be saved
    /// 
    /// # Returns
    /// 
    /// * `Result<()>` - Success or error
    pub fn generate_filtered_report(
        problem_filter: Option<Vec<String>>,
        output_path: &Path,
    ) -> Result<()> {
        let test_data = UnifiedReportTestSuite::create_test_data();
        let mut data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();
        // Apply filter if provided
        if let Some(filter) = problem_filter {
            data_refs.retain(|(problem, _)| filter.contains(&problem.name));
        }
        if data_refs.is_empty() {
            return Err(anyhow::anyhow!("No problems match the filter criteria"));
        }
        let report = PerformanceTableReport::new();
        let config = ReportConfig {
            format: ReportFormat::Markdown,
            include_detailed_stats: true,
            ..Default::default()
        };
        report.export_to_file(&data_refs, &config, output_path)?;
        println!("Generated filtered report with {} problems", data_refs.len());

        Ok(())
    }
    /// Example showing how to create a custom report collection
    pub fn create_custom_collection() -> ReportCollection {
        ReportCollection::new()
            .add_report(SummaryStatisticsReport::new())
            .add_report(PerformanceTableReport::new())
            // Additional reports can be added here as they are implemented
            // .add_report(ConvergenceReport::new())
            // .add_report(ComparisonReport::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comprehensive_report_generation() {
        let temp_dir = TempDir::new().unwrap();
        UnifiedReportingExample::generate_comprehensive_reports(temp_dir.path()).unwrap();

        // Verify output directories and files were created
        for format in &["html", "markdown", "latex", "csv"] {
            let format_dir = temp_dir.path().join(format);
            assert!(
                format_dir.exists(),
                "Format directory should exist: {}",
                format
            );

            // Check that report files were created
            let entries: Vec<_> = std::fs::read_dir(&format_dir).unwrap().collect();
            assert!(
                !entries.is_empty(),
                "Should have generated report files in {}",
                format
            );
        }
    }

    #[test]
    fn test_custom_report_generation() {
        let content = UnifiedReportingExample::generate_custom_report().unwrap();
        assert!(!content.is_empty());
        assert!(content.contains("<!DOCTYPE html>"));
        assert!(content.contains("Summary Statistics Report"));
        assert!(content.contains("dark"), "Should apply custom theme");
    }

    #[test]
    fn test_report_validation() {
        // This should not panic or return errors
        UnifiedReportingExample::validate_all_reports().unwrap();
    }
    #[test]
    fn test_filtered_report_generation() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("filtered_report.md");
        // Test with filter
        let filter = vec!["rosenbrock".to_string(), "sphere".to_string()];
        UnifiedReportingExample::generate_filtered_report(
            Some(filter),
            &output_path
        ).unwrap();
        assert!(output_path.exists());
        let content = std::fs::read_to_string(&output_path).unwrap();
        assert!(content.contains("rosenbrock") || content.contains("sphere"));
    }
    #[test]
    fn test_empty_filter_error() {
        let temp_dir = TempDir::new().unwrap();
        let output_path = temp_dir.path().join("empty_report.md");
        let filter = vec!["nonexistent_problem".to_string()];
        let result = UnifiedReportingExample::generate_filtered_report(Some(filter), &output_path);
        assert!(result.is_err());
    }
}