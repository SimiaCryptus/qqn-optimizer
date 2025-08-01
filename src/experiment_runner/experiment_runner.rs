#![allow(clippy::type_complexity)]

#[cfg(feature = "plotting")]
use super::PlottingManager;
use super::ReportGenerator;
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
    #[cfg(feature = "plotting")]
    plotting_manager: PlottingManager,
}

impl ExperimentRunner {
    pub fn new(output_dir: String, config: BenchmarkConfig) -> Self {
        Self {
            output_dir: output_dir.clone(),
            config: config.clone(),
            report_generator: ReportGenerator::new(output_dir.clone(), config.clone()),
            #[cfg(feature = "plotting")]
            plotting_manager: PlottingManager::new(output_dir),
        }
    }

    /// Run benchmarks with problem-specific optimizer sets
    pub async fn run_championship_benchmarks(
        &self,
        problems: Vec<ProblemSpec>,
        problem_optimizer_map: std::collections::HashMap<String, Vec<(String, Arc<dyn Optimizer>)>>,
    ) -> anyhow::Result<()> {
        info!("Starting championship benchmarks with problem-specific optimizers");
        
        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        let mut all_results: Vec<(&ProblemSpec, BenchmarkResults)> = Vec::new();
        
        // Run benchmarks for each problem with its specific optimizers
        for problem in &problems {
            let problem_name = problem.get_name();
            if let Some(optimizers) = problem_optimizer_map.get(&problem_name) {
                info!("Running championship benchmarks for problem: {}", problem_name);
                let results = self.run_problem_benchmarks(problem, optimizers).await?;
                all_results.push((problem, results));
            } else {
                warn!("No optimizers specified for problem: {}", problem_name);
            }
        }
        // Generate reports for championship results
        self.generate_reports(&all_results).await?;
        
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


        self.generate_reports(&all_results).await?;

        info!(
            "Benchmark experiments completed. Results saved to: {}",
            self.output_dir
        );
        tokio::task::yield_now().await;

        Ok(())
    }
    /// Generate reports and plots for benchmark results
    async fn generate_reports(
        &self,
        results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        #[cfg(feature = "plotting")]
        {
            self.plotting_manager
                .generate_all_plots(results)
                .await?;
        }
        self.report_generator
            .generate_main_report(results, false)
            .await?;
        Ok(())
    }


    pub async fn validate_problems(&self, problems: &[ProblemSpec]) -> anyhow::Result<()> {
        let mut validation_errors = Vec::new();
        
        for problem in problems {
            let initial_params = problem.problem.initial_point();
            let mut rng = rand::rngs::StdRng::try_from_os_rng()
                .map_err(|e| anyhow::anyhow!("Failed to create RNG: {}", e))?;
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
                    validation_errors.push(format!("Problem {} has non-finite initial value", problem.get_name()));
                    continue;
                }
                Err(e) => {
                    warn!(
                        "Problem {} failed initial evaluation: {}",
                        problem.get_name(),
                        e
                    );
                    validation_errors.push(format!("Problem {} failed evaluation: {}", problem.get_name(), e));
                    continue;
                }
            };
        if !validation_errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Problem validation failed:\n{}", 
                validation_errors.join("\n")
            ));
        }

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

        for (opt_name, optimizer) in optimizers.iter() {
            for run_id in 0..self.config.num_runs {
                // Yield to prevent blocking the async runtime
                if run_id % 10 == 0 {
                    tokio::task::yield_now().await;
                }
                
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
                // Check convergence against optimal value if available

                if let Some(optimal_value) = problem.problem.optimal_value() {
                    let tolerance = self.config.convergence_tolerance.unwrap_or(1e-6);
                    let success_threshold = optimal_value + tolerance;
                    result.convergence_achieved &=
                        result.best_value.is_finite() && result.best_value < success_threshold;
                } else {
                    // If no optimal value, use relative improvement criteria
                    result.convergence_achieved = result.best_value.is_finite();
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
) -> anyhow::Result<()> {
    // Enable no threshold mode for benchmarks
    enable_no_threshold_mode();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{report_path_prefix}{timestamp}");
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir)?;
    info!("Creating benchmark results in: {}", output_dir.display());
    
    // Create a timeout that's reasonable for the given parameters
    let total_timeout = Duration::from_secs(
        (time_limit.as_secs() * problems.len() as u64 * optimizers.len() as u64).max(300)
    );
    
    let result = tokio::time::timeout(
        total_timeout,
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
            info!("Benchmark completed successfully");
        }
        Ok(Err(e)) => {
            error!("Benchmark failed: {}", e);
            return Err(e);
        }
        Err(_) => {
            error!("Benchmark timed out after {:?}", total_timeout);
            return Err(anyhow::anyhow!("Benchmark execution timed out after {:?}", total_timeout));
        }
    }

    // Verify outputs were generated
    if !output_dir.join("benchmark_report.md").exists() {
        return Err(anyhow::anyhow!("Benchmark report was not generated"));
    }

    // Read and verify HTML content
    let html_content = fs::read_to_string(output_dir.join("benchmark_report.md"))?;
    assert!(html_content.contains("QQN Optimizer"));

    info!(
        "Comprehensive benchmark report generated at: {}",
        output_dir.display()
    );

    Ok(())
}

/// Extract optimizer family name from full optimizer name
pub fn get_optimizer_family(optimizer_name: &str) -> String {
    let families = [
        ("QQN", "QQN"),
        ("L-BFGS", "L-BFGS"),
        ("Trust Region", "Trust Region"),
        ("GD", "GD"),
        ("Adam", "Adam"),
    ];
    
    for (prefix, family) in &families {
        if optimizer_name.starts_with(prefix) {
            return family.to_string();
        }
    }
    
    warn!("Unknown optimizer family for '{}', using full name", optimizer_name);
    optimizer_name.to_string()
}