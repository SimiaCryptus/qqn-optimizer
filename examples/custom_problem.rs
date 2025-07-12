//! Example demonstrating how to implement a custom optimization problem.
//!
//! This example shows:
//! - Implementing the OptimizationProblem trait
//! - Creating a custom quadratic function
//! - Using it with different optimizers
//! - Comparing performance

use anyhow::Result;
use qqn_optimizer::{
    LBFGSConfig, LBFGSOptimizer, OptimizationProblem, Optimizer, OptimizerBox,
    QQNConfig, QQNOptimizer
};

/// Custom quadratic optimization problem: f(x) = 0.5 * x^T * A * x + b^T * x + c
/// where A is a positive definite matrix, b is a vector, and c is a scalar.
pub struct QuadraticProblem {
    name: String,
    dimension: usize,
    matrix_a: Vec<Vec<f64>>,  // Positive definite matrix
    vector_b: Vec<f64>,       // Linear term
    constant_c: f64,          // Constant term
    optimal_point: Vec<f64>,  // Known optimal point: x* = -A^(-1) * b
    optimal_value: f64,       // Known optimal value
}

impl QuadraticProblem {
    /// Create a new quadratic problem with specified condition number
    pub fn new(dimension: usize, condition_number: f64) -> Self {
        // Create a positive definite matrix with specified condition number
        let mut matrix_a = vec![vec![0.0; dimension]; dimension];
        
        // Create diagonal matrix with eigenvalues from 1 to condition_number
        for i in 0..dimension {
            let eigenvalue = 1.0 + (condition_number - 1.0) * (i as f64) / ((dimension - 1) as f64);
            matrix_a[i][i] = eigenvalue;
        }

        // Create a random linear term
        let vector_b: Vec<f64> = (0..dimension)
            .map(|i| (i as f64 + 1.0) * 0.1)
            .collect();

        let constant_c = 5.0;

        // Compute optimal point: x* = -A^(-1) * b
        // For diagonal A, this is simple: x*[i] = -b[i] / A[i][i]
        let optimal_point: Vec<f64> = vector_b.iter()
            .enumerate()
            .map(|(i, &bi)| -bi / matrix_a[i][i])
            .collect();

        // Compute optimal value
        let mut optimal_value = constant_c;
        for i in 0..dimension {
            optimal_value += vector_b[i] * optimal_point[i];
            optimal_value += 0.5 * matrix_a[i][i] * optimal_point[i] * optimal_point[i];
        }

        Self {
            name: format!("Quadratic{}D_Cond{:.1}", dimension, condition_number),
            dimension,
            matrix_a,
            vector_b,
            constant_c,
            optimal_point,
            optimal_value,
        }
    }
}

impl OptimizationProblem for QuadraticProblem {
    fn name(&self) -> &str {
        &self.name
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn initial_point(&self) -> Vec<f64> {
        // Start at origin
        vec![0.0; self.dimension]
    }

    fn evaluate(&self, x: &[f64]) -> Result<f64> {
        let mut result = self.constant_c;

        // Add linear term: b^T * x
        for i in 0..self.dimension {
            result += self.vector_b[i] * x[i];
        }

        // Add quadratic term: 0.5 * x^T * A * x
        for i in 0..self.dimension {
            for j in 0..self.dimension {
                result += 0.5 * x[i] * self.matrix_a[i][j] * x[j];
            }
        }

        Ok(result)
    }

    fn gradient(&self, x: &[f64]) -> Result<Vec<f64>> {
        let mut grad = vec![0.0; self.dimension];

        // Gradient: ∇f(x) = A * x + b
        for i in 0..self.dimension {
            grad[i] = self.vector_b[i];
            for j in 0..self.dimension {
                grad[i] += self.matrix_a[i][j] * x[j];
            }
        }

        Ok(grad)
    }

    fn optimal_value(&self) -> Option<f64> {
        Some(self.optimal_value)
    }

    fn convergence_tolerance(&self) -> f64 {
        1e-8
    }
    fn bounds(&self) -> Option<(Vec<f64>, Vec<f64>)> {
        // No bounds for this quadratic problem - allow unconstrained optimization
        None
    }
}

fn main() -> Result<()> {
    println!("Custom Optimization Problem Example");
    println!("===================================");

    // Create a moderately ill-conditioned quadratic problem
    let problem = QuadraticProblem::new(10, 100.0);

    println!("Problem: {}", problem.name());
    println!("Dimension: {}", problem.dimension());
    println!("Optimal value: {:.6}", problem.optimal_value().unwrap());
    println!("Optimal point: {:?}", problem.optimal_point);

    // Test with QQN optimizer
    println!("\n--- QQN Optimizer ---");
    let qqn_result = run_optimizer(
        &problem,
        Box::new(QQNOptimizer::new(QQNConfig::default())),
        "QQN"
    )?;

    // Test with L-BFGS optimizer
    println!("\n--- L-BFGS Optimizer ---");
    let lbfgs_result = run_optimizer(
        &problem,
        Box::new(LBFGSOptimizer::new(LBFGSConfig::default())),
        "L-BFGS"
    )?;

    // Compare results
    println!("\n--- Comparison ---");
    println!("QQN:    {} iterations, final value: {:.6}", qqn_result.0, qqn_result.1);
    println!("L-BFGS: {} iterations, final value: {:.6}", lbfgs_result.0, lbfgs_result.1);

    let qqn_error = (qqn_result.1 - problem.optimal_value().unwrap()).abs();
    let lbfgs_error = (lbfgs_result.1 - problem.optimal_value().unwrap()).abs();

    println!("QQN error:    {:.2e}", qqn_error);
    println!("L-BFGS error: {:.2e}", lbfgs_error);

    if qqn_result.0 < lbfgs_result.0 {
        println!("✓ QQN converged faster!");
    } else if qqn_result.0 == lbfgs_result.0 {
        println!("= Both optimizers converged in the same number of iterations");
    } else {
        println!("⚠ L-BFGS converged faster");
    }

    Ok(())
}

fn run_optimizer(
    problem: &QuadraticProblem,
    mut optimizer: Box<dyn OptimizerBox>,
    name: &str
) -> Result<(usize, f64)> {
    let mut x = problem.initial_point();
    let mut iteration = 0;
    let max_iterations = 1000;

    println!("Starting {} optimization...", name);

    while iteration < max_iterations {
        let gradient = problem.gradient(&x)?;
        let grad_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();

        // Check convergence
        if grad_norm < problem.convergence_tolerance() {
            println!("Converged in {} iterations", iteration);
            break;
        }

        // Perform optimization step
        let _step_result = optimizer.step_slice(&mut x, &gradient)
            .map_err(|e| anyhow::anyhow!("Optimizer step failed: {}", e))?;

        iteration += 1;

        // Print progress occasionally
        if iteration % 50 == 0 {
            let f_val = problem.evaluate(&x)?;
            println!("  Iteration {}: f = {:.6}, ||∇f|| = {:.2e}", iteration, f_val, grad_norm);
        }
    }

    let final_value = problem.evaluate(&x)?;
    Ok((iteration, final_value))
}