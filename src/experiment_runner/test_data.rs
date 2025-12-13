use crate::benchmarks::evaluation::{
    BenchmarkResults, ConvergenceReason, PerformanceMetrics, ProblemSpec, SingleResult,
};
use crate::OptimizationProblem;
use std::sync::Arc;

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
                timestamp: Default::default(),
                convergence_achieved: false,
                final_value: None,
                function_evaluations: 0,
                gradient_evaluations: 0,
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
                timestamp: Default::default(),
                convergence_achieved: false,
                final_value: None,
                function_evaluations: 0,
                gradient_evaluations: 0,
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
                timestamp: Default::default(),
                convergence_achieved: false,
                final_value: None,
                function_evaluations: 0,
                gradient_evaluations: 0,
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
                timestamp: Default::default(),
                convergence_achieved: false,
                final_value: None,
                function_evaluations: 0,
                gradient_evaluations: 0,
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
                timestamp: Default::default(),
                convergence_achieved: false,
                final_value: None,
                function_evaluations: 0,
                gradient_evaluations: 0,
            },
        ),
    ]
}

fn create_mock_benchmark_result(
    optimizer_name: &str,
    best_value: f32,
    convergence_achieved: bool,
    function_evaluations: u32,
    gradient_evaluations: u32,
) -> SingleResult {
    SingleResult {
        optimizer_name: optimizer_name.to_string(),
        run_id: 0,
        final_value: 0.0,
        best_value,
        final_gradient_norm: 0.0,
        convergence_achieved,
        function_evaluations: function_evaluations.try_into().unwrap(),
        gradient_evaluations: gradient_evaluations.try_into().unwrap(),
        execution_time: std::time::Duration::from_millis(100),
        trace: Default::default(),
        convergence_reason: ConvergenceReason::GradientTolerance,
        memory_usage: None,
        performance_metrics: PerformanceMetrics {
            iterations_per_second: 0.0,
            function_evaluations_per_second: 0.0,
            gradient_evaluations_per_second: 0.0,
            convergence_rate: 0.0,
        },
        problem_name: "mock_problem".to_string(),
        iterations: 0,
        error_message: None,
    }
}

fn create_mock_problem_spec(name: &str) -> ProblemSpec {
    let mock_problem = MockProblem {
        name: name.to_string(),
        dimensions: 2,
    };
    ProblemSpec::new(Arc::new(mock_problem), name.to_string(), Some(2), 42)
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

    fn initial_point(&self) -> Vec<f32> {
        todo!()
    }

    fn evaluate_f64(&self, _x: &[f32]) -> anyhow::Result<f32> {
        todo!()
    }

    fn gradient_f64(&self, _x: &[f32]) -> anyhow::Result<Vec<f32>> {
        todo!()
    }

    fn optimal_value(&self) -> Option<f32> {
        todo!()
    }

    fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
        todo!()
    }
}
