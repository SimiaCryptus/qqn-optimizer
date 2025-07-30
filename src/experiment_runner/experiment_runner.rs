use super::{PlottingManager, ReportGenerator};
use crate::benchmarks::evaluation::{
    enable_no_threshold_mode, BenchmarkConfig, BenchmarkResults, BenchmarkRunner, DurationWrapper,
    ProblemSpec, SingleResult,
};
use crate::Optimizer;
use log::{error, info, warn};
use rand::{Rng, SeedableRng};
use std::fs;
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

    /// Run benchmarks with problem-specific optimizer sets
    pub async fn run_championship_benchmarks(
        &self,
        problem_optimizer_map: std::collections::HashMap<String, Vec<(String, Arc<dyn Optimizer>)>>,
    ) -> anyhow::Result<()> {
        info!("Starting championship benchmarks with problem-specific optimizers");
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        // Run benchmarks for each problem with its specific optimizers
        for problem_name in problem_optimizer_map.keys() {
            // Find the problem by name (we'll need to pass problems separately or store them)
            info!("Running championship benchmarks for problem: {problem_name}");
            // This will be handled by the calling function
        }
        Ok(())
    }

    /// Run comprehensive comparative benchmarks
    pub async fn run_comparative_benchmarks(
        &self,
        problems: Vec<ProblemSpec>,
        optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    ) -> anyhow::Result<()> {
        info!("Starting comprehensive comparative benchmarks");

        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;

        // Validate problems
        self.validate_problems(&problems).await?;

        // Run benchmarks for each problem
        let mut all_results: Vec<(&ProblemSpec, BenchmarkResults)> = Vec::new();

        for problem in &problems {
            info!("Running benchmarks for problem: {}", problem.get_name());
            let results = self.run_problem_benchmarks(problem, &optimizers).await?;
            all_results.push((problem, results));
            tokio::task::yield_now().await;
        }

        // Generate comprehensive analysis and reports

        // Convert to the expected format with references
        let results_refs: Vec<(&ProblemSpec, BenchmarkResults)> = all_results
            .iter()
            .map(|(problem, results)| (*problem, results.clone()))
            .collect();

        self.plotting_manager
            .generate_all_plots(&results_refs)
            .await?;
        self.report_generator
            .generate_main_report(&results_refs, false)
            .await?;

        info!(
            "Benchmark experiments completed. Results saved to: {}",
            self.output_dir
        );
        tokio::task::yield_now().await;

        Ok(())
    }

    pub async fn validate_problems(&self, problems: &[ProblemSpec]) -> anyhow::Result<()> {
        for problem in problems {
            let initial_params = problem.problem.initial_point();
            let mut rng = rand::rngs::StdRng::try_from_os_rng()
                .expect("Failed to create random number generator");
            let initial_params: Vec<f64> = initial_params
                .iter()
                .map(|&x| x + rng.random_range(-1.0..1.0))
                .collect();

            // Try to evaluate with error handling for non-finite values
            let initial_value = match problem.problem.evaluate_f64(&initial_params) {
                Ok(val) if val.is_finite() => val,
                Ok(val) => {
                    warn!(
                        "Problem {} returned non-finite initial value: {}",
                        problem.get_name(),
                        val
                    );
                    continue;
                }
                Err(e) => {
                    warn!(
                        "Problem {} failed initial evaluation: {}",
                        problem.get_name(),
                        e
                    );
                    continue;
                }
            };

            info!(
                "Problem {}: initial_value = {:.6e}, dimensions = {}",
                problem.get_name(),
                initial_value,
                initial_params.len()
            );
            if initial_value < 1e-10 {
                warn!(
                    "Problem {} may be starting too close to optimum (initial_value = {:.6e})",
                    problem.get_name(),
                    initial_value
                );
            }
        }
        Ok(())
    }

    async fn run_problem_benchmarks(
        &self,
        problem: &ProblemSpec,
        optimizers: &[(String, Arc<dyn Optimizer>)],
    ) -> anyhow::Result<BenchmarkResults> {
        let runner = BenchmarkRunner::new(self.config.clone());
        let mut results = BenchmarkResults::new(self.config.clone());

        for (opt_name, ref optimizer) in optimizers.iter() {
            for run_id in 0..self.config.num_runs {
                let mut result = match runner
                    .run_single_benchmark(
                        problem,
                        &mut optimizer.clone_box(),
                        run_id,
                        opt_name,
                        self.config.initial_point_noise,
                    )
                    .await
                {
                    Ok(result) => result,
                    Err(e) => {
                        error!(
                            "Benchmark failed for {} with {}: {}",
                            problem.get_name(),
                            opt_name,
                            e
                        );
                        // Create a failed result instead of propagating the error
                        let mut failed_result = SingleResult::new(opt_name.clone(), run_id);
                        failed_result.convergence_achieved = false;
                        failed_result.final_value = f64::INFINITY;
                        failed_result.error_message = Some(format!("Evaluation error: {e}"));
                        results.add_result(failed_result);
                        continue;
                    }
                };

                if let Some(optimal_value) = problem.problem.optimal_value() {
                    let success_threshold = optimal_value;
                    result.convergence_achieved &=
                        result.best_value.is_finite() && result.best_value < success_threshold;
                } else {
                    result.convergence_achieved = false;
                }
                // Additional check for non-finite best values
                if !result.best_value.is_finite() {
                    warn!(
                        "Non-finite best value for {} with {}: {}",
                        problem.get_name(),
                        opt_name,
                        result.best_value
                    );
                    result.convergence_achieved = false;
                    if result.error_message.is_none() {
                        result.error_message =
                            Some(format!("Non-finite best value: {}", result.best_value));
                    }
                }

                results.add_result(result);
            }
        }

        Ok(results)
    }
}

