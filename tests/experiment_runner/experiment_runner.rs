use super::{standard_optimizers, PlottingManager, ReportGenerator, StatisticalAnalysis};
use log::{info, warn};
use qqn_optimizer::analysis::plotting::{ExtendedOptimizationTrace, PlottingEngine};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults, BenchmarkRunner, DurationWrapper, SingleResult};
use qqn_optimizer::core::gd::{GDConfig, GDOptimizer};
use qqn_optimizer::{AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, OptimizationProblem, Optimizer, QQNConfig, QQNOptimizer};
use rand::{Rng, SeedableRng};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Core experiment runner focused on benchmark execution
pub struct ExperimentRunner {
    output_dir: String,
    config: BenchmarkConfig,
    report_generator: ReportGenerator,
    plotting_manager: PlottingManager,
}

impl ExperimentRunner {
    pub fn new(output_dir: String, config: BenchmarkConfig) -> Self {
        Self {
            output_dir: output_dir.clone(),
            config: config.clone(),
            report_generator: ReportGenerator::new(output_dir.clone(), config.clone()),
            plotting_manager: PlottingManager::new(output_dir),
        }
    }
    
    /// Run comprehensive comparative benchmarks
    pub async fn run_comparative_benchmarks(&self, problems: Vec<Arc<dyn OptimizationProblem>>, optimizers: Vec<(String, Arc<dyn Optimizer>)>) -> anyhow::Result<()> {
        info!("Starting comprehensive comparative benchmarks");

        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        
        // Validate problems
        self.validate_problems(&problems).await?;

        // Run benchmarks for each problem
        let mut all_results = Vec::new();

        for problem in &problems {
            info!("Running benchmarks for problem: {}", problem.name());
            let results = self
                .run_problem_benchmarks(problem.as_ref(), &optimizers)
                .await?;
            all_results.push((problem.name().to_string(), results));
            tokio::task::yield_now().await;
        }

        // Generate comprehensive analysis and reports
        self.plotting_manager.generate_all_plots(&all_results).await?;
        self.report_generator.generate_html_report(&all_results, &problems).await?;

        info!(
            "Benchmark experiments completed. Results saved to: {}",
            self.output_dir
        );
        tokio::task::yield_now().await;

        Ok(())
    }

    pub async fn validate_problems(&self, problems: &[Arc<dyn OptimizationProblem>]) -> anyhow::Result<()> {
        for problem in problems {
            let initial_params = problem.initial_point();
            let mut rng = rand::rngs::StdRng::try_from_os_rng()
                .expect("Failed to create random number generator");
            let initial_params: Vec<f64> = initial_params
                .iter()
                .map(|&x| x + rng.random_range(-1.0..1.0))
                .collect();
            let initial_value = problem.evaluate_f64(&initial_params)?;
            info!(
                "Problem {}: initial_value = {:.6e}, dimensions = {}",
                problem.name(),
                initial_value,
                initial_params.len()
            );
            if initial_value < 1e-10 {
                warn!(
                    "Problem {} may be starting too close to optimum (initial_value = {:.6e})",
                    problem.name(),
                    initial_value
                );
            }
        }
        Ok(())
    }

    async fn run_problem_benchmarks(
        &self,
        problem: &dyn OptimizationProblem,
        optimizers: &[(String, Arc<dyn Optimizer>)],
    ) -> anyhow::Result<BenchmarkResults> {
        let runner = BenchmarkRunner::new(self.config.clone());
        let mut results = BenchmarkResults::new(self.config.clone());

        for (opt_name, ref optimizer) in optimizers.iter() {
            for run_id in 0..self.config.num_runs {
                let mut result = runner
                    .run_single_benchmark(problem, &mut optimizer.clone_box(), run_id, &opt_name)
                    .await?;

                if let Some(optimal_value) = problem.optimal_value() {
                    let success_threshold = optimal_value;
                    result.convergence_achieved &= result.final_value < success_threshold;
                } else {
                    result.convergence_achieved = false;
                }
                results.add_result(result);
            }
        }

        Ok(results)
    }
}