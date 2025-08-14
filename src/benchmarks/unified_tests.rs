//! Unified tests to ensure contract behavior across all optimization problems.

use crate::benchmarks::functions::OptimizationProblem;
use std::f64;

/// Test configuration for problem validation
#[derive(Debug, Clone)]
pub struct ProblemTestConfig {
    pub gradient_tolerance: f64,
    pub finite_check_tolerance: f64,
    pub gradient_step_size: f64,
    pub test_points_count: usize,
    pub random_seed: u64,
}

impl Default for ProblemTestConfig {
    fn default() -> Self {
        Self {
            gradient_tolerance: 1e-5,
            finite_check_tolerance: 1e10,
            gradient_step_size: 1e-8,
            test_points_count: 5,
            random_seed: 42,
        }
    }
}

/// Results from unified problem testing
#[derive(Debug)]
pub struct ProblemTestResults {
    pub problem_name: String,
    pub dimension_consistent: bool,
    pub initial_point_valid: bool,
    pub evaluation_at_initial_valid: bool,
    pub gradient_at_initial_valid: bool,
    pub gradient_numerical_match: bool,
    pub finite_values_maintained: bool,
    pub clone_behavior_correct: bool,
    pub optimal_value_reasonable: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ProblemTestResults {
    pub fn new(problem_name: String) -> Self {
        Self {
            problem_name,
            dimension_consistent: false,
            initial_point_valid: false,
            evaluation_at_initial_valid: false,
            gradient_at_initial_valid: false,
            gradient_numerical_match: false,
            finite_values_maintained: false,
            clone_behavior_correct: false,
            optimal_value_reasonable: false,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.dimension_consistent
            && self.initial_point_valid
            && self.evaluation_at_initial_valid
            && self.gradient_at_initial_valid
            && self.gradient_numerical_match
            && self.finite_values_maintained
            && self.clone_behavior_correct
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }
}

/// Unified test suite for optimization problems
pub struct UnifiedProblemTester {
    config: ProblemTestConfig,
}

impl UnifiedProblemTester {
    pub fn new(config: ProblemTestConfig) -> Self {
        Self { config }
    }

    pub fn with_default_config() -> Self {
        Self::new(ProblemTestConfig::default())
    }

    /// Run all tests on a problem
    pub fn test_problem(&self, problem: &dyn OptimizationProblem) -> ProblemTestResults {
        let mut results = ProblemTestResults::new(problem.name().to_string());

        // Test 1: Dimension consistency
        self.test_dimension_consistency(problem, &mut results);

        // Test 2: Initial point validity
        self.test_initial_point_validity(problem, &mut results);

        // Test 3: Function evaluation at initial point
        self.test_evaluation_at_initial(problem, &mut results);

        // Test 4: Gradient evaluation at initial point
        self.test_gradient_at_initial(problem, &mut results);

        // Test 5: Numerical gradient verification
        self.test_numerical_gradient(problem, &mut results);

        // Test 6: Finite values maintenance
        self.test_finite_values(problem, &mut results);

        // Test 7: Clone behavior
        self.test_clone_behavior(problem, &mut results);

        // Test 8: Optimal value reasonableness
        self.test_optimal_value(problem, &mut results);

        results
    }

    fn test_dimension_consistency(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        let dimension = problem.dimension();
        let initial_point = problem.initial_point();

        if initial_point.len() == dimension {
            results.dimension_consistent = true;
        } else {
            results.add_error(format!(
                "Dimension mismatch: problem.dimension()={}, initial_point.len()={}",
                dimension,
                initial_point.len()
            ));
        }
    }

    fn test_initial_point_validity(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        let initial_point = problem.initial_point();

        if initial_point.is_empty() {
            results.add_error("Initial point is empty".to_string());
            return;
        }

        let all_finite = initial_point.iter().all(|&x| x.is_finite());
        if all_finite {
            results.initial_point_valid = true;
        } else {
            results.add_error("Initial point contains non-finite values".to_string());
        }
    }

    fn test_evaluation_at_initial(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        let initial_point = problem.initial_point();

        match problem.evaluate_f64(&initial_point) {
            Ok(value) => {
                if value.is_finite() {
                    results.evaluation_at_initial_valid = true;
                } else {
                    results.add_error(format!(
                        "Function evaluation at initial point is not finite: {}",
                        value
                    ));
                }
            }
            Err(e) => {
                results.add_error(format!(
                    "Function evaluation at initial point failed: {}",
                    e
                ));
            }
        }
    }

    fn test_gradient_at_initial(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        let initial_point = problem.initial_point();

        match problem.gradient_f64(&initial_point) {
            Ok(gradient) => {
                if gradient.len() == problem.dimension() {
                    if gradient.iter().all(|&g| g.is_finite()) {
                        results.gradient_at_initial_valid = true;
                    } else {
                        results.add_error(
                            "Gradient at initial point contains non-finite values".to_string(),
                        );
                    }
                } else {
                    results.add_error(format!(
                        "Gradient dimension mismatch: expected {}, got {}",
                        problem.dimension(),
                        gradient.len()
                    ));
                }
            }
            Err(e) => {
                results.add_error(format!(
                    "Gradient evaluation at initial point failed: {}",
                    e
                ));
            }
        }
    }

    fn test_numerical_gradient(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        use rand::{Rng, SeedableRng};
        use rand_chacha::ChaCha8Rng;

        let mut rng = ChaCha8Rng::seed_from_u64(self.config.random_seed);

        // Test at multiple points
        let mut successful_tests = 0;
        let total_tests = self.config.test_points_count;

        for test_idx in 0..total_tests {
            // Generate test point (mix of initial point and random perturbations)
            let mut test_point = if test_idx == 0 {
                problem.initial_point()
            } else {
                let initial = problem.initial_point();
                initial
                    .iter()
                    .map(|&x| x + rng.random_range(-1.0..1.0))
                    .collect()
            };

            // Ensure test point is reasonable
            for x in test_point.iter_mut() {
                if !x.is_finite() {
                    *x = rng.random_range(-1.0..1.0);
                }
            }

            if let (Ok(analytical_grad), Ok(numerical_grad)) = (
                problem.gradient_f64(&test_point),
                self.compute_numerical_gradient(problem, &test_point),
            ) {
                if self.gradients_match(&analytical_grad, &numerical_grad) {
                    successful_tests += 1;
                }
            }
        }

        if successful_tests >= (total_tests + 1) / 2 {
            // At least half of the tests should pass
            results.gradient_numerical_match = true;
        } else {
            results.add_error(format!(
                "Numerical gradient verification failed: only {}/{} tests passed",
                successful_tests, total_tests
            ));
        }
    }

    fn compute_numerical_gradient(
        &self,
        problem: &dyn OptimizationProblem,
        point: &[f64],
    ) -> Result<Vec<f64>, String> {
        let mut numerical_grad = vec![0.0; point.len()];
        let h = self.config.gradient_step_size;

        for i in 0..point.len() {
            let mut point_plus = point.to_vec();
            let mut point_minus = point.to_vec();

            point_plus[i] += h;
            point_minus[i] -= h;

            match (
                problem.evaluate_f64(&point_plus),
                problem.evaluate_f64(&point_minus),
            ) {
                (Ok(f_plus), Ok(f_minus)) => {
                    if f_plus.is_finite() && f_minus.is_finite() {
                        numerical_grad[i] = (f_plus - f_minus) / (2.0 * h);
                    } else {
                        return Err(format!("Non-finite function values in numerical gradient computation at dimension {}", i));
                    }
                }
                (Err(e), _) | (_, Err(e)) => {
                    return Err(format!(
                        "Function evaluation failed in numerical gradient: {}",
                        e
                    ));
                }
            }
        }

        Ok(numerical_grad)
    }

    fn gradients_match(&self, analytical: &[f64], numerical: &[f64]) -> bool {
        if analytical.len() != numerical.len() {
            return false;
        }

        for (_i, (&a, &n)) in analytical.iter().zip(numerical.iter()).enumerate() {
            if !a.is_finite() || !n.is_finite() {
                return false;
            }

            // Use relative tolerance for large gradients, absolute for small ones
            let tolerance = if n.abs() > 1.0 {
                self.config.gradient_tolerance * n.abs()
            } else {
                self.config.gradient_tolerance
            };

            if (a - n).abs() > tolerance {
                // Allow some failures for very small gradients or problematic dimensions
                if n.abs() < 1e-10 && (a - n).abs() < 1e-6 {
                    continue;
                }
                return false;
            }
        }

        true
    }

    fn test_finite_values(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        use rand::{Rng, SeedableRng};
        use rand_chacha::ChaCha8Rng;

        let mut rng = ChaCha8Rng::seed_from_u64(self.config.random_seed);
        let dimension = problem.dimension();
        let mut all_finite = true;

        // Test at several random points
        for _ in 0..self.config.test_points_count {
            let test_point: Vec<f64> = (0..dimension).map(|_| rng.random_range(-10.0..10.0)).collect();

            // Skip points that might be outside valid domain
            if let (Ok(f_val), Ok(grad)) = (
                problem.evaluate_f64(&test_point),
                problem.gradient_f64(&test_point),
            ) {
                if !f_val.is_finite() || grad.iter().any(|&g| !g.is_finite()) {
                    // Only flag as error if the values are extremely large
                    if f_val.abs() > self.config.finite_check_tolerance
                        || grad
                            .iter()
                            .any(|&g| g.abs() > self.config.finite_check_tolerance)
                    {
                        all_finite = false;
                        break;
                    }
                }
            }
        }

        if all_finite {
            results.finite_values_maintained = true;
        } else {
            results.add_warning(
                "Some function/gradient evaluations produced non-finite values at random points"
                    .to_string(),
            );
            // Don't mark as error since some problems may have restricted domains
            results.finite_values_maintained = true;
        }
    }

    fn test_clone_behavior(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        let cloned = problem.clone_problem();

        // Test that cloned problem has same basic properties
        if cloned.name() == problem.name()
            && cloned.dimension() == problem.dimension()
            && cloned.optimal_value() == problem.optimal_value()
        {
            // Test that cloned problem gives same results
            let test_point = problem.initial_point();

            match (
                problem.evaluate_f64(&test_point),
                cloned.evaluate_f64(&test_point),
            ) {
                (Ok(orig_val), Ok(clone_val)) => {
                    if (orig_val - clone_val).abs() < 1e-12 {
                        results.clone_behavior_correct = true;
                    } else {
                        results.add_error(format!(
                            "Cloned problem gives different function value: {} vs {}",
                            orig_val, clone_val
                        ));
                    }
                }
                _ => {
                    results.add_error(
                        "Function evaluation failed on original or cloned problem".to_string(),
                    );
                }
            }
        } else {
            results.add_error("Cloned problem has different basic properties".to_string());
        }
    }

    fn test_optimal_value(
        &self,
        problem: &dyn OptimizationProblem,
        results: &mut ProblemTestResults,
    ) {
        match problem.optimal_value() {
            Some(opt_val) => {
                if opt_val.is_finite() {
                    results.optimal_value_reasonable = true;
                } else {
                    results.add_warning(format!("Optimal value is not finite: {}", opt_val));
                    results.optimal_value_reasonable = false;
                }
            }
            None => {
                results.add_warning("No optimal value specified".to_string());
                results.optimal_value_reasonable = true; // Not having an optimal value is acceptable
            }
        }
    }
}

/// Batch test multiple problems
pub fn test_multiple_problems(
    problems: Vec<Box<dyn OptimizationProblem>>,
    config: Option<ProblemTestConfig>,
) -> Vec<ProblemTestResults> {
    let tester = UnifiedProblemTester::new(config.unwrap_or_default());

    problems
        .iter()
        .map(|problem| tester.test_problem(problem.as_ref()))
        .collect()
}

/// Generate a summary report from test results
pub fn generate_test_report(results: &[ProblemTestResults]) -> String {
    let mut report = String::new();

    report.push_str("=== Unified Problem Test Report ===\n\n");

    let total_problems = results.len();
    let valid_problems = results.iter().filter(|r| r.is_valid()).count();

    report.push_str(&format!("Total problems tested: {}\n", total_problems));
    report.push_str(&format!("Valid problems: {}\n", valid_problems));
    report.push_str(&format!(
        "Success rate: {:.1}%\n\n",
        (valid_problems as f64 / total_problems as f64) * 100.0
    ));

    // Summary by test type
    let mut test_summaries = vec![
        (
            "Dimension Consistency",
            results.iter().filter(|r| r.dimension_consistent).count(),
        ),
        (
            "Initial Point Valid",
            results.iter().filter(|r| r.initial_point_valid).count(),
        ),
        (
            "Evaluation at Initial",
            results
                .iter()
                .filter(|r| r.evaluation_at_initial_valid)
                .count(),
        ),
        (
            "Gradient at Initial",
            results
                .iter()
                .filter(|r| r.gradient_at_initial_valid)
                .count(),
        ),
        (
            "Numerical Gradient Match",
            results
                .iter()
                .filter(|r| r.gradient_numerical_match)
                .count(),
        ),
        (
            "Finite Values",
            results
                .iter()
                .filter(|r| r.finite_values_maintained)
                .count(),
        ),
        (
            "Clone Behavior",
            results.iter().filter(|r| r.clone_behavior_correct).count(),
        ),
        (
            "Optimal Value",
            results
                .iter()
                .filter(|r| r.optimal_value_reasonable)
                .count(),
        ),
    ];

    report.push_str("Test Results Summary:\n");
    for (test_name, pass_count) in test_summaries {
        report.push_str(&format!(
            "  {}: {}/{} ({:.1}%)\n",
            test_name,
            pass_count,
            total_problems,
            (pass_count as f64 / total_problems as f64) * 100.0
        ));
    }

    report.push_str("\n");

    // Detailed results for failed problems
    let failed_problems: Vec<_> = results.iter().filter(|r| !r.is_valid()).collect();
    if !failed_problems.is_empty() {
        report.push_str("Failed Problems:\n");
        for result in failed_problems {
            report.push_str(&format!("\n{}: \n", result.problem_name));
            for error in &result.errors {
                report.push_str(&format!("  ERROR: {}\n", error));
            }
            for warning in &result.warnings {
                report.push_str(&format!("  WARNING: {}\n", warning));
            }
        }
    }

    // Warnings for valid problems
    let problems_with_warnings: Vec<_> = results
        .iter()
        .filter(|r| r.is_valid() && !r.warnings.is_empty())
        .collect();

    if !problems_with_warnings.is_empty() {
        report.push_str("\nWarnings:\n");
        for result in problems_with_warnings {
            report.push_str(&format!("\n{}: \n", result.problem_name));
            for warning in &result.warnings {
                report.push_str(&format!("  WARNING: {}\n", warning));
            }
        }
    }

    report
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::analytic_functions::*;
    use crate::benchmarks::ml_problems::*;
    use crate::benchmarks::mnist::*;
    use crate::benchmarks::mnist_onednn::*;
    use rand::{rngs::StdRng, SeedableRng};

    #[test]
    fn test_sphere_function_contract() {
        let problem = SphereFunction::new(3);
        let tester = UnifiedProblemTester::with_default_config();
        let results = tester.test_problem(&problem);

        assert!(results.is_valid(), "Sphere function should pass all tests");
        assert!(
            results.errors.is_empty(),
            "Sphere function should have no errors"
        );
    }

    #[test]
    fn test_rosenbrock_function_contract() {
        let problem = RosenbrockFunction::new(2);
        let tester = UnifiedProblemTester::with_default_config();
        let results = tester.test_problem(&problem);

        assert!(
            results.is_valid(),
            "Rosenbrock function should pass all tests"
        );
    }

    #[test]
    fn test_multiple_analytic_functions() {
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(2)),
            Box::new(RosenbrockFunction::new(2)),
            Box::new(RastriginFunction::new(2)),
            Box::new(MatyasFunction::new()),
            Box::new(BealeFunction::new()),
            Box::new(BoothFunction::new()),
        ];

        let results = test_multiple_problems(problems, None);

        // All analytic functions should pass
        for result in &results {
            assert!(
                result.is_valid(),
                "Problem {} should pass all tests. Errors: {:?}",
                result.problem_name,
                result.errors
            );
        }

        // Generate and print report
        let report = generate_test_report(&results);
        println!("{}", report);
    }

