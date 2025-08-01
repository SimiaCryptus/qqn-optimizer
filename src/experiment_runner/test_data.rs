use crate::benchmarks::evaluation::{
    BenchmarkResults, ConvergenceReason, PerformanceMetrics, ProblemSpec, SingleResult,
};
use crate::OptimizationProblem;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

pub fn create_test_data() -> Vec<(ProblemSpec, BenchmarkResults)> {
    vec![
        // Rosenbrock family problems
        (
            create_mock_problem_spec("rosenbrock_2d"),
            BenchmarkResults {
                results: vec![
                    create_mock_benchmark_result("lbfgs_default", 0.001, true, 150, 50),
                    create_mock_benchmark_result("lbfgs_aggressive", 0.0005, true, 120, 40),
                    create_mock_benchmark_result("adam_default", 0.1, false, 1000, 0),
                    create_mock_benchmark_result("adam_adaptive", 0.05, true, 800, 0),
                    create_mock_benchmark_result("sgd_momentum", 0.5, false, 2000, 0),
                    create_mock_benchmark_result("nelder_mead_standard", 0.01, true, 300, 0),
                ],
                config: Default::default(),
                timestamp: SystemTime::now(),
                convergence_achieved: true,
                final_value: Some(0.0005),
                function_evaluations: 3570,
                gradient_evaluations: 290,
            },
        ),
        (
            create_mock_problem_spec("rosenbrock_10d"),
            BenchmarkResults {
                results: vec![
                    create_mock_benchmark_result("lbfgs_default", 0.1, true, 500, 200),
                    create_mock_benchmark_result("lbfgs_aggressive", 0.05, true, 400, 150),
                    create_mock_benchmark_result("adam_default", 1.0, false, 5000, 0),
                    create_mock_benchmark_result("adam_adaptive", 0.8, false, 4000, 0),
                    create_mock_benchmark_result("sgd_momentum", 2.0, false, 8000, 0),
                    create_mock_benchmark_result("nelder_mead_standard", 0.2, true, 1500, 0),
                ],
                config: Default::default(),
                timestamp: SystemTime::now(),
                convergence_achieved: true,
                final_value: Some(0.05),
                function_evaluations: 18900,
                gradient_evaluations: 350,
            },
        ),
        // Sphere family problems
        (
            create_mock_problem_spec("sphere_2d"),
            BenchmarkResults {
                results: vec![
                    create_mock_benchmark_result("lbfgs_default", 1e-8, true, 50, 20),
                    create_mock_benchmark_result("lbfgs_aggressive", 1e-9, true, 40, 15),
                    create_mock_benchmark_result("adam_default", 1e-4, true, 200, 0),
                    create_mock_benchmark_result("adam_adaptive", 1e-5, true, 150, 0),
                    create_mock_benchmark_result("sgd_momentum", 1e-3, true, 500, 0),
                    create_mock_benchmark_result("nelder_mead_standard", 1e-6, true, 100, 0),
                ],
                config: Default::default(),
                timestamp: SystemTime::now(),
                convergence_achieved: true,
                final_value: Some(1e-9),
                function_evaluations: 1040,
                gradient_evaluations: 35,
            },
        ),
        (
            create_mock_problem_spec("sphere_10d"),
            BenchmarkResults {
                results: vec![
                    create_mock_benchmark_result("lbfgs_default", 1e-7, true, 100, 50),
                    create_mock_benchmark_result("lbfgs_aggressive", 1e-8, true, 80, 40),
                    create_mock_benchmark_result("adam_default", 1e-3, true, 400, 0),
                    create_mock_benchmark_result("adam_adaptive", 1e-4, true, 300, 0),
                    create_mock_benchmark_result("sgd_momentum", 1e-2, false, 1000, 0),
                    create_mock_benchmark_result("nelder_mead_standard", 1e-5, true, 200, 0),
                ],
                config: Default::default(),
                timestamp: SystemTime::now(),
                convergence_achieved: true,
                final_value: Some(1e-8),
                function_evaluations: 2080,
                gradient_evaluations: 90,
            },
        ),
        // Rastrigin family problems
        (
            create_mock_problem_spec("rastrigin_2d"),
            BenchmarkResults {
                results: vec![
                    create_mock_benchmark_result("lbfgs_default", 5.0, false, 1000, 300),
                    create_mock_benchmark_result("lbfgs_aggressive", 3.0, false, 800, 250),
                    create_mock_benchmark_result("adam_default", 2.0, true, 2000, 0),
                    create_mock_benchmark_result("adam_adaptive", 1.5, true, 1500, 0),
                    create_mock_benchmark_result("sgd_momentum", 8.0, false, 5000, 0),
                    create_mock_benchmark_result("nelder_mead_standard", 4.0, false, 2000, 0),
                ],
                config: Default::default(),
                timestamp: SystemTime::now(),
                convergence_achieved: true,
                final_value: Some(1.5),
                function_evaluations: 12300,
                gradient_evaluations: 550,
            },
        ),
    ]
}

