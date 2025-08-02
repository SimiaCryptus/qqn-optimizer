//! Comprehensive unified testing infrastructure for all report types.
//!
//! This module provides unified testing patterns that can be applied to any
//! report implementation using the unified Report trait.

use crate::benchmarks::evaluation::IterationData;
use crate::benchmarks::evaluation::{
    BenchmarkConfig, BenchmarkResults, ConvergenceReason, DurationWrapper, OptimizationTrace,
    PerformanceMetrics, ProblemSpec, SingleResult,
};
use crate::experiment_runner::unified_report::{
    Report, ReportConfig, ReportFormat,
};
use crate::SphereFunction;
use anyhow::Result;
use std::sync::Arc;
use std::time::Duration;
use tempfile::TempDir;

/// Test suite that validates any Report implementation
pub struct UnifiedReportTestSuite;

impl UnifiedReportTestSuite {
    /// Run all standard tests on a report implementation
    pub fn test_report<R: Report>(report: &R) -> Result<()> {
        Self::test_basic_functionality(report)?;
        Self::test_content_generation(report)?;
        Self::test_data_validation(report)?;
        Self::test_metadata_generation(report)?;
        Self::test_file_export(report)?;
        Self::test_all_formats(report)?;
        Ok(())
    }

    /// Test basic functionality of a report
    pub fn test_basic_functionality<R: Report + ?Sized>(report: &R) -> Result<()> {
        // Test name is not empty
        assert!(!report.name().is_empty(), "Report name should not be empty");

        // Test description is not empty
        assert!(
            !report.description().is_empty(),
            "Report description should not be empty"
        );

        // Test supported formats
        let formats = report.supported_formats();
        assert!(
            !formats.is_empty(),
            "Report should support at least one format"
        );

        println!("✓ Basic functionality test passed for {}", report.name());
        Ok(())
    }

    /// Test content generation for different formats
    pub fn test_content_generation<R: Report + ?Sized>(report: &R) -> Result<()> {
        let test_data = Self::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        for format in &report.supported_formats() {
            let config = ReportConfig {
                format: format.clone(),
                ..Default::default()
            };

            let content = report.generate_content(&data_refs, &config)?;
            assert!(
                !content.is_empty(),
                "Generated content should not be empty for format {:?}",
                format
            );

            // Format-specific validations
            match format {
                ReportFormat::Html => {
                    assert!(
                        content.contains("<") && content.contains(">"),
                        "HTML content should contain HTML tags"
                    );
                }
                ReportFormat::Latex => {
                    assert!(
                        content.contains("\\"),
                        "LaTeX content should contain LaTeX commands"
                    );
                }
                ReportFormat::Markdown => {
                    assert!(
                        content.contains("#") || content.contains("|"),
                        "Markdown content should contain markdown syntax"
                    );
                }
                ReportFormat::Csv => {
                    assert!(
                        content.contains(","),
                        "CSV content should contain comma separators"
                    );
                }
            }
        }

        println!("✓ Content generation test passed for {}", report.name());
        Ok(())
    }

    /// Test data validation
    pub fn test_data_validation<R: Report + ?Sized>(report: &R) -> Result<()> {
        // Test empty data validation
        let empty_data = vec![];
        assert!(
            report.validate_data(&empty_data).is_err(),
            "Should reject empty data"
        );

        // Test valid data validation
        let test_data = Self::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();
        assert!(
            report.validate_data(&data_refs).is_ok(),
            "Should accept valid data"
        );

        // Test data with empty results
        let empty_results_data = Self::create_empty_results_data();
        let empty_refs: Vec<_> = empty_results_data
            .iter()
            .map(|(p, r)| (p, r.clone()))
            .collect();
        assert!(
            report.validate_data(&empty_refs).is_err(),
            "Should reject data with empty results"
        );

        println!("✓ Data validation test passed for {}", report.name());
        Ok(())
    }

    /// Test metadata generation
    pub fn test_metadata_generation<R: Report + ?Sized>(report: &R) -> Result<()> {
        let test_data = Self::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        let metadata = report.get_metadata(&data_refs);

        assert_eq!(metadata.report_type, report.name());
        assert!(
            metadata.problem_count > 0,
            "Should have non-zero problem count"
        );
        assert!(
            metadata.optimizer_count > 0,
            "Should have non-zero optimizer count"
        );
        assert!(metadata.data_points > 0, "Should have non-zero data points");

        println!("✓ Metadata generation test passed for {}", report.name());
        Ok(())
    }

