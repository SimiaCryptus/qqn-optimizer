use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::{ExtendedOptimizationTrace, PlotConfig, PlottingEngine};
use log::{info, warn};
use std::fs;

/// Manages plot generation with error handling
pub struct PlottingManager {
    output_dir: String,
    plotting_engine: PlottingEngine,
    enable_enhanced_plots: bool,
}

impl PlottingManager {
    pub fn new(output_dir: String) -> Self {
        let config = PlotConfig {
            width: 1200,
            height: 800,
            enable_legends: true,
            enable_grid: true,
            ..Default::default()
        };
        let plotting_engine = PlottingEngine::new(output_dir.clone()).with_config(config);
        Self {
            output_dir,
            plotting_engine,
            enable_enhanced_plots: true,
        }
    }
    /// Generate a summary report of all plots created
    pub async fn generate_plot_summary(&self) -> anyhow::Result<()> {
        let summary_path = self.output_dir.join("plot_summary.md");
        let mut summary = String::from("# Plot Generation Summary\n\n");
        // List convergence plots
        let convergence_dir = self.output_dir.join("convergence");
        if convergence_dir.exists() {
            summary.push_str("## Convergence Plots\n\n");
            for entry in fs::read_dir(&convergence_dir)? {
                let entry = entry?;
                if entry.path().extension().and_then(|s| s.to_str()) == Some("png") {
                    let filename = entry.file_name().to_string_lossy();
                    summary.push_str(&format!("- {}\n", filename));
                }
            }
            summary.push_str("\n");
        }
        // Add performance comparison section
        summary.push_str("## Performance Comparison Plots\n\n");
        summary.push_str("- performance_comparison.png\n");
        summary.push_str("- performance_distribution.png\n");
        fs::write(summary_path, summary)?;
        Ok(())
    }


    pub async fn generate_all_plots(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        info!(
            "Generating plots for {} benchmark results",
            all_results.len()
        );

        // Generate convergence plots for each problem
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            let traces: Vec<ExtendedOptimizationTrace> = results
                .results
                .iter()
                .filter(|r| !r.trace.iterations.is_empty())
                .map(|r| ExtendedOptimizationTrace {
                    optimizer_name: r.optimizer_name.clone(),
                    objective_values: r
                        .trace
                        .iterations
                        .iter()
                        .map(|i| i.function_value)
                        .collect(),
                    evaluation_counts: r
                        .trace
                        .iterations
                        .iter()
                        .map(|i| i.total_evaluations())
                        .collect(),
                })
                .collect();

            if !traces.is_empty() {
                info!(
                    "Generating plots for {} with {} optimizers",
                    problem_name,
                    traces.len()
                );

                let filename = format!("convergence/{}", problem_name.replace(" ", "_"));
                self.generate_plot_with_fallback(
                    || self.plotting_engine.convergence_plot(&traces, &filename),
                    &format!("convergence plot for {problem_name}"),
                )
                .await;

                if self.enable_enhanced_plots {
                    self.generate_plot_with_fallback(
                        || {
                            self.plotting_engine
                                .log_convergence_plot(&traces, &format!("{filename}_log"))
                        },
                        &format!("log convergence plot for {problem_name}"),
                    )
                    .await;
                }
            }
            tokio::task::yield_now().await;
        }

        // Generate performance comparison plots
        if let Some((_, first_results)) = all_results.first() {
            if self.enable_enhanced_plots {
                info!("Generating performance comparison plots");

                self.generate_plot_with_fallback(
                    || {
                        self.plotting_engine
                            .performance_comparison(first_results, "performance_comparison")
                    },
                    "performance comparison plot",
                )
                .await;

                self.generate_plot_with_fallback(
                    || {
                        self.plotting_engine
                            .performance_boxplot(first_results, "performance_distribution")
                    },
                    "performance boxplot",
                )
                .await;
            }
        }

        tokio::task::yield_now().await;
        info!("Plot generation completed");
        // Generate summary report
        self.generate_plot_summary().await?;
        Ok(())
    }

    async fn generate_plot_with_fallback<F>(&self, plot_fn: F, plot_description: &str)
    where
        F: FnOnce() -> anyhow::Result<()>,
    {
        // Use a more robust approach to handle plotting errors
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(plot_fn));

        match result {
            Ok(Ok(_)) => info!("Generated {plot_description}"),
            Ok(Err(e)) => {
                warn!("Failed to generate {plot_description}: {e}");
            }
            Err(_) => {
                warn!("Skipping {plot_description} due to panic in plotting library");
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("Test Problem"), "test_problem");
        assert_eq!(sanitize_filename("Test/Problem:1"), "test_problem_1");
        assert_eq!(sanitize_filename("Test*Problem?"), "test_problem");
        assert_eq!(sanitize_filename("Test<>Problem|"), "test_problem");
    }
    #[tokio::test]
    async fn test_plotting_manager_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = PlottingManager::new(temp_dir.path().to_string_lossy().to_string());
        assert!(manager.enable_enhanced_plots);
        assert_eq!(manager.max_concurrent_plots, 4);
    }
    #[tokio::test]
    async fn test_plotting_manager_configuration() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = PlottingManager::new(temp_dir.path().to_string_lossy().to_string())
            .with_enhanced_plots(false)
            .with_max_concurrent_plots(8);
        assert!(!manager.enable_enhanced_plots);
        assert_eq!(manager.max_concurrent_plots, 8);
    }
    #[tokio::test]
    async fn test_empty_results_handling() {
        let temp_dir = tempfile::tempdir().unwrap();
        let manager = PlottingManager::new(temp_dir.path().to_string_lossy().to_string());
        assert!(manager.generate_all_plots(&[]).await.is_ok());
    }
}