fn create_mock_benchmark_result(
    optimizer_name: &str,
    best_value: f64,
    convergence_achieved: bool,
    function_evaluations: u32,
    gradient_evaluations: u32,
) -> SingleResult {
    let iterations = match optimizer_name {
        name if name.starts_with("lbfgs") => function_evaluations / 3,
        name if name.starts_with("adam") || name.starts_with("sgd") => function_evaluations,
        _ => function_evaluations / 2,
    };
    let execution_time = Duration::from_millis(function_evaluations as u64 * 2);
    let gradient_norm = if convergence_achieved { 1e-6 } else { 0.1 };
    
    SingleResult {
        optimizer_name: optimizer_name.to_string(),
        run_id: 0,
        final_value: best_value * 1.1, // Final value slightly worse than best
        best_value,
        final_gradient_norm: gradient_norm,
        convergence_achieved,
        function_evaluations: function_evaluations.try_into().unwrap(),
        gradient_evaluations: gradient_evaluations.try_into().unwrap(),
        execution_time,
        trace: Default::default(),
        convergence_reason: if convergence_achieved {
            ConvergenceReason::GradientTolerance
        } else {
            ConvergenceReason::MaxIterations
        },
        memory_usage: None,
        performance_metrics: PerformanceMetrics {
            iterations_per_second: iterations as f64 / execution_time.as_secs_f64(),
            function_evaluations_per_second: function_evaluations as f64 / execution_time.as_secs_f64(),
            gradient_evaluations_per_second: gradient_evaluations as f64 / execution_time.as_secs_f64(),
            convergence_rate: if convergence_achieved { 0.95 } else { 0.0 },
        },
        problem_name: optimizer_name.split('_').next().unwrap_or("unknown").to_string(),
        iterations,
        error_message: None,
    }
}

fn create_mock_problem_spec(name: &str) -> ProblemSpec {
    let dimensions = if name.contains("2d") { 2 } else { 10 };
    let mock_problem = MockProblem {
        name: name.to_string(),
        dimensions,
    };
    ProblemSpec::new(Arc::new(mock_problem), name.to_string(), Some(dimensions), 42)
}

// Mock optimization problem for testing
struct MockProblem {
    name: String,
    dimensions: usize,
}

impl OptimizationProblem for MockProblem {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn dimension(&self) -> usize {
        self.dimensions
    }

    fn initial_point(&self) -> Vec<f64> {
        vec![1.0; self.dimensions]
    }

    fn evaluate_f64(&self, _x: &[f64]) -> anyhow::Result<f64> {
        // Return a dummy value based on problem type
        Ok(match self.name.as_str() {
            name if name.starts_with("sphere") => 0.0,
            name if name.starts_with("rosenbrock") => 0.0,
            name if name.starts_with("rastrigin") => 0.0,
            _ => 0.0,
        })
    }

    fn gradient_f64(&self, _x: &[f64]) -> anyhow::Result<Vec<f64>> {
        Ok(vec![0.0; self.dimensions])
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(0.0)
    }

    fn clone_boxed(&self) -> Box<dyn OptimizationProblem> {
        Box::new(MockProblem {
            name: self.name.clone(),
            dimensions: self.dimensions,
        })
    }
}