    /// Test file export functionality
    pub fn test_file_export<R: Report + ?Sized>(report: &R) -> Result<()> {
        let test_data = Self::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        let temp_dir = TempDir::new()?;

        for format in &report.supported_formats() {
            let config = ReportConfig {
                format: format.clone(),
                ..Default::default()
            };

            let filename = format!(
                "{}.{}",
                report.name(),
                match format {
                    ReportFormat::Html => "html",
                    ReportFormat::Latex => "tex",
                    ReportFormat::Csv => "csv",
                    ReportFormat::Markdown => "md",
                }
            );

            let output_path = temp_dir.path().join(filename);
            report.export_to_file(&data_refs, &config, &output_path)?;

            assert!(output_path.exists(), "Output file should be created");
            let file_content = std::fs::read_to_string(&output_path)?;
            assert!(!file_content.is_empty(), "Output file should not be empty");
        }

        println!("✓ File export test passed for {}", report.name());
        Ok(())
    }

    /// Test all supported formats work
    pub fn test_all_formats<R: Report + ?Sized>(report: &R) -> Result<()> {
        let test_data = Self::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        for format in &[
            ReportFormat::Html,
            ReportFormat::Latex,
            ReportFormat::Markdown,
            ReportFormat::Csv,
        ] {
            if report.supported_formats().contains(format) {
                let config = ReportConfig {
                    format: format.clone(),
                    ..Default::default()
                };

                let result = report.generate_content(&data_refs, &config);
                assert!(result.is_ok(), "Format {:?} should be supported", format);
            }
        }

        println!("✓ All formats test passed for {}", report.name());
        Ok(())
    }

    /// Create test data for reports
    pub fn create_test_data() -> Vec<(ProblemSpec, BenchmarkResults)> {
        let mut test_data = Vec::new();

        // Create test data for multiple problems and optimizers
        let problems = vec![
            ("Sphere", "Convex Unimodal"),
            ("Rosenbrock", "Non-Convex Unimodal"),
            ("Rastrigin", "Highly Multimodal"),
        ];

        let optimizers = vec!["QQN-Basic", "L-BFGS", "Adam", "GD"];

        for (prob_name, family) in problems {
            let problem_spec = ProblemSpec {
                name: Some(prob_name.to_string()),
                problem: Arc::new(SphereFunction::new(2)),
                dimensions: Some(2),
                family: family.to_string(),
                seed: 0,
            };

            let mut results = Vec::new();

            for (i, optimizer) in optimizers.iter().enumerate() {
                for run_id in 0..3 {
                    // 3 runs per optimizer
                    // Create trace data with iterations for convergence analysis
                    let mut trace = OptimizationTrace::default();
                    if run_id < 2 {
                        // Only successful runs have trace data
                        let num_iterations = 100 + i * 20;
                        for iter in 0..num_iterations {
                            let progress = iter as f64 / num_iterations as f64;
                            let function_value = 1e-6 * (i + 1) as f64 * (1.0 - progress * 0.9);
                            trace.iterations.push(IterationData {
                                iteration: iter,
                                function_value,
                                gradient_norm: 1e-8 * (1.0 - progress * 0.9),
                                step_size: 0.01,
                                parameters: vec![],
                                timestamp: DurationWrapper::from(Duration::from_millis(
                                    iter as u64,
                                )),
                                total_function_evaluations: 0,
                                total_gradient_evaluations: 0,
                            });
                        }
                    }

                    let result = SingleResult {
                        problem_name: "".to_string(),
                        optimizer_name: optimizer.to_string(),
                        run_id,
                        final_value: 1e-6 * (i + 1) as f64, // Different performance
                        final_gradient_norm: 1e-8,
                        iterations: 100 + i * 20,
                        function_evaluations: 150 + i * 30,
                        gradient_evaluations: 100 + i * 20,
                        execution_time: Duration::from_millis(100 + i as u64 * 50),
                        convergence_achieved: run_id < 2, // 2 out of 3 runs succeed
                        convergence_reason: if run_id < 2 {
                            ConvergenceReason::FunctionTolerance
                        } else {
                            ConvergenceReason::MaxIterations
                        },
                        memory_usage: None,
                        best_value: 1e-6 * (i + 1) as f64,
                        trace,
                        error_message: None,
                        performance_metrics: PerformanceMetrics::default(),
                    };
                    results.push(result);
                }
            }

            let benchmark_results = BenchmarkResults {
                config: BenchmarkConfig::default(),
                timestamp: Default::default(),
                convergence_achieved: false,
                final_value: None,
                function_evaluations: 0,
                results,
                gradient_evaluations: 0,
            };

            test_data.push((problem_spec, benchmark_results));
        }

        test_data
    }

