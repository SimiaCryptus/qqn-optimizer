use crate::utils::math::{tensor_from_vec, tensors_to_vec, DifferentiableFunction};
use anyhow::Result;
use candle_core::Tensor;
/// Trait defining an optimization problem interface
pub trait OptimizationProblem: Send + Sync {
    /// Get the problem name
    fn name(&self) -> &str;
    /// Get the problem dimension
    fn dimension(&self) -> usize;
    /// Get the initial starting point
    fn initial_point(&self) -> Vec<f64>;
    /// Evaluate the objective function at point x
    fn evaluate_f64(&self, x: &[f64]) -> Result<f64>;
    /// Compute the gradient at point x
    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>>;
    /// Get the optimal value if known
    fn optimal_value(&self) -> Option<f64>;
    /// Clone this optimization problem
    fn clone_problem(&self) -> Box<dyn OptimizationProblem>;
}

/// Wrapper to make benchmark functions work with the new DifferentiableFunction trait
pub struct BenchmarkFunctionWrapper<T: OptimizationProblem> {
    problem: T,
}
impl<T: OptimizationProblem> BenchmarkFunctionWrapper<T> {}
impl<T: OptimizationProblem> DifferentiableFunction for BenchmarkFunctionWrapper<T> {
    fn evaluate(&self, params: &[Tensor]) -> candle_core::Result<f64> {
        let x_vec = tensors_to_vec(params);
        self.problem
            .evaluate_f64(&x_vec)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))
    }
    fn gradient(&self, params: &[Tensor]) -> candle_core::Result<Vec<Tensor>> {
        let x_vec = tensors_to_vec(params);
        let grad_vec = self
            .problem
            .gradient_f64(&x_vec)
            .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
        Ok(vec![tensor_from_vec(grad_vec)])
    }
}

