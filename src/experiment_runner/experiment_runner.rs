#![allow(clippy::type_complexity)]

#[cfg(feature = "plotting")]
use super::PlottingManager;
use super::ReportGenerator;
use crate::benchmarks::evaluation::{
    new_initial_point, BenchmarkConfig, BenchmarkResults, BenchmarkRunner, DurationWrapper,
    ProblemSpec, SingleResult,
};
use crate::Optimizer;
use dfdx::shapes::Shape;
use log::{error, info, warn};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

/// Core experiment runner focused on benchmark execution
/// derives clone, Debug, Default, Serialize, Deserialize
#[derive(Clone)]
pub struct ExperimentRunner {
    pub(crate) output_dir: String,
    config: BenchmarkConfig,
    max_concurrent_tasks: usize,
    pub(crate) report_generator: ReportGenerator,
    #[cfg(feature = "plotting")]
    pub(crate) plotting_manager: PlottingManager,
}

impl ExperimentRunner {
    pub fn new(
        output_dir: String,
        config: BenchmarkConfig,
        max_concurrent_tasks: Option<usize>,
    ) -> Self {
        let max_concurrent_tasks = max_concurrent_tasks.unwrap_or_else(|| {
            // Default to number of CPU cores, but cap at 8 to avoid overwhelming the system
            std::cmp::min(num_cpus::get(), 8)
        });

        info!(
            "Experiment runner configured with {} concurrent tasks",
            max_concurrent_tasks
        );

        Self {
            output_dir: output_dir.clone(),
            config: config.clone(),
            max_concurrent_tasks,
            report_generator: ReportGenerator::new(output_dir.clone(), config.clone()),
            #[cfg(feature = "plotting")]
            plotting_manager: PlottingManager::new(output_dir),
        }
    }

    /// Run benchmarks with problem-specific optimizer sets
    pub fn run_championship_benchmarks(
        &self,
        problem_optimizer_map: std::collections::HashMap<String, Vec<(String, Arc<dyn Optimizer>)>>,
    ) -> anyhow::Result<()> {
        info!("Starting championship benchmarks with problem-specific optimizers");
        // Ensure output directory exists
        fs::create_dir_all(self.output_dir.to_string())?;
        // Run benchmarks for each problem with its specific optimizers
        for problem_name in problem_optimizer_map.keys() {
            // Find the problem by name (we'll need to pass problems separately or store them)
            info!("Running championship benchmarks for problem: {problem_name}");
            // This will be handled by the calling function
        }
        Ok(())
    }

    /// Run comprehensive comparative benchmarks
    pub fn run_comparative_benchmarks(
        &self,
        problems: Vec<ProblemSpec>,
        optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    ) -> anyhow::Result<()> {
        info!("Starting comprehensive comparative benchmarks");

        // Ensure output directory exists
        fs::create_dir_all(self.output_dir.to_string())?;

        // Validate problems
        self.validate_problems(&problems)?;

        // Run benchmarks for each problem with configurable parallelism
        let all_results = self.run_problems_parallel(problems, optimizers)?;

        // Generate comprehensive analysis and reports

        // Convert to the expected format with references
        let results_refs: Vec<(&ProblemSpec, BenchmarkResults)> = all_results
            .iter()
            .map(|(problem, results)| (problem, results.clone()))
            .collect();

        #[cfg(feature = "plotting")]
        {
            self.plotting_manager
                .generate_all_plots(&results_refs);
        }
        self.report_generator
            .generate_main_report(&results_refs, false);

        info!(
            "Benchmark experiments completed. Results saved to: {}",
            self.output_dir
        );
        tokio::task::yield_now();

        Ok(())
    }
    /// Run multiple problems in parallel with controlled concurrency
    fn run_problems_parallel(
        &self,
        problems: Vec<ProblemSpec>,
        optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    ) -> anyhow::Result<Vec<(ProblemSpec, BenchmarkResults)>> {
        let completed_count = Arc::new(AtomicUsize::new(0));
        let total_problems = problems.len();
        let config = self.config.clone();
        let total_tasks = problems.len() * optimizers.len() * config.num_runs;
        info!(
            "Starting {} total benchmark tasks across {} problems with {} max concurrent",
            total_tasks, total_problems, self.max_concurrent_tasks
        );

        // Store problems in a way that allows sharing across tasks
        let problems = Arc::new(problems);
        let optimizers = Arc::new(optimizers);
        let mut tasks = Vec::new();
        for (problem_idx, problem) in problems.iter().enumerate() {
            let optimizers = optimizers.clone();
            let config = config.clone();
            let completed_count = completed_count.clone();
            let problem = problem.clone();
            let max_concurrent_per_problem =
                std::cmp::max(1, self.max_concurrent_tasks / problems.len());

            let mut rng = StdRng::seed_from_u64(42);
            let future = {
                let mut rng = StdRng::seed_from_u64(rng.random());
                info!("Starting benchmarks for problem: {}", problem.get_name());
                let runner = BenchmarkRunner::new(config);
                let result = Self::run_problem_benchmarks_static(
                    &problem,
                    &optimizers,
                    &runner,
                    &mut rng,
                    max_concurrent_per_problem,
                );
                let completed = completed_count.fetch_add(1, Ordering::SeqCst) + 1;
                info!(
                    "Completed problem {} ({}/{})",
                    problem.get_name(),
                    completed,
                    total_problems
                );
                result.map(|results| (problem_idx, results))
            };
            tasks.push(future);
        }
        // Wait for all tasks to complete
        let mut all_results = Vec::new();
        for task in tasks {
            match task {
                Ok((problem_idx, results)) => {
                    // Clone the problem to avoid lifetime issues
                    let problem = problems[problem_idx].clone();
                    all_results.push((problem, results));
                }
                Err(e) => {
                    error!("Problem benchmark failed: {}", e);
                    return Err(e);
                }
            }
        }
        Ok(all_results)
    }

