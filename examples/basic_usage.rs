//! Basic usage example demonstrating QQN optimization on the Rosenbrock function.
//!
//! This example shows how to:
//! - Create and configure a QQN optimizer
//! - Define an optimization problem
//! - Run the optimization loop
//! - Analyze the results

use anyhow::Result;
use qqn_optimizer::{
    QQNOptimizer, QQNConfig, RosenbrockFunction, OptimizationProblem,
    LineSearchConfig, StrongWolfeConfig
};

fn main() -> Result<()> {
    // Configure the QQN optimizer
    let config = QQNConfig {
        threshold: 0.01,           // Magnitude difference threshold
        lbfgs_history: 10,         // L-BFGS history length
        line_search: LineSearchConfig::StrongWolfe(StrongWolfeConfig {
            c1: 1e-4,              // Armijo condition parameter
            c2: 0.9,               // Curvature condition parameter
            max_iterations: 20,     // Maximum line search iterations
        }),
        epsilon: 1e-8,             // Numerical stability constant
    };

    let mut optimizer = QQNOptimizer::new(config);

    // Define the optimization problem (2D Rosenbrock function)
    let problem = RosenbrockFunction::new(2);
    let mut x = problem.initial_point(); // Start at [-1.2, 1.0]

    println!("Starting optimization of 2D Rosenbrock function");
    println!("Initial point: {:?}", x);
    println!("Initial value: {:.6}", problem.evaluate(&x)?);

    // Optimization loop
    let mut iteration = 0;
    let max_iterations = 1000;

    while iteration < max_iterations {
        // Compute gradient
        let gradient = problem.gradient(&x)?;
        let grad_norm = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();

        // Print progress
        if iteration % 10 == 0 {
            let f_val = problem.evaluate(&x)?;
            println!("Iteration {}: f = {:.6}, ||∇f|| = {:.6}", iteration, f_val, grad_norm);
        }

        // Check convergence
        if grad_norm < 1e-6 {
            println!("Converged! Gradient norm: {:.2e}", grad_norm);
            break;
        }

        // Perform optimization step
        let step_result = optimizer.step(&mut x, &gradient)?;

        // Print step information
        if iteration % 50 == 0 {
            println!("  Step size: {:.6}", step_result.step_size);
            println!("  Function evaluations: {}", step_result.function_evaluations);
            println!("  QQN mode: {}", if step_result.convergence_info.qqn_mode_active { "Hybrid" } else { "L-BFGS" });
        }

        iteration += 1;
    }

    // Final results
    let final_value = problem.evaluate(&x)?;
    let final_gradient = problem.gradient(&x)?;
    let final_grad_norm = final_gradient.iter().map(|g| g * g).sum::<f64>().sqrt();

    println!("\nOptimization completed!");
    println!("Final point: {:?}", x);
    println!("Final value: {:.6}", final_value);
    println!("Final gradient norm: {:.2e}", final_grad_norm);
    println!("Total iterations: {}", iteration);

    // Compare with known optimum
    let optimum = vec![1.0, 1.0];
    let distance_to_optimum = x.iter()
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