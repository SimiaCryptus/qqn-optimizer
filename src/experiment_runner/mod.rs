#![allow(clippy::module_inception)]

pub mod experiment_runner;
pub mod optimizer_sets;
pub mod plotting_manager;
pub mod problem_sets;
pub mod report_generator;
pub mod reports;
pub mod statistical_analysis;
pub mod unified_report;
pub mod unified_report_tests;
pub mod unified_report_example;

pub use experiment_runner::ExperimentRunner;
pub use plotting_manager::PlottingManager;
pub use report_generator::ReportGenerator;
pub use statistical_analysis::StatisticalAnalysis;
pub use unified_report::{Report, ReportCollection, ReportConfig, ReportFormat, ReportMetadata};
pub use unified_report_tests::UnifiedReportTestSuite;
