use std::collections::HashMap;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use tokio::time::timeout;
use crate::core::optimizer::{Optimizer, StepResult, ConvergenceInfo};
use crate::benchmarks::functions::OptimizationProblem;
use crate::utils::math::compute_magnitude;

/// Configuration for benchmark execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub max_iterations: usize,
    pub tolerance: f64,
    pub max_function_evaluations: usize,
    pub time_limit: Duration,
    pub random_seed: u64,
    pub num_runs: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            max_iterations: 1000,
            tolerance: 1e-6,
            max_function_evaluations: 10000,
            time_limit: Duration::from_secs(600), // 10 minutes
            random_seed: 42,
            num_runs: 10,
        }
    }
}

/// Trace of optimization progress for a single run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationTrace {
    pub iterations: Vec<IterationData>,
    pub total_function_evaluations: usize,
    pub total_gradient_evaluations: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IterationData {
    pub iteration: usize,
    pub function_value: f64,
    pub gradient_norm: f64,
    pub step_size: f64,
    pub parameters: Vec<f64>,
    pub timestamp: Duration,
}

impl OptimizationTrace {
    pub fn new() -> Self {
        Self {
            iterations: Vec::new(),
            total_function_evaluations: 0,
            total_gradient_evaluations: 0,
        }
    }

    pub fn record_iteration(
        &mut self,
        iteration: usize,
        function_value: f64,
        parameters: &[f64],
        gradient: &[f64],
        step_size: f64,
        timestamp: Duration,
    ) {
        let gradient_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
        
        self.iterations.push(IterationData {
            iteration,
            function_value,
            gradient_norm,
            step_size,
            parameters: parameters.to_vec(),
            timestamp,
        });
    }

    pub fn final_value(&self) -> Option<f64> {
        self.iterations.last().map(|data| data.function_value)
    }

    pub fn final_gradient_norm(&self) -> Option<f64> {
        self.iterations.last().map(|data| data.gradient_norm)
    }
}

/// Results from a single optimization run
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleResult {
    pub problem_name: String,
    pub optimizer_name: String,
    pub run_id: usize,
    pub final_value: f64,
    pub final_gradient_norm: f64,
    pub iterations: usize,
    pub function_evaluations: usize,
    pub gradient_evaluations: usize,
    pub convergence_achieved: bool,
    pub execution_time: Duration,
    pub trace: OptimizationTrace,
    pub convergence_reason: ConvergenceReason,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConvergenceReason {
    GradientTolerance,
    FunctionTolerance,
    MaxIterations,
    MaxFunctionEvaluations,
    TimeLimit,
    NumericalError,
}

/// Collection of benchmark results
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    pub results: Vec<SingleResult>,
    pub config: BenchmarkConfig,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl BenchmarkResults {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            results: Vec::new(),
            config,
            timestamp: chrono::Utc::now(),
        }
    }

    pub fn add_result(&mut self, result: SingleResult) {
        self.results.push(result);
    }

    pub fn get_results_for_problem(&self, problem_name: &str) -> Vec<&SingleResult> {
        self.results
            .iter()
            .filter(|r| r.problem_name == problem_name)
            .collect()
    }

    pub fn get_results_for_optimizer(&self, optimizer_name: &str) -> Vec<&SingleResult> {
        self.results
            .iter()
            .filter(|r| r.optimizer_name == optimizer_name)
            .collect()
    }

    pub fn get_problem_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.results
            .iter()
            .map(|r| r.problem_name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        names.sort();
        names
    }

    pub fn get_optimizer_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.results
            .iter()
            .map(|r| r.optimizer_name.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        names.sort();
        names
    }
}

