//! Example demonstrating how to use the unified reporting system.
//!
//! This example shows how to create reports using the unified trait,
//! configure them, and generate outputs in multiple formats.

use crate::experiment_runner::{
    Report, ReportCollection, ReportConfig, ReportFormat, UnifiedReportTestSuite
};
use crate::experiment_runner::reports::unified_summary_statistics::SummaryStatisticsReport;
use crate::experiment_runner::reports::unified_performance_table::PerformanceTableReport;
use anyhow::Result;
use std::path::Path;
use tempfile::TempDir;

/// Example usage of the unified reporting system
pub struct UnifiedReportingExample;

impl UnifiedReportingExample {
    /// Generate all reports in multiple formats
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
            std::fs::create_dir_all(&format_dir)?;
            
            let config = ReportConfig {
                format,
                include_detailed_stats: true,
                include_plots: false, // We don't have plotting in this example
                ..Default::default()
            };
            
            let metadata_list = reports.generate_all(&data_refs, &config, &format_dir)?;
            
            println!("Generated {} reports in {:?} format", metadata_list.len(), config.format);
            for metadata in metadata_list {
                println!("  - {} ({})", metadata.report_type, metadata.data_points);
            }
        }
        
        Ok(())
    }
    
    /// Example of using a single report with custom configuration
    pub fn generate_custom_report() -> Result<String> {
        let report = SummaryStatisticsReport::new();
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();
        
        let mut style_options = std::collections::HashMap::new();
        style_options.insert("theme".to_string(), "dark".to_string());
        style_options.insert("font_size".to_string(), "12px".to_string());
        
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
        println!("Generated {} report with {} data points", 
            metadata.report_type, metadata.data_points);
        
        Ok(content)
    }
    
    /// Example of validating reports using the unified test suite
    pub fn validate_all_reports() -> Result<()> {
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
        
        Ok(())
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
            assert!(format_dir.exists(), "Format directory should exist: {}", format);
            
            // Check that report files were created
            let entries: Vec<_> = std::fs::read_dir(&format_dir).unwrap().collect();
            assert!(!entries.is_empty(), "Should have generated report files in {}", format);
        }
    }
    
    #[test]
    fn test_custom_report_generation() {
        let content = UnifiedReportingExample::generate_custom_report().unwrap();
        assert!(!content.is_empty());
        assert!(content.contains("<!DOCTYPE html>"));
        assert!(content.contains("Summary Statistics Report"));
    }
    
    #[test]
    fn test_report_validation() {
        // This should not panic or return errors
        UnifiedReportingExample::validate_all_reports().unwrap();
    }
}