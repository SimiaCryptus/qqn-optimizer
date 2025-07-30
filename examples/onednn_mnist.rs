#!/usr/bin/env -S cargo +nightly -Zscript
//! OneDNN MNIST Neural Network Example
//!
//! This example demonstrates how to use the OneDNN-based MNIST neural network
//! implementation with the QQN optimizer.
//!
//! To run this example:
//! ```bash
//! # First install OneDNN (see docs/onednn_mnist.md)
//! cargo run --example onednn_mnist --features onednn
//! ```

use qqn_optimizer::{
    experiment_runner::problem_sets::mnist_onednn_problems, init_logging,
    line_search::strong_wolfe::StrongWolfeLineSearch, optimizers::Optimizer, QQNOptimizer,
};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

#[cfg(feature = "onednn")]
use qqn_optimizer::{benchmarks::mnist_onednn::ActivationType, MnistOneDnnNeuralNetwork};

fn main() -> anyhow::Result<()> {
    // Initialize logging
    init_logging(false)?;

    println!("OneDNN MNIST Neural Network Example");
    println!("==================================");

    #[cfg(not(feature = "onednn"))]
    {
        println!("❌ OneDNN feature not enabled!");
        println!("To run this example with OneDNN support:");
        println!("  cargo run --example onednn_mnist --features onednn");
        println!("\nNote: OneDNN must be installed on your system.");
        println!("See docs/onednn_mnist.md for installation instructions.");
        return Ok(());
    }

    #[cfg(feature = "onednn")]
    {
        run_onednn_example()?;
    }

    Ok(())
}

#[cfg(feature = "onednn")]
fn run_onednn_example() -> anyhow::Result<()> {
    let mut rng = StdRng::seed_from_u64(42);

    println!("🚀 Creating OneDNN-based MNIST neural network...");

    // Create a small network for demonstration
    let network = MnistOneDnnNeuralNetwork::create(
        Some(100), // 100 samples for quick demo
        &[32, 16], // Two hidden layers: 32 and 16 neurons
        Some(32),  // Batch size of 32
        &mut rng,
        Some(ActivationType::ReLU), // ReLU activation
    )?;

    println!("✅ Network created successfully!");
    println!("   - Architecture: 784 → 32 → 16 → 10");
    println!("   - Activation: ReLU (hidden), Logistic (output)");
    println!("   - Parameters: {}", network.dimension());
    println!("   - Training samples: 100");

    // Verify initialization
    network.verify_initialization()?;

    // Test function evaluation
    println!("\n🧮 Testing function evaluation...");
    let start = Instant::now();
    let initial_params = network.initial_point();
    let initial_loss = network.evaluate_f64(&initial_params)?;
    let eval_time = start.elapsed();

    println!("   - Initial loss: {:.6}", initial_loss);
    println!("   - Evaluation time: {:?}", eval_time);

    // Test gradient computation
    println!("\n🔧 Testing gradient computation...");
    let start = Instant::now();
    let gradient = network.gradient_f64(&initial_params)?;
    let grad_time = start.elapsed();

    let grad_norm: f64 = gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
    println!("   - Gradient norm: {:.6}", grad_norm);
    println!("   - Gradient computation time: {:?}", grad_time);

    // Run optimization with QQN
    println!("\n🎯 Running optimization with QQN...");
    let line_search = StrongWolfeLineSearch::new();
    let mut optimizer = QQNOptimizer::new(line_search);

    let start = Instant::now();
    let result = optimizer.optimize(
        &|x: &[f64]| network.evaluate_f64(x).unwrap(),
        &|x: &[f64]| network.gradient_f64(x).unwrap(),
        initial_params,
        50,   // Max 50 function evaluations for demo
        1e-4, // Gradient tolerance
    );
    let opt_time = start.elapsed();

    println!("✅ Optimization completed!");
    println!("   - Final loss: {:.6}", result.fx);
    println!("   - Function evaluations: {}", result.num_f_evals);
    println!("   - Total time: {:?}", opt_time);
    println!("   - Converged: {}", result.converged);

    // Performance comparison hint
    println!("\n📊 Performance Comparison:");
    println!("   To compare OneDNN vs Candle performance, run:");
    println!("   cargo run --example benchmark_comparison --features onednn");

    // Problem set demonstration
    println!("\n📋 Available OneDNN Problem Sets:");
    let problems = mnist_onednn_problems(50); // Small set for demo
    for (i, problem) in problems.iter().enumerate() {
        println!(
            "   {}. {} (dim: {})",
            i + 1,
            problem.name(),
            problem.problem().dimension()
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onednn_example_compiles() {
        // This test ensures the example compiles even without OneDNN
        assert!(true);
    }

    #[cfg(feature = "onednn")]
    #[test]
    fn test_onednn_network_creation() {
        let mut rng = StdRng::seed_from_u64(42);

        // Test creating a small network
        let network = MnistOneDnnNeuralNetwork::create(
            Some(10),
            &[8],
            Some(5),
            &mut rng,
            Some(ActivationType::ReLU),
        );

        assert!(network.is_ok());

        if let Ok(net) = network {
            assert_eq!(net.dimension(), 8 * 784 + 8 + 8 * 10 + 10);
            assert!(net.name().contains("OneDNN"));
        }
    }
}