/// Main benchmark runner
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self { config }
    }

    /// Run benchmarks for all combinations of problems and optimizers
    pub async fn run_benchmarks(
        &self,
        problems: Vec<Box<dyn OptimizationProblem>>,
        optimizers: Vec<Box<dyn Optimizer>>,
    ) -> Result<BenchmarkResults, BenchmarkError> {
        let mut results = BenchmarkResults::new(self.config.clone());
        
        for problem in &problems {
            for optimizer in &optimizers {
                for run_id in 0..self.config.num_runs {
                    let result = self.run_single_benchmark(
                        problem.as_ref(),
                        optimizer.as_ref(),
                        run_id,
                    ).await?;
                    
                    results.add_result(result);
                }
            }
        }

        Ok(results)
    }

    /// Run a single benchmark with one problem and one optimizer
    async fn run_single_benchmark(
        &self,
        problem: &dyn OptimizationProblem,
        optimizer: &dyn Optimizer,
        run_id: usize,
    ) -> Result<SingleResult, BenchmarkError> {
        // Set random seed for reproducibility
        let seed = self.config.random_seed + run_id as u64;
        
        // Clone optimizer for this run
        let mut opt = optimizer.clone_box();
        opt.reset();

        // Initialize parameters
        let mut x = problem.initial_point();
        let mut iteration = 0;
        let mut function_evaluations = 0;
        let mut gradient_evaluations = 0;
        let start_time = Instant::now();

        let mut trace = OptimizationTrace::new();

        // Main optimization loop with timeout
        let optimization_result = timeout(
            self.config.time_limit,
            self.optimization_loop(
                problem,
                opt.as_mut(),
                &mut x,
                &mut iteration,
                &mut function_evaluations,
                &mut gradient_evaluations,
                &mut trace,
                start_time,
            )
        ).await;

        let (convergence_achieved, convergence_reason) = match optimization_result {
            Ok(Ok(reason)) => (reason == ConvergenceReason::GradientTolerance || 
                              reason == ConvergenceReason::FunctionTolerance, reason),
            Ok(Err(_)) => (false, ConvergenceReason::NumericalError),
            Err(_) => (false, ConvergenceReason::TimeLimit),
        };

        // Final evaluation
        let final_value = problem.evaluate(&x).map_err(BenchmarkError::ProblemError)?;
        let final_gradient = problem.gradient(&x).map_err(BenchmarkError::ProblemError)?;
        let final_gradient_norm = final_gradient.iter().map(|g| g * g).sum::<f64>().sqrt();

        Ok(SingleResult {
            problem_name: problem.name().to_string(),
            optimizer_name: opt.name().to_string(),
            run_id,
            final_value,
            final_gradient_norm,
            iterations: iteration,
            function_evaluations,
            gradient_evaluations,
            convergence_achieved,
            execution_time: start_time.elapsed(),
            trace,
            convergence_reason,
        })
    }

    async fn optimization_loop(
        &self,
        problem: &dyn OptimizationProblem,
        optimizer: &mut dyn Optimizer,
        x: &mut [f64],
        iteration: &mut usize,
        function_evaluations: &mut usize,
        gradient_evaluations: &mut usize,
        trace: &mut OptimizationTrace,
        start_time: Instant,
    ) -> Result<ConvergenceReason, BenchmarkError> {
        while *iteration < self.config.max_iterations {
            // Evaluate function and gradient
            let f_val = problem.evaluate(x).map_err(BenchmarkError::ProblemError)?;
            *function_evaluations += 1;

            let gradient = problem.gradient(x).map_err(BenchmarkError::ProblemError)?;
            *gradient_evaluations += 1;

            // Record iteration data
            trace.record_iteration(
                *iteration,
                f_val,
                x,
                &gradient,
                0.0, // Will be updated after step
                start_time.elapsed(),
            );

            // Check convergence
            let gradient_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
            if gradient_norm < self.config.tolerance {
                return Ok(ConvergenceReason::GradientTolerance);
            }

            // Check function evaluation limit
            if *function_evaluations >= self.config.max_function_evaluations {
                return Ok(ConvergenceReason::MaxFunctionEvaluations);
            }

            // Perform optimization step
            let step_result = optimizer.step(x, &gradient)
                .map_err(BenchmarkError::OptimizerError)?;

            // Update counters
            *function_evaluations += step_result.function_evaluations;
            *gradient_evaluations += step_result.gradient_evaluations;

            // Update step size in trace
            if let Some(last_iteration) = trace.iterations.last_mut() {
                last_iteration.step_size = step_result.step_size;
            }

            // Check for numerical errors
            if x.iter().any(|&xi| !xi.is_finite()) {
                return Ok(ConvergenceReason::NumericalError);
            }

            *iteration += 1;
        }

        Ok(ConvergenceReason::MaxIterations)
    }

    fn check_convergence(
        &self,
        function_value: f64,
        gradient: &[f64],
        problem: &dyn OptimizationProblem,
    ) -> bool {
        // Gradient norm convergence
        let gradient_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
        if gradient_norm < self.config.tolerance {
            return true;
        }

        // Function value convergence (if optimal value is known)
        if let Some(optimal_value) = problem.optimal_value() {
            let function_tolerance = (function_value - optimal_value).abs();
            if function_tolerance < self.config.tolerance {
                return true;
            }
        }

        false
    }
}