pub async fn run_benchmark(
    report_path_prefix: &str,
    max_evals: usize,
    num_runs: usize,
    time_limit: Duration,
    problems: Vec<ProblemSpec>,
    optimizers: Vec<(String, Arc<dyn Optimizer>)>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Enable no threshold mode for benchmarks
    enable_no_threshold_mode();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{report_path_prefix}{timestamp}");
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir)?;
    println!("Creating benchmark results in: {}", output_dir.display());
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(
            output_dir.to_string_lossy().to_string(),
            BenchmarkConfig {
                max_iterations: max_evals,
                maximum_function_calls: max_evals,
                time_limit: DurationWrapper::from(time_limit),
                num_runs,
                ..BenchmarkConfig::default()
            },
        )
        .run_comparative_benchmarks(problems, optimizers),
    )
    .await;
    match result {
        Ok(Ok(())) => {
            println!("Benchmark completed successfully");
        }
        Ok(Err(e)) => {
            eprintln!("Benchmark failed: {e}");
            return Err(e.into());
        }
        Err(_) => {
            eprintln!("Benchmark timed out");
            return Err("Benchmark execution timed out".into());
        }
    }

    // Verify outputs were generated
    assert!(output_dir.join("benchmark_report.md").exists());
    // assert!(output_dir.join("detailed_results.csv").exists());
    // assert!(output_dir.join("summary_statistics.csv").exists());

    // Read and verify HTML content
    let html_content = fs::read_to_string(output_dir.join("benchmark_report.md"))?;
    assert!(html_content.contains("QQN Optimizer"));

    println!(
        "Comprehensive benchmark report generated at: {}",
        output_dir.display()
    );

    Ok(())
}

/// Extract optimizer family name from full optimizer name
pub fn get_optimizer_family(optimizer_name: &str) -> String {
    if optimizer_name.starts_with("QQN") {
        "QQN".to_string()
    } else if optimizer_name.starts_with("L-BFGS") {
        "L-BFGS".to_string()
    } else if optimizer_name.starts_with("Trust Region") {
        "Trust Region".to_string()
    } else if optimizer_name.starts_with("GD") {
        "GD".to_string()
    } else if optimizer_name.starts_with("Adam") {
        "Adam".to_string()
    } else {
        warn!("Unknown optimizer family for '{optimizer_name}', using full name");
        optimizer_name.to_string()
    }
}
