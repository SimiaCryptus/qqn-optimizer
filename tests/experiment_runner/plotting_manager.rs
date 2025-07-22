use log::{info, warn};
use qqn_optimizer::benchmarks::evaluation::BenchmarkResults;
use qqn_optimizer::{ExtendedOptimizationTrace, OptimizationProblem, PlotConfig, PlottingEngine};
use std::fs;
use std::sync::Arc;

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
        let plotting_engine = PlottingEngine::new(output_dir.clone())
            .with_config(config);
        Self {
            output_dir,
            plotting_engine,
            enable_enhanced_plots: true,
        }
    }
    pub fn with_enhanced_plots(mut self, enable: bool) -> Self {
        self.enable_enhanced_plots = enable;
        self
    }


    pub async fn generate_all_plots(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        info!("Generating plots for {} benchmark results", all_results.len());


        // Generate convergence plots for each problem
        for (problem, results) in all_results {
            let problem_name = problem.name();
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
                        .map(|i| i.total_function_evaluations.max(i.total_gradient_evaluations))
                        .collect(),
                })
                .collect();

            if !traces.is_empty() {
                info!("Generating plots for {} with {} optimizers", problem_name, traces.len());

                let filename = format!("convergence_{}", problem_name.replace(" ", "_"));
                self.generate_plot_with_fallback(
                    || self.plotting_engine.convergence_plot(&traces, &filename),
                    &format!("convergence plot for {}", problem_name),
                ).await;

                if self.enable_enhanced_plots {
                    self.generate_plot_with_fallback(
                        || self.plotting_engine.log_convergence_plot(&traces, &format!("{}_log", filename)),
                        &format!("log convergence plot for {}", problem_name),
                    ).await;
                }
            }
            tokio::task::yield_now().await;
        }

        // Generate performance comparison plots
        if let Some((_, first_results)) = all_results.first() {
            if self.enable_enhanced_plots {
                info!("Generating performance comparison plots");

                self.generate_plot_with_fallback(
                    || self.plotting_engine.performance_comparison(first_results, "performance_comparison"),
                    "performance comparison plot",
                ).await;

                self.generate_plot_with_fallback(
                    || self.plotting_engine.performance_boxplot(first_results, "performance_distribution"),
                    "performance boxplot",
                ).await;
            }
        }

        tokio::task::yield_now().await;
        info!("Plot generation completed");
        Ok(())
    }

    async fn generate_plot_with_fallback<F>(&self, plot_fn: F, plot_description: &str)
    where
        F: FnOnce() -> anyhow::Result<()>,
    {
        // Use a more robust approach to handle plotting errors
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(plot_fn));

        match result {
            Ok(Ok(_)) => info!("Generated {}", plot_description),
            Ok(Err(e)) => {
                warn!("Failed to generate {}: {}", plot_description, e);
            }
            Err(_) => {
                warn!(
                    "Skipping {} due to panic in plotting library",
                    plot_description
                );
            }
        }
    }
}