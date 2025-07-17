//! Example demonstrating how to implement a custom optimization problem.
//!
//! This example shows:
//! - Implementing the OptimizationProblem trait
//! - Creating a custom quadratic function
//! - Using it with different optimizers
//! - Comparing performance

use anyhow::Result;
use candle_core::{Device, Tensor};
use qqn_optimizer::{
    LBFGSConfig, LBFGSOptimizer, OptimizationProblem, Optimizer,
    QQNConfig, QQNOptimizer,
};
use qqn_optimizer::utils::math::DifferentiableFunction;

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

    fn evaluate_f64(&self, x: &[f64]) -> Result<f64> {
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

    fn gradient_f64(&self, x: &[f64]) -> Result<Vec<f64>> {
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
   fn clone_problem(&self) -> Box<dyn OptimizationProblem> {
       Box::new(QuadraticProblem {
           name: self.name.clone(),
           dimension: self.dimension,
           matrix_a: self.matrix_a.clone(),
           vector_b: self.vector_b.clone(),
           constant_c: self.constant_c,
           optimal_point: self.optimal_point.clone(),
           optimal_value: self.optimal_value,
       })
   }
}
impl DifferentiableFunction for QuadraticProblem {
    fn evaluate(&self, params: &[Tensor]) -> candle_core::Result<f64> {
        // Convert tensors to f64 vector
        let x: Result<Vec<f64>, _> = params.iter()
            .map(|t| t.to_scalar::<f64>())
            .collect();
        let x = x?;
        // Evaluate using f64 implementation
        let result = self.evaluate_f64(&x)
            .map_err(|e| candle_core::Error::Msg(format!("Evaluation error: {}", e)))?;
        Ok(result)
    }
    fn gradient(&self, params: &[Tensor]) -> candle_core::Result<Vec<Tensor>> {
        // Convert tensors to f64 vector
        let x: Result<Vec<f64>, _> = params.iter()
            .map(|t| t.to_scalar::<f64>())
            .collect();
        let x = x?;
        // Compute gradient using f64 implementation
        let grad = self.gradient_f64(&x)
            .map_err(|e| candle_core::Error::Msg(format!("Gradient error: {}", e)))?;
        // Convert back to tensors
        grad.iter()
            .map(|&g| Tensor::from_slice(&[g], (1,), &Device::Cpu))
            .collect()
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
         "QQN",
     )?;
     // Test with L-BFGS optimizer
     println!("\n--- L-BFGS Optimizer ---");
     let lbfgs_result = run_optimizer(
         &problem,
         Box::new(LBFGSOptimizer::new(LBFGSConfig::default())),
         "L-BFGS",
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
     mut optimizer: Box<dyn Optimizer>,
     name: &str,
 ) -> Result<(usize, f64)> {
     let initial_point = problem.initial_point();
     let device = Device::Cpu;
     // Convert initial point to tensors
     let mut params: Vec<Tensor> = initial_point.iter()
         .map(|&val| Tensor::from_slice(&[val], (1,), &device))
         .collect::<candle_core::Result<Vec<_>>>()
         .map_err(|e| anyhow::anyhow!("Failed to create tensors: {}", e))?;
     let mut iteration = 0;
     let max_iterations = 1000;
     println!("Starting {} optimization...", name);
     while iteration < max_iterations {
         // Convert tensors back to f64 for convergence checking
         let x: Vec<f64> = params.iter()
             .map(|t| t.to_scalar::<f64>())
             .collect::<candle_core::Result<Vec<_>>>()
             .map_err(|e| anyhow::anyhow!("Failed to extract values: {}", e))?;
         let gradient = problem.gradient_f64(&x)?;
         let grad_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
         // Check convergence
         if grad_norm < problem.convergence_tolerance() {
             println!("Converged in {} iterations", iteration);
             break;
         }
         // Perform optimization step
         let _step_result = optimizer.step(&mut params, problem)
             .map_err(|e| anyhow::anyhow!("Optimizer step failed: {}", e))?;
         iteration += 1;
         // Print progress occasionally
         if iteration % 50 == 0 {
             let x: Vec<f64> = params.iter()
                 .map(|t| t.to_scalar::<f64>())
                 .collect::<candle_core::Result<Vec<_>>>()
                 .map_err(|e| anyhow::anyhow!("Failed to extract values: {}", e))?;
             let f_val = problem.evaluate_f64(&x)?;
             println!("  Iteration {}: f = {:.6}, ||∇f|| = {:.2e}", iteration, f_val, grad_norm);
         }
     }
     // Convert final parameters back to f64 for evaluation
     let final_x: Vec<f64> = params.iter()
         .map(|t| t.to_scalar::<f64>())
         .collect::<candle_core::Result<Vec<_>>>()
         .map_err(|e| anyhow::anyhow!("Failed to extract final values: {}", e))?;
     let final_value = problem.evaluate_f64(&final_x)?;
     Ok((iteration, final_value))
 }