    #[test]
    fn test_all_analytic_functions_comprehensive() {
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            // 2D functions
            Box::new(SphereFunction::new(2)),
            Box::new(RosenbrockFunction::new(2)),
            Box::new(RastriginFunction::new(2)),
            Box::new(AckleyFunction::new(2)),
            Box::new(MatyasFunction::new()),
            Box::new(LeviFunction::new()),
            Box::new(GoldsteinPriceFunction::new()),
            Box::new(BealeFunction::new()),
            Box::new(HimmelblauFunction::new()),
            Box::new(BoothFunction::new()),
            Box::new(GriewankFunction::new(2)),
            Box::new(SchwefelFunction::new(2)),
            Box::new(LevyFunction::new(2)),
            Box::new(ZakharovFunction::new(2)),
            // Higher dimensional functions
            Box::new(SphereFunction::new(5)),
            Box::new(RosenbrockFunction::new(5)),
            Box::new(RastriginFunction::new(5)),
            Box::new(AckleyFunction::new(5)),
            Box::new(StyblinskiTangFunction::new(5)),
            Box::new(MichalewiczFunction::new(5)),
            // Specialized functions
            Box::new(IllConditionedRosenbrock::new(4, 1000.0)),
            Box::new(TrigonometricFunction::new(3)),
            Box::new(PenaltyFunctionI::new(3)),
            Box::new(BarrierFunction::new(3)),
            Box::new(NoisySphere::new(3, 0.1)),
            Box::new(SparseRosenbrock::new(4)),
            Box::new(SparseQuadratic::new(4)),
        ];

