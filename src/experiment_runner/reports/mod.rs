//! Report generation modules for experiment analysis
//! 
//! This module provides various report types for analyzing optimization experiments,
//! including performance comparisons, convergence analysis, and statistical summaries.

// Comparison and analysis reports
pub mod comparison_matrix;
pub mod convergence_analysis;
pub mod efficiency_matrix;
// Internal use only - contains implementation details
pub(crate) mod failure_analysis;

// Family-based reports
pub mod family_vs_family;
pub mod family_vs_family_report;
// Visualization reports
pub mod heatmap;
pub mod parameter_evolution;

// Performance and summary reports
pub mod performance_analysis;
pub mod run_by_run;
pub mod summary_statistics;
pub mod unified_performance_table;
pub mod unified_summary_statistics;