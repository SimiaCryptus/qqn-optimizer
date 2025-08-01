//! Experiment runner module for optimization benchmarking and analysis.
//! 
//! This module provides tools for running optimization experiments, generating reports,
//! and performing statistical analysis on the results.
// Core modules

pub mod experiment_runner;
pub mod report_generator;
pub mod statistical_analysis;
pub mod unified_report;

// Data and configuration modules
pub mod optimizer_sets;
pub mod problem_sets;
pub mod test_data;

// Report modules
pub mod reports;

// Feature-gated modules
#[cfg(feature = "plotting")]
pub mod plotting_manager;

// Test and example modules
#[cfg(test)]
pub mod unified_report_tests;
#[cfg(any(test, doc))]
pub mod unified_report_example;

// Private modules
mod optimizer_problems;

// Core re-exports
pub use experiment_runner::{ExperimentRunner, ExperimentConfig};
pub use report_generator::{ReportGenerator, ReportGeneratorConfig};
pub use statistical_analysis::{StatisticalAnalysis, StatisticalMetrics};
pub use unified_report::{
    Report, ReportCollection, ReportConfig, ReportFormat, ReportMetadata,
    UnifiedReport, UnifiedReportBuilder
};
// Report re-exports
pub use reports::{
    convergence_analysis::ConvergenceAnalysisReport,
    efficiency_matrix::EfficiencyMatrixReport,
    family_vs_family_report::FamilyVsFamilyReport,
    heatmap::SuccessRateHeatmapReport,
    unified_performance_table::PerformanceTableReport,
    unified_summary_statistics::SummaryStatisticsReport,
};

// Feature-gated re-exports
#[cfg(feature = "plotting")]
pub use plotting_manager::PlottingManager;
// Test utilities
#[cfg(test)]
pub use unified_report_tests::UnifiedReportTestSuite;