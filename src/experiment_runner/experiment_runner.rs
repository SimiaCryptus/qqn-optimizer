#![allow(clippy::type_complexity)]

#[cfg(feature = "plotting")]
use super::PlottingManager;
use super::ReportGenerator;
use crate::benchmarks::evaluation::{
    new_initial_point, BenchmarkConfig, BenchmarkResults, BenchmarkRunner, DurationWrapper,
    ProblemSpec, SingleResult,
};
use crate::Optimizer;
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
    pub async fn run_championship_benchmarks(
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
    pub async fn run_comparative_benchmarks(
        &self,
        problems: Vec<ProblemSpec>,
        optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    ) -> anyhow::Result<()> {
        info!("Starting comprehensive comparative benchmarks");

        // Ensure output directory exists
        fs::create_dir_all(self.output_dir.to_string())?;

        // Validate problems
        self.validate_problems(&problems).await?;

        // Run benchmarks for each problem with configurable parallelism
        let all_results = self.run_problems_parallel(problems, optimizers).await?;

        // Generate comprehensive analysis and reports

        // Convert to the expected format with references
        let results_refs: Vec<(&ProblemSpec, BenchmarkResults)> = all_results
            .iter()
            .map(|(problem, results)| (problem, results.clone()))
            .collect();

        #[cfg(feature = "plotting")]
        {
            self.plotting_manager
                .generate_all_plots(&results_refs)
                .await?;
        }
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
    /// Run multiple problems in parallel with controlled concurrency
    async fn run_problems_parallel(
        &self,
        problems: Vec<ProblemSpec>,
        optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    ) -> anyhow::Result<Vec<(ProblemSpec, BenchmarkResults)>> {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent_tasks));
        let mut tasks = Vec::new();
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
        for (problem_idx, problem) in problems.iter().enumerate() {
            let semaphore = semaphore.clone();
            let optimizers = optimizers.clone();
            let config = config.clone();
            let completed_count = completed_count.clone();
            let problem = problem.clone();
            let max_concurrent_per_problem =
                std::cmp::max(1, self.max_concurrent_tasks / problems.len());

            let mut rng = StdRng::seed_from_u64(42);
            let future = async move {
                let mut rng = StdRng::seed_from_u64(rng.random());
                let _permit = semaphore.acquire().await.unwrap();
                info!("Starting benchmarks for problem: {}", problem.get_name());
                let runner = BenchmarkRunner::new(config);
                let result = Self::run_problem_benchmarks_static(
                    &problem,
                    &optimizers,
                    &runner,
                    &mut rng,
                    max_concurrent_per_problem,
                )
                .await;
                let completed = completed_count.fetch_add(1, Ordering::SeqCst) + 1;
                info!(
                    "Completed problem {} ({}/{})",
                    problem.get_name(),
                    completed,
                    total_problems
                );
                result.map(|results| (problem_idx, results))
            };
            let task = tokio::spawn(future);
            tasks.push(task);
        }
        // Wait for all tasks to complete
        let mut all_results = Vec::new();
        for task in tasks {
            match task.await {
                Ok(Ok((problem_idx, results))) => {
                    // Clone the problem to avoid lifetime issues
                    let problem = problems[problem_idx].clone();
                    all_results.push((problem, results));
                }
                Ok(Err(e)) => {
                    error!("Problem benchmark failed: {}", e);
                    return Err(e);
                }
                Err(e) => {
                    error!("Task panicked: {}", e);
                    return Err(anyhow::anyhow!("Task execution failed: {}", e));
                }
            }
        }
        Ok(all_results)
    }

    pub async fn validate_problems(&self, problems: &[ProblemSpec]) -> anyhow::Result<()> {
        for problem in problems {
            let initial_params = problem.problem.initial_point();
            let mut rng = rand::rngs::StdRng::try_from_os_rng()
                .expect("Failed to create random number generator");
            let initial_params: Vec<f32> = initial_params
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
    async fn run_problem_benchmarks_static(
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

        for (opt_name, optimizer) in optimizers.iter() {
            for run_id in 0..config.num_runs {
                let semaphore = semaphore.clone();
                let optimizer = optimizer.clone();
                let opt_name = opt_name.clone();
                let problem = problem.clone();
                let config = config.clone();

                let mut rng = StdRng::seed_from_u64(rng.random());
                let future = async move {
                    let _permit = semaphore.acquire().await.unwrap();
                    let start = std::time::Instant::now();
                    Self::run_single_benchmark_static(
                        &problem, optimizer, run_id, &opt_name, config, &mut rng,
                    )
                    .await
                    .map(|result| {
                        info!(
                            "Completed benchmark: {} - {} (run {}) in {:?}",
                            problem.get_name(),
                            opt_name,
                            run_id,
                            start.elapsed()
                        );
                        result
                    })
                };
                // Use regular spawn instead of spawn_local
                let task = tokio::spawn(future);

                tasks.push(task);
            }
        }

        // Collect all results
        for task in tasks {
            match task.await {
                Ok(Ok(result)) => {
                    results.add_result(result);
                }
                Ok(Err(e)) => {
                    error!("Single benchmark failed: {}", e);
                    // Continue with other benchmarks rather than failing entirely
                }
                Err(e) => {
                    error!("Benchmark task panicked: {}", e);
                }
            }
        }
        Ok(results)
    }

    /// Static version of single benchmark run for parallel execution
    async fn run_single_benchmark_static(
        problem: &ProblemSpec,
        optimizer: Arc<dyn Optimizer>,
        run_id: usize,
        opt_name: &str,
        config: BenchmarkConfig,
        rng: &mut StdRng,
    ) -> anyhow::Result<SingleResult> {
        let runner = BenchmarkRunner::new(config.clone());
        let mut result = match runner
            .run_single_benchmark(
                problem,
                &mut optimizer.clone_box(),
                run_id,
                &opt_name.to_string(),
                new_initial_point(problem, config.initial_point_noise, rng),
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
                let mut failed_result = SingleResult::new(opt_name.to_string(), run_id);
                failed_result.convergence_achieved = false;
                failed_result.final_value = f32::INFINITY;
                failed_result.error_message = Some(format!("Evaluation error: {e}"));
                return Ok(failed_result);
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

        Ok(result)
    }
}

pub async fn run_benchmark(
    report_path_prefix: &str,
    max_evals: usize,
    num_runs: usize,
    time_limit: Duration,
    max_concurrent_tasks: Option<usize>,
    problems: Vec<ProblemSpec>,
    optimizers: Vec<(String, Arc<dyn Optimizer>)>,
    initial_point_noise: f32,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{report_path_prefix}{timestamp}");
    let output_dir = std::path::PathBuf::from(&output_dir_name.to_string());
    fs::create_dir_all(output_dir_name.to_string())?;
    println!("Creating benchmark results in: {}", output_dir.display());
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(
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
