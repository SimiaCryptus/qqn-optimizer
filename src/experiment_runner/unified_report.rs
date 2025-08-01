//! Unified trait for all report types.
//!
//! This module provides a common interface for all report generation,
//! enabling consistent testing and usage patterns across different report types.

use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fmt;

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
    /// Maximum number of iterations to include in trace plots
    pub max_trace_points: Option<usize>,
    /// Include raw data tables
    pub include_raw_data: bool,
}

/// Supported report output formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Hash, Eq)]
pub enum ReportFormat {
    Html,
    Latex,
    Csv,
    Markdown,
    Json,
}
impl fmt::Display for ReportFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReportFormat::Html => write!(f, "html"),
            ReportFormat::Latex => write!(f, "tex"),
            ReportFormat::Csv => write!(f, "csv"),
            ReportFormat::Markdown => write!(f, "md"),
            ReportFormat::Json => write!(f, "json"),
        }
    }
}
impl ReportFormat {
    /// Get the file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            ReportFormat::Html => "html",
            ReportFormat::Latex => "tex",
            ReportFormat::Csv => "csv",
            ReportFormat::Markdown => "md",
            ReportFormat::Json => "json",
        }
    }
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            format: ReportFormat::Html,
            include_detailed_stats: true,
            include_plots: true,
            style_options: HashMap::new(),
            max_trace_points: Some(1000),
            include_raw_data: false,
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
    /// Report format used
    pub format: ReportFormat,
    /// Configuration used
    pub config: ReportConfig,
}

/// Unified trait for all report types
pub trait Report: Send + Sync {
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
        // Validate format is supported
        if !self.supported_formats().contains(&config.format) {
            anyhow::bail!(
                "Report '{}' does not support format '{}'",
                self.name(),
                config.format
            );
        }

