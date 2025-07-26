use super::{PlottingManager, ReportGenerator};
use log::{info, warn, error};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults, BenchmarkRunner, DurationWrapper, SingleResult};
use qqn_optimizer::{ OptimizationProblem, Optimizer};
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
    
    /// Run comprehensive comparative benchmarks
    pub async fn run_comparative_benchmarks(&self, problems: Vec<Arc<dyn OptimizationProblem>>, optimizers: Vec<(String, Arc<dyn Optimizer>)>) -> anyhow::Result<()> {
        info!("Starting comprehensive comparative benchmarks");

        // Ensure output directory exists
        fs::create_dir_all(&self.output_dir)?;
        
        // Validate problems
        self.validate_problems(&problems).await?;

        // Run benchmarks for each problem
        let mut all_results: Vec<(&Arc<dyn OptimizationProblem>, BenchmarkResults)> = Vec::new();

        for problem in &problems {
            info!("Running benchmarks for problem: {}", problem.name());
            let results = self
                .run_problem_benchmarks(problem.as_ref(), &optimizers)
                .await?;
            all_results.push((problem, results));
            tokio::task::yield_now().await;
        }

        // Generate comprehensive analysis and reports
        self.plotting_manager.generate_all_plots(&all_results).await?;
        self.report_generator.generate_main_report(&all_results, &problems).await?;

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
            
            // Try to evaluate with error handling for non-finite values
            let initial_value = match problem.evaluate_f64(&initial_params) {
                Ok(val) if val.is_finite() => val,
                Ok(val) => {
                    warn!(
                        "Problem {} returned non-finite initial value: {}",
                        problem.name(),
                        val
                    );
                    continue;
                }
                Err(e) => {
                    warn!(
                        "Problem {} failed initial evaluation: {}",
                        problem.name(),
                        e
                    );
                    continue;
                }
            };
            
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
                let mut result = match runner
                    .run_single_benchmark(problem, &mut optimizer.clone_box(), run_id, &opt_name)
                    .await
                {
                    Ok(result) => result,
                    Err(e) => {
                        error!(
                            "Benchmark failed for {} with {}: {}",
                            problem.name(),
                            opt_name,
                            e
                        );
                        // Create a failed result instead of propagating the error
                        let mut failed_result = SingleResult::new(opt_name.clone(), run_id);
                        failed_result.convergence_achieved = false;
                        failed_result.final_value = f64::INFINITY;
                        failed_result.error_message = Some(format!("Evaluation error: {}", e));
                        results.add_result(failed_result);
                        continue;
                    }
                };

                if let Some(optimal_value) = problem.optimal_value() {
                    let success_threshold = optimal_value;
                    result.convergence_achieved &= result.final_value.is_finite() && result.final_value < success_threshold;
                } else {
                    result.convergence_achieved = false;
                }
                // Additional check for non-finite final values
                if !result.final_value.is_finite() {
                    warn!(
                        "Non-finite final value for {} with {}: {}",
                        problem.name(),
                        opt_name,
                        result.final_value
                    );
                    result.convergence_achieved = false;
                    if result.error_message.is_none() {
                        result.error_message = Some(format!("Non-finite final value: {}", result.final_value));
                    }
                }
                
                results.add_result(result);
            }
        }

        Ok(results)
    }
}

pub async fn run_benchmark(report_path_prefix: &str, max_evals: usize, num_runs: usize, time_limit: Duration, problems: Vec<Arc<dyn OptimizationProblem>>, optimizers: Vec<(String, Arc<dyn Optimizer>)>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("{}{}", report_path_prefix, timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir)?;
    println!("Creating benchmark results in: {}", output_dir.display());
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(output_dir.to_string_lossy().to_string(), BenchmarkConfig {
            max_iterations: max_evals,
            maximum_function_calls: max_evals,
            time_limit: DurationWrapper::from(time_limit),
            num_runs: num_runs,
            ..BenchmarkConfig::default()
        }).run_comparative_benchmarks(problems, optimizers),
    ).await;
    match result {
        Ok(Ok(())) => {
            println!("Benchmark completed successfully");
        }
        Ok(Err(e)) => {
            eprintln!("Benchmark failed: {}", e);
            return Err(e.into());
        }
        Err(_) => {
            eprintln!("Benchmark timed out");
            return Err("Benchmark execution timed out".into());
        }
    }

    // Verify outputs were generated
    assert!(output_dir.join("benchmark_report.md").exists());
    assert!(output_dir.join("detailed_results.csv").exists());
    assert!(output_dir.join("summary_statistics.csv").exists());

    // Read and verify HTML content
    let html_content = fs::read_to_string(output_dir.join("benchmark_report.md"))?;
    assert!(html_content.contains("QQN Optimizer"));

    println!(
        "Comprehensive benchmark report generated at: {}",
        output_dir.display()
    );

    Ok(())
}