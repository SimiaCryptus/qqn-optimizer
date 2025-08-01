//! Analysis and visualization tools for optimization results.
//!
//! This module provides:
//! - Statistical analysis of benchmark results
//! - Performance comparison tools
//! - Visualization and plotting capabilities
//! - Academic report generation

#[cfg(feature = "plotting")]
pub mod plotting;
pub mod reporting;
pub mod statistics;

// Re-export commonly used types

use crate::benchmarks::evaluation::BenchmarkResults;
use crate::optimizers::OptResult;
use std::path::Path;

#[cfg(feature = "plotting")]
pub use plotting::{ExtendedOptimizationTrace, PlotConfig, PlottingEngine};
pub use reporting::{AcademicReport, CSVExporter, LaTeXExporter};
pub use statistics::{
    ConvergenceComparison, EffectSize, PerformanceProfiles, RobustnessAnalysis, SignificanceTest,
    StatisticalAnalysis,
};

/// Generate comprehensive analysis report
/// 
/// # Arguments
/// * `results` - Benchmark results to analyze
/// 
/// # Returns
/// Complete analysis report with statistical tests and comparisons
pub fn generate_full_analysis(results: &BenchmarkResults) -> OptResult<AnalysisReport> {
    let stats = StatisticalAnalysis::from_results(results)?;

    Ok(AnalysisReport {
        convergence_comparison: stats.analyze_convergence()?,
        performance_profiles: stats.compute_performance_profiles()?,
        robustness_analysis: stats.analyze_robustness()?,
        statistical_tests: stats.run_significance_tests()?,
        effect_sizes: stats.compute_effect_sizes()?,
    })
}

/// Complete analysis report structure
#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub convergence_comparison: ConvergenceComparison,
    pub performance_profiles: PerformanceProfiles,
    pub robustness_analysis: RobustnessAnalysis,
    pub statistical_tests: Vec<SignificanceTest>,
    pub effect_sizes: Vec<EffectSize>,
}

impl AnalysisReport {
    /// Export to LaTeX format for academic papers
    pub fn to_latex(&self) -> OptResult<String> {
        let exporter = LaTeXExporter::new();
        exporter.export_report(self)
    }

    /// Export to CSV for further analysis
    pub fn to_csv(&self, output_dir: &Path) -> OptResult<()> {
        let exporter = CSVExporter::new();
        exporter.export_report(self, output_dir)
    }

    /// Generate summary statistics
    pub fn summary(&self) -> String {
        format!(
            "Analysis Summary:\n\
             - Problems analyzed: {}\n\
             - Optimizers compared: {}\n\
             - Significant improvements: {}\n\
             - Average effect size: {:.3}\n\
             - Best performing optimizer: {}",
            self.performance_profiles.num_problems(),
            self.performance_profiles.num_optimizers(),
            self.statistical_tests
                .iter()
                .filter(|t| t.is_significant())
                .count(),
            self.average_effect_size(),
            self.best_optimizer()
        )
    }
    /// Calculate average effect size across all comparisons
    pub fn average_effect_size(&self) -> f64 {
        if self.effect_sizes.is_empty() {
            return 0.0;
        }
        self.effect_sizes
            .iter()
            .map(|e| e.magnitude())
            .sum::<f64>()
            / self.effect_sizes.len() as f64
    }
    /// Identify the best performing optimizer
    pub fn best_optimizer(&self) -> &str {
        self.performance_profiles
            .best_optimizer()
            .unwrap_or("Unknown")
    }
    /// Check if the analysis contains significant results
    pub fn has_significant_results(&self) -> bool {
        self.statistical_tests
            .iter()
            .any(|test| test.is_significant())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_analysis_report_creation() {
        // Test that the module structure is correct
        use super::*;
        
        // Verify that all submodules are accessible
        let _ = statistics::StatisticalAnalysis;
        let _ = reporting::AcademicReport;
        
        #[cfg(feature = "plotting")]
        {
            let _ = plotting::PlotConfig;
        }
    }

    #[test]
    fn test_analysis_report_methods() {
        use super::*;
        
        // Create a mock report for testing
        let report = AnalysisReport {
            convergence_comparison: ConvergenceComparison::default(),
            performance_profiles: PerformanceProfiles::default(),
            robustness_analysis: RobustnessAnalysis::default(),
            statistical_tests: vec![],
            effect_sizes: vec![],
        };
        
        // Test basic methods
        assert_eq!(report.average_effect_size(), 0.0);
        assert!(!report.has_significant_results());
        
        // Test summary generation
        let summary = report.summary();
        assert!(summary.contains("Analysis Summary"));
    }
}