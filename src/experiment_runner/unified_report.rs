//! Unified trait for all report types.
//!
//! This module provides a common interface for all report generation,
//! enabling consistent testing and usage patterns across different report types.

use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Configuration for report generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportConfig {
    /// Output format (html, latex, csv, markdown)
    pub format: ReportFormat,
    /// Include detailed statistics
    pub include_detailed_stats: bool,
    /// Include plots and visualizations
    pub include_plots: bool,
    /// Custom styling options
    pub style_options: HashMap<String, String>,
}

/// Supported report output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ReportFormat {
    Html,
    Latex,
    Csv,
    Markdown,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Html,
            include_detailed_stats: true,
            include_plots: true,
            style_options: HashMap::new(),
        }
    }
}

/// Metadata about a generated report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportMetadata {
    /// Report type identifier
    pub report_type: String,
    /// Generation timestamp
    pub generated_at: chrono::DateTime<chrono::Utc>,
    /// Number of problems analyzed
    pub problem_count: usize,
    /// Number of optimizers compared
    pub optimizer_count: usize,
    /// Total data points processed
    pub data_points: usize,
}

/// Unified trait for all report types
pub trait Report {
    /// Get the name/identifier for this report type
    fn name(&self) -> &'static str;
    
    /// Get a description of what this report provides
    fn description(&self) -> &'static str;
    
    /// Generate the report content for the specified format
    fn generate_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> Result<String>;
    
    /// Export the report to a file
    fn export_to_file(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
        output_path: &Path,
    ) -> Result<()> {
        let content = self.generate_content(data, config)?;
        std::fs::write(output_path, content)?;
        Ok(())
    }
    
    /// Validate that the input data is suitable for this report
    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> Result<()> {
        if data.is_empty() {
            anyhow::bail!("Cannot generate {} report: no data provided", self.name());
        }
        
        for (problem, results) in data {
            if results.results.is_empty() {
                anyhow::bail!(
                    "Cannot generate {} report: no results for problem '{}'",
                    self.name(),
                    problem.get_name()
                );
            }
        }
        
        Ok(())
    }
    
    /// Get metadata about the generated report
    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let problem_count = data.len();
        let optimizer_count = data
            .iter()
            .flat_map(|(_, results)| &results.results)
            .map(|r| &r.optimizer_name)
            .collect::<std::collections::HashSet<_>>()
            .len();
        let data_points = data
            .iter()
            .map(|(_, results)| results.results.len())
            .sum();
            
        ReportMetadata {
            report_type: self.name().to_string(),
            generated_at: chrono::Utc::now(),
            problem_count,
            optimizer_count,
            data_points,
        }
    }
    
    /// Get supported output formats for this report type
    fn supported_formats(&self) -> Vec<ReportFormat> {
        vec![ReportFormat::Html, ReportFormat::Latex, ReportFormat::Markdown]
    }
}

/// Collection of reports for batch processing
#[derive(Default)]
pub struct ReportCollection {
    reports: Vec<Box<dyn Report>>,
}