// Add to src/benchmarks/functions.rs

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use crate::benchmarks::analytic_functions::{AckleyFunction, BealeFunction, BoothFunction, GoldsteinPriceFunction, GriewankFunction, HimmelblauFunction, LeviFunction, LevyFunction, MatyasFunction, MichalewiczFunction, RastriginFunction, RosenbrockFunction, SchwefelFunction, SphereFunction, StyblinskiTangFunction, ZakharovFunction};

    const EPSILON: f64 = 1e-10;
    const GRADIENT_EPSILON: f64 = 1e-6;

    /// Helper function to test numerical gradient against analytical gradient
    fn test_gradient_numerical(problem: &dyn OptimizationProblem, x: &[f64], tolerance: f64) {
        let analytical_grad = problem.gradient_f64(x).unwrap();
        let mut numerical_grad = vec![0.0; x.len()];

        let h = 1e-8;
        for i in 0..x.len() {
            let mut x_plus = x.to_vec();
            let mut x_minus = x.to_vec();
            x_plus[i] += h;
            x_minus[i] -= h;

            let f_plus = problem.evaluate_f64(&x_plus).unwrap();
            let f_minus = problem.evaluate_f64(&x_minus).unwrap();
            numerical_grad[i] = (f_plus - f_minus) / (2.0 * h);
        }

        for i in 0..analytical_grad.len() {
            assert_relative_eq!(
                analytical_grad[i],
                numerical_grad[i],
                epsilon = tolerance,
                max_relative = tolerance
            );
        }
    }

    #[test]
    fn test_sphere_function() {
        let problem = SphereFunction::new(3);

        // Test at origin (global minimum)
        let origin = vec![0.0, 0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&origin).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_origin = problem.gradient_f64(&origin).unwrap();
        for &g in &grad_origin {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at arbitrary point
        let point = vec![1.0, 2.0, 3.0];
        let expected_value = 1.0 + 4.0 + 9.0; // 14.0
        assert_relative_eq!(
            problem.evaluate_f64(&point).unwrap(),
            expected_value,
            epsilon = EPSILON
        );

        let expected_grad = vec![2.0, 4.0, 6.0];
        let grad = problem.gradient_f64(&point).unwrap();
        for i in 0..grad.len() {
            assert_relative_eq!(grad[i], expected_grad[i], epsilon = EPSILON);
        }

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);

        // Test properties
        assert_eq!(problem.dimension(), 3);
        assert_eq!(problem.name(), "Sphere_3D");
    }

    #[test]
    fn test_rosenbrock_function() {
        let problem = RosenbrockFunction::new(2);

        // Test at global minimum (1, 1)
        let optimum = vec![1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at initial point
        let initial = problem.initial_point();
        let value = problem.evaluate_f64(&initial).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &initial, GRADIENT_EPSILON);

        // Test higher dimension
        let problem_3d = RosenbrockFunction::new(3);
        let optimum_3d = vec![1.0, 1.0, 1.0];
        assert_relative_eq!(
            problem_3d.evaluate_f64(&optimum_3d).unwrap(),
            0.0,
            epsilon = EPSILON
        );
    }

    #[test]
    fn test_rastrigin_function() {
        let problem = RastriginFunction::new(2);

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[ignore]
    #[test]
    fn test_ackley_function() {
        let problem = AckleyFunction::new(2);

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = 1e-10);
        }

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_matyas_function() {
        let problem = MatyasFunction::new();

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test at arbitrary point
        let point = vec![1.0, 2.0];
        let expected_value = 0.26 * (1.0 + 4.0) - 0.48 * 1.0 * 2.0;
        assert_relative_eq!(
            problem.evaluate_f64(&point).unwrap(),
            expected_value,
            epsilon = EPSILON
        );

        let expected_grad = vec![0.52 * 1.0 - 0.48 * 2.0, 0.52 * 2.0 - 0.48 * 1.0];
        let grad = problem.gradient_f64(&point).unwrap();
        for i in 0..grad.len() {
            assert_relative_eq!(grad[i], expected_grad[i], epsilon = EPSILON);
        }

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_levi_function_2d() {
        let problem = LeviFunction::new();

        // Test at global minimum (1, 1)
        let optimum = vec![1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        // Test at arbitrary point
        let point = vec![0.5, 0.5];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 0.0);

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_goldstein_price_function() {
        let problem = GoldsteinPriceFunction::new();

        // Test at global minimum (0, -1)
        let optimum = vec![0.0, -1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            3.0,
            epsilon = 1e-10
        );

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value > 3.0);

        // Test gradient numerically (using numerical gradient due to complexity)
        test_gradient_numerical(&problem, &point, 1e-4);
    }

    #[test]
    fn test_styblinski_tang_function() {
        let problem = StyblinskiTangFunction::new(2);
        // Test gradient numerically
        let point = vec![0.0, 0.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_michalewicz_function() {
        let problem = MichalewiczFunction::new(2);

        // Test at arbitrary point
        let point = vec![1.0, 1.0];
        let value = problem.evaluate_f64(&point).unwrap();
        assert!(value < 0.0); // Michalewicz is negative

        // Test gradient numerically
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_beale_function() {
        let problem = BealeFunction::new();

        // Test at global minimum (3, 0.5)
        let optimum = vec![3.0, 0.5];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = 1e-8);
        }

        // Test gradient numerically
        let point = vec![1.0, 1.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_himmelblau_function() {
        let problem = HimmelblauFunction::new();

        // Test at one of the global minima (3, 2)
        let optimum = vec![3.0, 2.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = 1e-8);
        }

        // Test gradient numerically
        let point = vec![1.0, 1.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_booth_function() {
        let problem = BoothFunction::new();

        // Test at global minimum (1, 3)
        let optimum = vec![1.0, 3.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test gradient numerically
        let point = vec![0.0, 0.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_griewank_function() {
        let problem = GriewankFunction::new(2);

        // Test at global minimum (0, 0)
        let optimum = vec![0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test gradient numerically
        let point = vec![1.0, 1.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_schwefel_function() {
        let problem = SchwefelFunction::new(2);

        // Test at approximate global minimum
        let optimum = vec![420.9687, 420.9687];
        let value = problem.evaluate_f64(&optimum).unwrap();
        assert_relative_eq!(value, 0.0, epsilon = 1e-3);

        // Test gradient numerically
        let point = vec![100.0, 100.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_levy_function_nd() {
        let problem = LevyFunction::new(3);

        // Test at global minimum (1, 1, 1)
        let optimum = vec![1.0, 1.0, 1.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = 1e-10
        );

        // Test gradient numerically
        let point = vec![2.0, 2.0, 2.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_zakharov_function() {
        let problem = ZakharovFunction::new(3);

        // Test at global minimum (0, 0, 0)
        let optimum = vec![0.0, 0.0, 0.0];
        assert_relative_eq!(
            problem.evaluate_f64(&optimum).unwrap(),
            0.0,
            epsilon = EPSILON
        );

        let grad_optimum = problem.gradient_f64(&optimum).unwrap();
        for &g in &grad_optimum {
            assert_relative_eq!(g, 0.0, epsilon = EPSILON);
        }

        // Test gradient numerically
        let point = vec![1.0, 2.0, 3.0];
        test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
    }

    #[test]
    fn test_dimension_mismatch_errors() {
        let problem = SphereFunction::new(3);

        // Test with wrong dimension
        let wrong_dim = vec![1.0, 2.0]; // 2D instead of 3D
        assert!(problem.evaluate_f64(&wrong_dim).is_err());
        assert!(problem.gradient_f64(&wrong_dim).is_err());
    }

    #[test]
    fn test_clone_problem() {
        let problem = SphereFunction::new(3);
        let cloned = problem.clone_problem();

        assert_eq!(cloned.name(), problem.name());
        assert_eq!(cloned.dimension(), problem.dimension());
        assert_eq!(cloned.optimal_value(), problem.optimal_value());

        let point = vec![1.0, 2.0, 3.0];
        assert_relative_eq!(
            cloned.evaluate_f64(&point).unwrap(),
            problem.evaluate_f64(&point).unwrap(),
            epsilon = EPSILON
        );
    }

    #[test]
    fn test_initial_points() {
        let functions: Vec<Box<dyn OptimizationProblem>> = vec![
            Box::new(SphereFunction::new(2)),
            Box::new(RosenbrockFunction::new(2)),
            Box::new(RastriginFunction::new(2)),
            Box::new(AckleyFunction::new(2)),
            Box::new(MatyasFunction::new()),
            Box::new(LeviFunction::new()),
            Box::new(BealeFunction::new()),
            Box::new(HimmelblauFunction::new()),
            Box::new(BoothFunction::new()),
        ];

        for problem in functions {
            let initial = problem.initial_point();
            assert_eq!(initial.len(), problem.dimension());

            // Should be able to evaluate at initial point
            assert!(problem.evaluate_f64(&initial).is_ok());
            assert!(problem.gradient_f64(&initial).is_ok());
        }
    }

    #[test]
    fn test_function_names() {
        assert_eq!(SphereFunction::new(3).name(), "Sphere_3D");
        assert_eq!(RosenbrockFunction::new(5).name(), "Rosenbrock_5D");
        assert_eq!(RastriginFunction::new(10).name(), "Rastrigin_10D");
        assert_eq!(AckleyFunction::new(2).name(), "Ackley_2D_a20_b0.2_c6.283185307179586");
        assert_eq!(MatyasFunction::new().name(), "Matyas_2D");
        assert_eq!(LeviFunction::new().name(), "Levi_2D");
        assert_eq!(BealeFunction::new().name(), "Beale_2D");
        assert_eq!(HimmelblauFunction::new().name(), "Himmelblau_2D");
        assert_eq!(BoothFunction::new().name(), "Booth_2D");
    }

    #[test]
    fn test_gradient_at_multiple_points() {
        let problem = SphereFunction::new(2);
        let test_points = vec![
            vec![0.0, 0.0],
            vec![1.0, 1.0],
            vec![-1.0, 2.0],
            vec![0.5, -0.5],
        ];

        for point in test_points {
            test_gradient_numerical(&problem, &point, GRADIENT_EPSILON);
        }
    }
}