/// Benchmark execution errors
#[derive(Debug, thiserror::Error)]
pub enum BenchmarkError {
    #[error("Problem evaluation error: {0}")]
    ProblemError(String),
    
    #[error("Optimizer error: {0}")]
    OptimizerError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Utility functions for benchmark analysis
impl BenchmarkResults {
    /// Calculate success rate for each optimizer
    pub fn success_rates(&self) -> HashMap<String, f64> {
        let mut rates = HashMap::new();
        
        for optimizer_name in self.get_optimizer_names() {
            let results = self.get_results_for_optimizer(&optimizer_name);
            let successful = results.iter().filter(|r| r.convergence_achieved).count();
            let total = results.len();
            
            rates.insert(optimizer_name, successful as f64 / total as f64);
        }
        
        rates
    }

    /// Calculate average final values for each optimizer on each problem
    pub fn average_final_values(&self) -> HashMap<(String, String), f64> {
        let mut averages = HashMap::new();
        
        for problem_name in self.get_problem_names() {
            for optimizer_name in self.get_optimizer_names() {
                let results: Vec<_> = self.results
                    .iter()
                    .filter(|r| r.problem_name == problem_name && r.optimizer_name == optimizer_name)
                    .collect();
                
                if !results.is_empty() {
                    let avg = results.iter().map(|r| r.final_value).sum::<f64>() / results.len() as f64;
                    averages.insert((problem_name.clone(), optimizer_name.clone()), avg);
                }
            }
        }
        
        averages
    }

    /// Calculate average execution times
    pub fn average_execution_times(&self) -> HashMap<String, Duration> {
        let mut times = HashMap::new();
        
        for optimizer_name in self.get_optimizer_names() {
            let results = self.get_results_for_optimizer(&optimizer_name);
            let total_time: Duration = results.iter().map(|r| r.execution_time).sum();
            let avg_time = total_time / results.len() as u32;
            
            times.insert(optimizer_name, avg_time);
        }
        
        times
    }

    /// Save results to JSON file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<(), BenchmarkError> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    /// Load results from JSON file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self, BenchmarkError> {
        let json = std::fs::read_to_string(path)?;
        let results = serde_json::from_str(&json)?;
        Ok(results)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::functions::SphereFunction;
    use crate::core::lbfgs::{LBFGSOptimizer, LBFGSConfig};

    #[tokio::test]
    async fn test_benchmark_runner() {
        let config = BenchmarkConfig {
            max_iterations: 100,
            tolerance: 1e-6,
            num_runs: 2,
            ..Default::default()
        };

        let runner = BenchmarkRunner::new(config);
        
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(2)),
        ];
        
        let optimizers: Vec<Box<dyn Optimizer>> = vec![
            Box::new(LBFGSOptimizer::new(LBFGSConfig::default())),
        ];

        let results = runner.run_benchmarks(problems, optimizers).await.unwrap();
        
        assert_eq!(results.results.len(), 2); // 1 problem × 1 optimizer × 2 runs
        assert!(results.results.iter().all(|r| r.convergence_achieved));
    }

    #[test]
    fn test_benchmark_results_analysis() {
        let config = BenchmarkConfig::default();
        let mut results = BenchmarkResults::new(config);
        
        // Add mock results
        results.add_result(SingleResult {
            problem_name: "sphere".to_string(),
            optimizer_name: "lbfgs".to_string(),
            run_id: 0,
            final_value: 1e-8,
            final_gradient_norm: 1e-6,
            iterations: 50,
            function_evaluations: 100,
            gradient_evaluations: 50,
            convergence_achieved: true,
            execution_time: Duration::from_millis(100),
            trace: OptimizationTrace::new(),
            convergence_reason: ConvergenceReason::GradientTolerance,
        });

        let success_rates = results.success_rates();
        assert_eq!(success_rates.get("lbfgs"), Some(&1.0));
        
        let avg_values = results.average_final_values();
        assert_eq!(avg_values.get(&("sphere".to_string(), "lbfgs".to_string())), Some(&1e-8));
    }
}