        let content = self.generate_content(data, config)?;
        // Ensure parent directory exists
        if let Some(parent) = output_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
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
    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)], config: &ReportConfig) -> ReportMetadata {
        let problem_count = data.len();
        let optimizer_count = data
            .iter()
            .flat_map(|(_, results)| &results.results)
            .map(|r| &r.optimizer_name)
            .collect::<std::collections::HashSet<_>>()
            .len();
        let data_points = data.iter().map(|(_, results)| results.results.len()).sum();

        ReportMetadata {
            report_type: self.name().to_string(),
            generated_at: chrono::Utc::now(),
            problem_count,
            optimizer_count,
            data_points,
            format: config.format.clone(),
            config: config.clone(),
        }
    }

    /// Get supported output formats for this report type
    fn supported_formats(&self) -> Vec<ReportFormat> {
        vec![
            ReportFormat::Html,
            ReportFormat::Latex,
            ReportFormat::Markdown,
            ReportFormat::Json,
        ]
    }
    /// Check if a specific format is supported
    fn supports_format(&self, format: &ReportFormat) -> bool {
        self.supported_formats().contains(format)
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

            let filename = format!("{}.{}", report.name(), config.format.extension());

            let output_path = output_dir.join(filename);
            report.export_to_file(data, config, &output_path)?;

            metadata.push(report.get_metadata(data, config));
        }

        Ok(metadata)
    }
    /// Generate reports in parallel
    pub fn generate_all_parallel(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
        output_dir: &Path,
    ) -> Result<Vec<ReportMetadata>> {
        use rayon::prelude::*;
        std::fs::create_dir_all(output_dir)?;
        let results: Result<Vec<_>> = self.reports
            .par_iter()
            .map(|report| {
                report.validate_data(data)?;
                let filename = format!("{}.{}", report.name(), config.format.extension());
                let output_path = output_dir.join(filename);
                report.export_to_file(data, config, &output_path)?;
                Ok(report.get_metadata(data, config))
            })
            .collect();
        results
    }


    /// Get all report names in the collection
    pub fn report_names(&self) -> Vec<&'static str> {
        self.reports.iter().map(|r| r.name()).collect()
    }
    /// Get the number of reports in the collection
    pub fn len(&self) -> usize {
        self.reports.len()
    }
    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.reports.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::evaluation::{
        BenchmarkConfig, BenchmarkResults, ConvergenceReason, OptimizationTrace,
        PerformanceMetrics, ProblemSpec, SingleResult,
    };
    use crate::SphereFunction;
    use std::sync::Arc;
    use std::time::Duration;

    // Mock report implementation for testing
    struct MockReport {
        name: &'static str,
    }
    unsafe impl Send for MockReport {}
    unsafe impl Sync for MockReport {}


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
                ReportFormat::Html => Ok(format!(
                    "<h1>{}</h1><p>Problems: {}</p>",
                    self.name(),
                    data.len()
                )),
                ReportFormat::Markdown => {
                    Ok(format!("# {}\n\nProblems: {}\n", self.name(), data.len()))
                }
                ReportFormat::Latex => Ok(format!(
                    "\\section{{{}}}\nProblems: {}\n",
                    self.name(),
                    data.len()
                )),
                ReportFormat::Csv => Ok(format!(
                    "report_type,problem_count\n{},{}\n",
                    self.name(),
                    data.len()
                )),
                ReportFormat::Json => Ok(format!(
                    r#"{{"report_type":"{}","problem_count":{}}}"#,
                    self.name(),
                    data.len()
                )),
            }
        }
    }

    fn create_test_data() -> Vec<(ProblemSpec, BenchmarkResults)> {
        // Create minimal test data
        let problem_spec = ProblemSpec {
            name: None,
            problem: Arc::new(SphereFunction::new(2)),
            dimensions: Some(2),
            family: "Test".to_string(),
            seed: 0,
        };

        let result = SingleResult {
            problem_name: "".to_string(),
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
            memory_usage: None,
            best_value: 1e-6,
            trace: OptimizationTrace::default(),
            error_message: None,
            performance_metrics: PerformanceMetrics {
                iterations_per_second: 0.0,
                function_evaluations_per_second: 0.0,
                gradient_evaluations_per_second: 0.0,
                convergence_rate: 0.0,
            },
        };

        let results = BenchmarkResults {
            config: BenchmarkConfig::default(),
            timestamp: Default::default(),
            convergence_achieved: false,
            final_value: None,
            function_evaluations: 0,
            results: vec![result],
            gradient_evaluations: 0,
        };

        vec![(problem_spec, results)]
    }

    #[test]
    fn test_report_trait_basic_functionality() {
        let report = MockReport {
            name: "test_report",
        };
        assert_eq!(report.name(), "test_report");
        assert_eq!(report.description(), "Mock report for testing");

        let formats = report.supported_formats();
        assert!(formats.contains(&ReportFormat::Html));
        assert!(formats.contains(&ReportFormat::Latex));
        assert!(formats.contains(&ReportFormat::Markdown));
        assert!(formats.contains(&ReportFormat::Json));
    }

    #[test]
    fn test_report_content_generation() {
        let report = MockReport {
            name: "test_report",
        };
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
        let report = MockReport {
            name: "test_report",
        };

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
        let report = MockReport {
            name: "test_report",
        };
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();
        let config = ReportConfig::default();

        let metadata = report.get_metadata(&data_refs, &config);
        assert_eq!(metadata.report_type, "test_report");
        assert_eq!(metadata.problem_count, 1);
        assert_eq!(metadata.optimizer_count, 1);
        assert_eq!(metadata.data_points, 1);
        assert_eq!(metadata.format, ReportFormat::Html);
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
        assert_eq!(collection.len(), 2);
        assert!(!collection.is_empty());
    }

    #[test]
    fn test_different_output_formats() {
        let report = MockReport {
            name: "test_report",
        };
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();

        let html_config = ReportConfig {
            format: ReportFormat::Html,
            ..Default::default()
        };
        let html_content = report.generate_content(&data_refs, &html_config).unwrap();
        assert!(html_content.contains("<h1>"));

        let md_config = ReportConfig {
            format: ReportFormat::Markdown,
            ..Default::default()
        };
        let md_content = report.generate_content(&data_refs, &md_config).unwrap();
        assert!(md_content.contains("# test_report"));

        let latex_config = ReportConfig {
            format: ReportFormat::Latex,
            ..Default::default()
        };
        let latex_content = report.generate_content(&data_refs, &latex_config).unwrap();
        assert!(latex_content.contains("\\section{"));

        let csv_config = ReportConfig {
            format: ReportFormat::Csv,
            ..Default::default()
        };
        let csv_content = report.generate_content(&data_refs, &csv_config).unwrap();
        assert!(csv_content.contains("report_type,problem_count"));
        let json_config = ReportConfig {
            format: ReportFormat::Json,
            ..Default::default()
        };
        let json_content = report.generate_content(&data_refs, &json_config).unwrap();
        assert!(json_content.contains(r#""report_type":"test_report""#));
    }
    #[test]
    fn test_format_extensions() {
        assert_eq!(ReportFormat::Html.extension(), "html");
        assert_eq!(ReportFormat::Latex.extension(), "tex");
        assert_eq!(ReportFormat::Csv.extension(), "csv");
        assert_eq!(ReportFormat::Markdown.extension(), "md");
        assert_eq!(ReportFormat::Json.extension(), "json");
    }
    #[test]
    fn test_format_display() {
        assert_eq!(format!("{}", ReportFormat::Html), "html");
        assert_eq!(format!("{}", ReportFormat::Latex), "tex");
        assert_eq!(format!("{}", ReportFormat::Csv), "csv");
        assert_eq!(format!("{}", ReportFormat::Markdown), "md");
        assert_eq!(format!("{}", ReportFormat::Json), "json");
    }
    #[test]
    fn test_supports_format() {
        let report = MockReport {
            name: "test_report",
        };
        assert!(report.supports_format(&ReportFormat::Html));
        assert!(report.supports_format(&ReportFormat::Latex));
        assert!(report.supports_format(&ReportFormat::Csv));
        assert!(report.supports_format(&ReportFormat::Markdown));
        assert!(report.supports_format(&ReportFormat::Json));
    }
}