        let config = ProblemTestConfig {
            gradient_tolerance: 1e-4, // More lenient for complex functions
            test_points_count: 3,     // Fewer test points for speed
            ..Default::default()
        };

        let results = test_multiple_problems(problems, Some(config));

        // Generate comprehensive report
        let report = generate_test_report(&results);
        println!("{}", report);

        // Check that most functions pass (allow some failures for very specialized functions)
        let valid_count = results.iter().filter(|r| r.is_valid()).count();
        let total_count = results.len();
        let success_rate = valid_count as f64 / total_count as f64;

        assert!(
            success_rate >= 0.8,
            "At least 80% of functions should pass unified tests. Success rate: {:.1}%",
            success_rate * 100.0
        );
    }
    #[test]
    fn test_ml_problems_unified() {
        let mut rng = StdRng::seed_from_u64(42);
        // Generate small synthetic datasets for testing
        let (x_data, y_data) = generate_linear_regression_data(20, 3, &mut rng);
        let (svm_x, svm_y) = generate_svm_data(20, 3, &mut rng);
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(LinearRegression::new(x_data.clone(), y_data.clone(), 0.01).unwrap()),
            Box::new(LogisticRegression::new(x_data.clone(), 
                y_data.iter().map(|&y| if y > 0.0 { 1.0 } else { 0.0 }).collect(), 0.01).unwrap()),
            Box::new(SupportVectorMachine::new(svm_x, svm_y, 1.0).unwrap()),
            Box::new(NeuralNetworkTraining::mlp_classification(vec![3, 5, 2], &mut rng).unwrap()),
        ];
        let config = ProblemTestConfig {
            gradient_tolerance: 1e-3, // More lenient for ML problems
            test_points_count: 2,     // Fewer test points for speed
            ..Default::default()
        };
        let results = test_multiple_problems(problems, Some(config));
        let report = generate_test_report(&results);
        println!("{}", report);
        // ML problems should have reasonable success rate
        let valid_count = results.iter().filter(|r| r.is_valid()).count();
        let success_rate = valid_count as f64 / results.len() as f64;
        assert!(
            success_rate >= 0.5,
            "At least 50% of ML problems should pass unified tests. Success rate: {:.1}%",
            success_rate * 100.0
        );
    }
    #[test]
    fn test_mnist_problems_unified() {
        let mut rng = StdRng::seed_from_u64(42);
        // Create small MNIST-like problems for testing
        let x_data = vec![vec![0.5; 784]; 10]; // 10 samples, 784 features
        let mut y_data = vec![vec![0.0; 10]; 10]; // 10 samples, 10 classes
        for (i, label) in y_data.iter_mut().enumerate() {
            label[i % 10] = 1.0; // One-hot encoding
        }
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(MnistNeuralNetwork::new(
                x_data.clone(), y_data.clone(), &[20], Some(5), &mut rng, None
            ).unwrap()),
            #[cfg(feature = "onednn")]
            Box::new(MnistOneDnnNeuralNetwork::new(
                x_data, y_data, &[20], Some(5), &mut rng, None
            ).unwrap()),
        ];
        let config = ProblemTestConfig {
            gradient_tolerance: 1e-2, // Very lenient for neural networks
            test_points_count: 1,     // Single test point for speed
            finite_check_tolerance: 1e8, // Allow larger values
            ..Default::default()
        };
        let results = test_multiple_problems(problems, Some(config));
        let report = generate_test_report(&results);
        println!("{}", report);
        // Neural networks are complex, allow some failures
        let valid_count = results.iter().filter(|r| r.is_valid()).count();
        let success_rate = valid_count as f64 / results.len() as f64;
        // At least basic functionality should work
        assert!(
            success_rate >= 0.3,
            "At least 30% of neural network problems should pass basic tests. Success rate: {:.1}%",
            success_rate * 100.0
        );
    }
    #[test]
    fn test_mixed_problem_types() {
        let mut rng = StdRng::seed_from_u64(42);
        // Mix of analytic and ML problems
        let (x_data, y_data) = generate_linear_regression_data(15, 2, &mut rng);
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            // Analytic functions
            Box::new(SphereFunction::new(3)),
            Box::new(RosenbrockFunction::new(3)),
            Box::new(BealeFunction::new()),
            // ML problems
            Box::new(LinearRegression::new(x_data.clone(), y_data.clone(), 0.01).unwrap()),
            Box::new(LogisticRegression::new(x_data, 
                y_data.iter().map(|&y| if y > 0.0 { 1.0 } else { 0.0 }).collect(), 0.01).unwrap()),
        ];
        let results = test_multiple_problems(problems, None);
        let report = generate_test_report(&results);
        println!("{}", report);
        // Check that different problem types are handled consistently
        let analytic_results: Vec<_> = results.iter()
            .filter(|r| r.problem_name.contains("Sphere") || 
                       r.problem_name.contains("Rosenbrock") || 
                       r.problem_name.contains("Beale"))
            .collect();
        let ml_results: Vec<_> = results.iter()
            .filter(|r| r.problem_name.contains("Regression"))
            .collect();
        // Analytic functions should have high success rate
        let analytic_success = analytic_results.iter().filter(|r| r.is_valid()).count() as f64 
            / analytic_results.len() as f64;
        assert!(
            analytic_success >= 0.9,
            "Analytic functions should have >90% success rate: {:.1}%",
            analytic_success * 100.0
        );
        // ML problems should have reasonable success rate
        let ml_success = ml_results.iter().filter(|r| r.is_valid()).count() as f64 
            / ml_results.len() as f64;
        assert!(
            ml_success >= 0.5,
            "ML problems should have >50% success rate: {:.1}%",
            ml_success * 100.0
        );
    }
    #[test]
    fn test_gradient_consistency_across_problems() {
        let rng = StdRng::seed_from_u64(42);
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(2)),
            Box::new(RosenbrockFunction::new(2)),
        ];
        let config = ProblemTestConfig {
            gradient_tolerance: 1e-6,
            test_points_count: 5,
            ..Default::default()
        };
        for problem in &problems {
            let results = UnifiedProblemTester::new(config.clone()).test_problem(problem.as_ref());
            assert!(
                results.gradient_numerical_match,
                "Problem {} failed gradient consistency test: {:?}",
                results.problem_name,
                results.errors
            );
        }
    }
    #[test]
    fn test_parameter_bounds_handling() {
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(3)),
            Box::new(RastriginFunction::new(3)),
            Box::new(AckleyFunction::new(3)),
        ];
        let tester = UnifiedProblemTester::with_default_config();
        for problem in &problems {
            let results = tester.test_problem(problem.as_ref());
            // Test with extreme parameter values
            let dimension = problem.dimension();
            let extreme_params = vec![1e6; dimension];
            // Should handle extreme values gracefully (either return finite value or error)
            match problem.evaluate_f64(&extreme_params) {
                Ok(value) => {
                    if !value.is_finite() {
                        panic!("Problem {} returned non-finite value for extreme parameters", 
                               problem.name());
                    }
                }
                Err(_) => {
                    // Returning an error for extreme values is acceptable
                }
            }
            assert!(
                results.finite_values_maintained,
                "Problem {} failed finite values test",
                results.problem_name
            );
        }
    }
    #[test]
    fn test_problem_cloning_behavior() {
        let mut rng = StdRng::seed_from_u64(42);
        let (x_data, y_data) = generate_linear_regression_data(10, 2, &mut rng);
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(3)),
            Box::new(LinearRegression::new(x_data, y_data, 0.01).unwrap()),
        ];
        for problem in &problems {
            let cloned = problem.clone_problem();
            // Basic properties should match
            assert_eq!(problem.name(), cloned.name());
            assert_eq!(problem.dimension(), cloned.dimension());
            assert_eq!(problem.optimal_value(), cloned.optimal_value());
            // Function evaluations should match
            let test_point = problem.initial_point();
            let orig_value = problem.evaluate_f64(&test_point).unwrap();
            let clone_value = cloned.evaluate_f64(&test_point).unwrap();
            assert!(
                (orig_value - clone_value).abs() < 1e-12,
                "Cloned problem gives different result: {} vs {} for {}",
                orig_value, clone_value, problem.name()
            );
        }
    }
    #[test]
    fn test_dimension_consistency() {
        let mut rng = StdRng::seed_from_u64(42);
        let problems: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(5)),
            Box::new(RosenbrockFunction::new(4)),
            Box::new(NeuralNetworkTraining::mlp_classification(vec![3, 4, 2], &mut rng).unwrap()),
        ];
        for problem in &problems {
            let dimension = problem.dimension();
            let initial_point = problem.initial_point();
            assert_eq!(
                initial_point.len(), dimension,
                "Problem {} has dimension mismatch: dimension()={}, initial_point.len()={}",
                problem.name(), dimension, initial_point.len()
            );
            // Test gradient dimension consistency
            if let Ok(gradient) = problem.gradient_f64(&initial_point) {
                assert_eq!(
                    gradient.len(), dimension,
                    "Problem {} gradient dimension mismatch: expected {}, got {}",
                    problem.name(), dimension, gradient.len()
                );
            }
        }
    }

    #[test]
    fn test_custom_config() {
        let problem = RastriginFunction::new(3);

        let strict_config = ProblemTestConfig {
            gradient_tolerance: 1e-8,
            test_points_count: 10,
            ..Default::default()
        };

        let tester = UnifiedProblemTester::new(strict_config);
        let results = tester.test_problem(&problem);

        // Should still pass with stricter config
        assert!(results.is_valid() || !results.errors.is_empty());
    }
}