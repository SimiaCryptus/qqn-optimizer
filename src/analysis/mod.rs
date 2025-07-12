//! Analysis and visualization tools for optimization results.
//!
//! This module provides:
//! - Statistical analysis of benchmark results
//! - Performance comparison tools
//! - Visualization and plotting capabilities
//! - Academic report generation

pub mod statistics;
pub mod plotting;
pub use statistics::*;
pub use plotting::*;
pub mod plotting;
pub mod reporting;

// Re-export commonly used types
pub use statistics::{
    StatisticalAnalysis, ConvergenceComparison, PerformanceProfiles,
    RobustnessAnalysis, SignificanceTest, EffectSize,
};
pub use plotting::{
    PlottingEngine, PlotConfig, ConvergencePlot, PerformancePlot,
    MagnitudeRatioPlot, StatisticalPlot,
};
pub use reporting::{
    ReportGenerator, AcademicReport, LaTeXExporter, CSVExporter,
};

use crate::core::OptResult;
use crate::benchmarks::BenchmarkResults;

/// Generate comprehensive analysis report
pub fn generate_full_analysis(results: &BenchmarkResults) -> OptResult<AnalysisReport> {
    let stats = StatisticalAnalysis::new(results);
    let convergence = stats.convergence_comparison()?;
    let performance = stats.performance_profiles()?;
    let robustness = stats.robustness_analysis()?;

    Ok(AnalysisReport {
        convergence_comparison: convergence,
        performance_profiles: performance,
        robustness_analysis: robustness,
        statistical_tests: stats.significance_tests()?,
        effect_sizes: stats.effect_sizes()?,
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
    pub fn to_csv(&self, output_dir: &std::path::Path) -> OptResult<()> {
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
             - Average effect size: {:.3}",
            self.convergence_comparison.num_problems(),
            self.convergence_comparison.num_optimizers(),
            self.statistical_tests.iter().filter(|t| t.is_significant()).count(),
            self.effect_sizes.iter().map(|e| e.magnitude()).sum::<f64>() / self.effect_sizes.len() as f64
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_report_creation() {
        // This would require mock data in a real implementation
        // For now, just test that the types compile
        assert!(true);
    }
}