    /// Create test data with empty results (for validation testing)
    pub fn create_empty_results_data() -> Vec<(ProblemSpec, BenchmarkResults)> {
        let problem_spec = ProblemSpec {
            name: None,
            problem: Arc::new(SphereFunction::new(2)),
            dimensions: Some(2),
            family: "Test".to_string(),
            seed: 0,
        };

        let benchmark_results = BenchmarkResults {
            config: BenchmarkConfig::default(),
            timestamp: Default::default(),
            convergence_achieved: false,
            final_value: None,
            function_evaluations: 0,
            results: vec![], // Empty results
            gradient_evaluations: 0,
        };

        vec![(problem_spec, benchmark_results)]
    }
}

/// Integration tests for the unified reporting system
#[cfg(test)]
mod tests {
    use crate::experiment_runner::{FamilyVsFamilyReport, ReportCollection, SummaryStatisticsReport};
    use super::*;

    #[test]
    fn test_summary_statistics_report_with_unified_suite() {
        let report = SummaryStatisticsReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }
    #[test]
    fn test_family_vs_family_report_with_unified_suite() {
        let report = FamilyVsFamilyReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }

    #[test]
    fn test_report_collection() {
        let collection = ReportCollection::new()
            .add_report(SummaryStatisticsReport::new())
            .add_report(FamilyVsFamilyReport::new());

        let mut names = collection.report_names();
        names.sort();
        assert_eq!(names, vec!["family_vs_family", "summary_statistics"]);

        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        let temp_dir = TempDir::new().unwrap();
        let config = ReportConfig::default();

        let metadata_list = collection
            .generate_all(&data_refs, &config, temp_dir.path())
            .unwrap();
        assert_eq!(metadata_list.len(), 2);

        let report_types: Vec<_> = metadata_list
            .iter()
            .map(|m| m.report_type.as_str())
            .collect();
        assert!(report_types.contains(&"summary_statistics"));
        assert!(report_types.contains(&"family_vs_family"));
    }

    #[test]
    fn test_unified_test_suite_create_test_data() {
        let test_data = UnifiedReportTestSuite::create_test_data();
        assert!(!test_data.is_empty());

        // Verify structure
        for (problem_spec, benchmark_results) in &test_data {
            assert!(!problem_spec.get_name().is_empty());
            assert!(!benchmark_results.results.is_empty());
        }
    }

    #[test]
    fn test_unified_test_suite_validation() {
        let report = SummaryStatisticsReport::new();

        // Test individual validation methods
        UnifiedReportTestSuite::test_basic_functionality(&report).unwrap();
        UnifiedReportTestSuite::test_content_generation(&report).unwrap();
        UnifiedReportTestSuite::test_data_validation(&report).unwrap();
        UnifiedReportTestSuite::test_metadata_generation(&report).unwrap();
        UnifiedReportTestSuite::test_file_export(&report).unwrap();
        UnifiedReportTestSuite::test_all_formats(&report).unwrap();
    }

    #[test]
    fn test_report_formats_consistency() {
        let report = SummaryStatisticsReport::new();
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        // Test that all supported formats generate different but valid content
        let mut contents = std::collections::HashMap::new();

        for format in &report.supported_formats() {
            let config = ReportConfig {
                format: format.clone(),
                ..Default::default()
            };

            let content = report.generate_content(&data_refs, &config).unwrap();
            contents.insert(format.clone(), content);
        }

        // All formats should produce different content
        let values: Vec<_> = contents.values().collect();
        for i in 0..values.len() {
            for j in i + 1..values.len() {
                assert_ne!(
                    values[i], values[j],
                    "Different formats should produce different content"
                );
            }
        }
    }
}
