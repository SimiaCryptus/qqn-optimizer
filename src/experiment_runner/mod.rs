#![allow(clippy::module_inception)]

pub mod experiment_runner;
pub mod optimizer_sets;
#[cfg(feature = "plotting")]
pub mod plotting_manager;
pub mod problem_sets;
pub mod report_generator;
pub mod reports;
pub mod statistical_analysis;
pub mod test_data;
pub mod unified_report;
pub mod unified_report_tests;
pub use experiment_runner::*;
pub use report_generator::*;
pub use reports::convergence_analysis::ConvergenceAnalysisReport;
pub use reports::efficiency_matrix::EfficiencyMatrixReport;
pub use reports::family_vs_family_report::FamilyVsFamilyReport;
pub use reports::heatmap::SuccessRateHeatmapReport;
pub use reports::unified_performance_table::PerformanceTableReport;
pub use reports::unified_summary_statistics::SummaryStatisticsReport;
pub mod unified_report_example;

pub use experiment_runner::ExperimentRunner;
#[cfg(feature = "plotting")]
pub use plotting_manager::PlottingManager;
pub use report_generator::ReportGenerator;
pub use statistical_analysis::StatisticalAnalysis;
pub use unified_report::{Report, ReportCollection, ReportConfig, ReportFormat, ReportMetadata};
pub use unified_report_tests::UnifiedReportTestSuite;