impl ReportCollection {
    /// Create a new empty report collection
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Add a report to the collection
    pub fn add_report<R: Report + 'static>(mut self, report: R) -> Self {
        self.reports.push(Box::new(report));
        self
    }
    
    /// Generate all reports in the collection
    pub fn generate_all(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
        output_dir: &Path,
    ) -> Result<Vec<ReportMetadata>> {
        let mut metadata = Vec::new();
        
        std::fs::create_dir_all(output_dir)?;
        
        for report in &self.reports {
            report.validate_data(data)?;
            
            let filename = format!("{}.{}", 
                report.name(), 
                match config.format {
                    ReportFormat::Html => "html",
                    ReportFormat::Latex => "tex", 
                    ReportFormat::Csv => "csv",
                    ReportFormat::Markdown => "md",
                }
            );
            
            let output_path = output_dir.join(filename);
            report.export_to_file(data, config, &output_path)?;
            
            metadata.push(report.get_metadata(data));
        }
        
        Ok(metadata)
    }
    
    /// Get all report names in the collection
    pub fn report_names(&self) -> Vec<&'static str> {
        self.reports.iter().map(|r| r.name()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::evaluation::{BenchmarkConfig, ConvergenceReason, SingleResult};
    use crate::optimizers::OptimizationTrace;
    use std::time::Duration;

    // Mock report implementation for testing
    struct MockReport {
        name: &'static str,
    }
    
    impl Report for MockReport {
        fn name(&self) -> &'static str {
            self.name
        }
        
        fn description(&self) -> &'static str {
            "Mock report for testing"
        }
        
        fn generate_content(
            &self,
            data: &[(&ProblemSpec, BenchmarkResults)],
            config: &ReportConfig,
        ) -> Result<String> {
            self.validate_data(data)?;
            
            match config.format {
                ReportFormat::Html => Ok(format!("<h1>{}</h1><p>Problems: {}</p>", 
                    self.name(), data.len())),
                ReportFormat::Markdown => Ok(format!("# {}\n\nProblems: {}\n", 
                    self.name(), data.len())),
                ReportFormat::Latex => Ok(format!("\\section{{{}}}\nProblems: {}\n", 
                    self.name(), data.len())),
                ReportFormat::Csv => Ok(format!("report_type,problem_count\n{},{}\n", 
                    self.name(), data.len())),
            }
        }
    }
    
    fn create_test_data() -> Vec<(ProblemSpec, BenchmarkResults)> {
        // Create minimal test data
        let problem_spec = ProblemSpec {
            problem: Box::new(crate::benchmarks::analytic_functions::Sphere::new(2)),
            dimensions: Some(2),
            family: "Test".to_string(),
        };
        
        let result = SingleResult {
            optimizer_name: "TestOptimizer".to_string(),
            run_id: 0,
            final_value: 1e-6,
            final_gradient_norm: 1e-8,
            iterations: 100,
            function_evaluations: 150,
            gradient_evaluations: 100,
            execution_time: Duration::from_millis(100),
            convergence_achieved: true,
            convergence_reason: ConvergenceReason::FunctionTolerance,
            best_value: 1e-6,
            trace: OptimizationTrace::default(),
            error_message: None,
        };
        
        let results = BenchmarkResults {
            config: BenchmarkConfig::default(),
            results: vec![result],
        };
        
        vec![(problem_spec, results)]
    }
    
    #[test]
    fn test_report_trait_basic_functionality() {
        let report = MockReport { name: "test_report" };
        assert_eq!(report.name(), "test_report");
        assert_eq!(report.description(), "Mock report for testing");
        
        let formats = report.supported_formats();
        assert!(formats.contains(&ReportFormat::Html));
        assert!(formats.contains(&ReportFormat::Latex));
        assert!(formats.contains(&ReportFormat::Markdown));
    }
    
    #[test]
    fn test_report_content_generation() {
        let report = MockReport { name: "test_report" };
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();
        
        let config = ReportConfig {
            format: ReportFormat::Html,
            ..Default::default()
        };
        
        let content = report.generate_content(&data_refs, &config).unwrap();
        assert!(content.contains("<h1>test_report</h1>"));
        assert!(content.contains("Problems: 1"));
    }
    
    #[test]
    fn test_report_validation() {
        let report = MockReport { name: "test_report" };
        
        // Test empty data validation
        let empty_data = vec![];
        assert!(report.validate_data(&empty_data).is_err());
        
        // Test valid data validation
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();
        assert!(report.validate_data(&data_refs).is_ok());
    }
    
    #[test]
    fn test_report_metadata() {
        let report = MockReport { name: "test_report" };
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();
        
        let metadata = report.get_metadata(&data_refs);
        assert_eq!(metadata.report_type, "test_report");
        assert_eq!(metadata.problem_count, 1);
        assert_eq!(metadata.optimizer_count, 1);
        assert_eq!(metadata.data_points, 1);
    }
    
    #[test]
    fn test_report_collection() {
        let mut collection = ReportCollection::new();
        collection = collection
            .add_report(MockReport { name: "report1" })
            .add_report(MockReport { name: "report2" });
        
        let names = collection.report_names();
        assert_eq!(names.len(), 2);
        assert!(names.contains(&"report1"));
        assert!(names.contains(&"report2"));
    }
    
    #[test]
    fn test_different_output_formats() {
        let report = MockReport { name: "test_report" };
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();
        
        let html_config = ReportConfig { format: ReportFormat::Html, ..Default::default() };
        let html_content = report.generate_content(&data_refs, &html_config).unwrap();
        assert!(html_content.contains("<h1>"));
        
        let md_config = ReportConfig { format: ReportFormat::Markdown, ..Default::default() };
        let md_content = report.generate_content(&data_refs, &md_config).unwrap();
        assert!(md_content.contains("# test_report"));
        
        let latex_config = ReportConfig { format: ReportFormat::Latex, ..Default::default() };
        let latex_content = report.generate_content(&data_refs, &latex_config).unwrap();
        assert!(latex_content.contains("\\section{"));
        
        let csv_config = ReportConfig { format: ReportFormat::Csv, ..Default::default() };
        let csv_content = report.generate_content(&data_refs, &csv_config).unwrap();
        assert!(csv_content.contains("report_type,problem_count"));
    }
}