    pub fn validate_problems(&self, problems: &[ProblemSpec]) -> anyhow::Result<()> {
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

    /// Static version of run_problem_benchmarks for use in parallel tasks
    fn run_problem_benchmarks_static(
        problem: &ProblemSpec,
        optimizers: &[(String, Arc<dyn Optimizer>)],
        runner: &BenchmarkRunner,
        rng: &mut StdRng,
        max_concurrent: usize,
    ) -> anyhow::Result<BenchmarkResults> {
        let mut results = BenchmarkResults::new(runner.config.clone());

        // Run optimizer benchmarks with controlled parallelism within each problem
        let semaphore = Arc::new(Semaphore::new(max_concurrent));
        let mut tasks = Vec::new();
        let config = runner.config.clone();
        let total_optimizer_tasks = optimizers.len() * config.num_runs;
        info!(
            "Running {} optimizer tasks for problem {} with {} max concurrent",
            total_optimizer_tasks,
            problem.get_name(),
            max_concurrent
        );
        let mut rng = StdRng::seed_from_u64(rng.random());
        let problem = problem.clone();
        let mut point = new_initial_point(&problem, config.initial_point_noise, &mut rng)?;
        let (mut graph, mut loss, grads, result) = BenchmarkRunner::compile(&problem, &mut point);

        for (opt_name, optimizer) in optimizers.iter() {
            for run_id in 0..config.num_runs {
                let optimizer = optimizer.clone();
                let opt_name = opt_name.clone();
                let config = config.clone();

                // Use regular spawn instead of spawn_local

                let start = std::time::Instant::now();
                let problem1 = &problem;
                let opt_name1 = &opt_name;
                let runner1 = BenchmarkRunner::new(config.clone());
                let opt_name2 = &opt_name1.to_string();
                let mut point = new_initial_point(problem1, config.initial_point_noise, &mut rng)?;
                let mut result = runner1.run(
                    problem1, optimizer.clone_box(), run_id, opt_name2, Arc::get_mut(&mut graph).expect("Graph should be unique"), &mut point, &mut loss, grads.clone(),
                    result.clone()?,
                )?;

                if let Some(optimal_value) = problem1.problem.optimal_value() {
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
                            problem1.get_name(),
                            opt_name1,
                            result.best_value
                        );
                    result.convergence_achieved = false;
                    if result.error_message.is_none() {
                        result.error_message =
                            Some(format!("Non-finite best value: {}", result.best_value));
                    }
                }
                info!(
                            "Completed benchmark: {} - {} (run {}) in {:?}",
                            problem.get_name(),
                            opt_name,
                            run_id,
                            start.elapsed()
                        );
                tasks.push(result);
            }
        }

        // Collect all results
        for result in tasks {
            results.add_result(result);
        }
        Ok(results)
    }
}

pub fn run_benchmark(
    report_path_prefix: &str,
    max_evals: usize,
    num_runs: usize,
    time_limit: Duration,
    max_concurrent_tasks: Option<usize>,
    problems: Vec<ProblemSpec>,
    optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    initial_point_noise: f64,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{report_path_prefix}{timestamp}");
    let output_dir = std::path::PathBuf::from(&output_dir_name.to_string());
    fs::create_dir_all(output_dir_name.to_string())?;
    println!("Creating benchmark results in: {}", output_dir.display());
    let result =         ExperimentRunner::new(
        output_dir.to_string_lossy().to_string(),
        BenchmarkConfig {
            max_iterations: max_evals,
            maximum_function_calls: max_evals,
            time_limit: DurationWrapper::from(time_limit),
            initial_point_noise,
            num_runs,
            ..BenchmarkConfig::default()
        },
        max_concurrent_tasks,
    )
        .run_comparative_benchmarks(problems, optimizers);
    match result {
        Ok(()) => {
            println!("Benchmark completed successfully");
        }
        Err(_) => {
            eprintln!("Benchmark timed out");
            return Err("Benchmark execution timed out".into());
        }
    }

    // Verify outputs were generated
    assert!(output_dir.join("benchmark_report.md".to_string()).exists());
    // assert!(output_dir.join("detailed_results.csv").exists());
    // assert!(output_dir.join("summary_statistics.csv").exists());

    // Read and verify HTML content
    let html_content = fs::read_to_string(output_dir.join("benchmark_report.md".to_string()))?;
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
    } else if optimizer_name.starts_with("LBFGS") {
        "L-BFGS".to_string()
    } else if optimizer_name.starts_with("L-BFGS") {
        "L-BFGS".to_string()
    } else if optimizer_name.starts_with("Trust Region") {
        "Trust Region".to_string()
    } else if optimizer_name.starts_with("TrustRegion") {
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
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_optimizer_family() {
        assert_eq!(get_optimizer_family("QQN-123"), "QQN");
        assert_eq!(get_optimizer_family("LBFGS-Default"), "L-BFGS");
        assert_eq!(get_optimizer_family("L-BFGS-Strong"), "L-BFGS");
        assert_eq!(get_optimizer_family("Trust Region Method"), "Trust Region");
        assert_eq!(get_optimizer_family("TrustRegion"), "Trust Region");
        assert_eq!(get_optimizer_family("GD-Momentum"), "GD");
        assert_eq!(get_optimizer_family("Adam-W"), "Adam");
        assert_eq!(get_optimizer_family("Unknown"), "Unknown");
    }
}