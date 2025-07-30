//! Basic usage example demonstrating QQN optimization on the Rosenbrock function.
//!
//! This example shows how to:
//! - Create and configure a QQN optimizer
//! - Define an optimization problem
//! - Run the optimization loop
//! - Analyze the results

use anyhow::Result;
use candle_core::{Device, Tensor};
use qqn_optimizer::benchmarks::analytic_functions::RosenbrockFunction;
use qqn_optimizer::line_search::{LineSearchConfig, LineSearchMethod};
use qqn_optimizer::utils::math::SeparateFunctions;
use qqn_optimizer::{OptimizationProblem, Optimizer, QQNConfig, QQNOptimizer};
use std::sync::Arc;

fn main() -> Result<()> {
    // Configure the QQN optimizer
    let config = QQNConfig {
        lbfgs_history: 10, // L-BFGS history length
        min_lbfgs_iterations: 2,
        line_search: LineSearchConfig {
            method: LineSearchMethod::StrongWolfe,
            c1: 1e-4,
            c2: 0.9,
            max_iterations: 20,
            initial_step: 1.0,
            min_step: 1e-16,
            max_step: 1e16,
            verbose: false,         // Enable verbose output for line search
            line_bracket_method: 1, // 1: gradient-based bracketing, 2: function-value-based bracketing
        },
        epsilon: 1e-8,  // Numerical stability constant
        verbose: false, // Enable verbose output
        min_step_persist: 0.0,
        min_step_size: 0.0,
        gradient_scale_factor: 1.0,
    };

    let mut optimizer = QQNOptimizer::new(config);

    // Define the optimization problem (2D Rosenbrock function)
    let problem = Arc::new(RosenbrockFunction::new(2));
    let mut initial_point = problem.initial_point(); // Random initial point in 2D
    let device = Device::Cpu;

    println!("Starting optimization of 2D Rosenbrock function");
    println!("Initial point: {:?}", initial_point);
    println!(
        "Initial value: {:.6}",
        problem.evaluate_f64(&initial_point)?
    );

    // Optimization loop
    let mut iteration = 0;
    let max_iterations = 1000;

    while iteration < max_iterations {
        // Compute gradient
        let gradient = problem.gradient_f64(&initial_point)?;
        let grad_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();

        // Print progress
        if iteration % 10 == 0 {
            let f_val = problem.evaluate_f64(&initial_point)?;
            println!(
                "Iteration {}: f = {:.6}, ||∇f|| = {:.6}",
                iteration, f_val, grad_norm
            );
        }

        // Check convergence
        if grad_norm < 1e-6 {
            println!("Converged! Gradient norm: {:.2e}", grad_norm);
            break;
        }

        // Create a function object that implements both objective and gradient computation
        let function = Arc::new(SeparateFunctions::new(
            {
                let problem = problem.clone();
                move |params: &[Tensor]| -> candle_core::Result<f64> {
                    let x_vec = params[0].to_vec1::<f64>()?;
                    problem
                        .evaluate_f64(&x_vec)
                        .map_err(|e| candle_core::Error::Msg(e.to_string()))
                }
            },
            {
                let problem = problem.clone();
                let device = device.clone();
                move |params: &[Tensor]| -> candle_core::Result<Vec<Tensor>> {
                    let x_vec = params[0].to_vec1::<f64>()?;
                    let grad = problem
                        .gradient_f64(&x_vec)
                        .map_err(|e| candle_core::Error::Msg(e.to_string()))?;
                    Ok(vec![Tensor::from_slice(&grad, grad.len(), &device)
                        .map_err(|e| candle_core::Error::Msg(e.to_string()))?])
                }
            },
        ));

        // Convert Vec<f64> to Tensor for optimizer
        let mut x_tensor = vec![Tensor::from_slice(
            &initial_point,
            initial_point.len(),
            &device,
        )?];

        // Perform optimization step
        let _step_result = optimizer.step(&mut x_tensor, function.clone())?;

        // Convert result back to Vec<f64>
        initial_point = x_tensor[0].to_vec1::<f64>()?;

        // Print step information
        if iteration % 50 == 0 {
            println!("  Step size: {:.6}", _step_result.step_size);
        }

        iteration += 1;
    }

    // Final results
    let final_value = problem.evaluate_f64(&initial_point)?;
    let final_gradient = problem.gradient_f64(&initial_point)?;
    let final_grad_norm = final_gradient.iter().map(|g| g * g).sum::<f64>().sqrt();

    println!("\nOptimization completed!");
    println!("Final point: {:?}", initial_point);
    println!("Final value: {:.6}", final_value);
    println!("Final gradient norm: {:.2e}", final_grad_norm);
    println!("Total iterations: {}", iteration);

    // Compare with known optimum
    let optimum = vec![1.0, 1.0];
    let distance_to_optimum = initial_point
        .iter()
        .zip(&optimum)
        .map(|(xi, opt)| (xi - opt).powi(2))
        .sum::<f64>()
        .sqrt();

    println!("Distance to optimum [1, 1]: {:.6}", distance_to_optimum);

    if distance_to_optimum < 1e-3 {
        println!("✓ Successfully found the global minimum!");
    } else {
        println!("⚠ Did not reach the global minimum within tolerance");
    }

    Ok(())
}
