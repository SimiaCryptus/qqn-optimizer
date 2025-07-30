#!/usr/bin/env rust

//! Example demonstrating the Fashion-MNIST alternate problem suite
//! 
//! This example shows how to use the Fashion-MNIST neural network problems
//! as an alternative to the regular MNIST digit classification tasks.

use qqn_optimizer::benchmarks::fashion_mnist::{FashionMnistNeuralNetwork, ActivationType};
use qqn_optimizer::experiment_runner::problem_sets::fashion_mnist_problems;
use qqn_optimizer::{OptimizationProblem, QQNOptimizer, QQNConfig};
use rand::prelude::StdRng;
use rand::SeedableRng;

fn main() -> anyhow::Result<()> {
    println!("=== Fashion-MNIST Alternate Problem Suite Demo ===\n");

    // Create a simple Fashion-MNIST neural network problem
    let mut rng = StdRng::seed_from_u64(42);
    
    println!("1. Creating a Fashion-MNIST neural network with ReLU activation...");
    
    match FashionMnistNeuralNetwork::create_single_hidden(
        Some(100), // Use 100 samples for quick demo
        32,        // 32 hidden units
        Some(32),  // Batch size of 32
        &mut rng,
        Some(ActivationType::ReLU),
    ) {
        Ok(network) => {
            println!("   ✓ Successfully created Fashion-MNIST network");
            println!("   - Problem name: {}", network.name());
            println!("   - Problem dimension: {}", network.dimension());
            
            // Test evaluation
            let initial_point = network.initial_point();
            match network.evaluate_f64(&initial_point) {
                Ok(loss) => {
                    println!("   - Initial loss: {:.6}", loss);
                }
                Err(e) => {
                    println!("   - Could not evaluate: {}", e);
                }
            }
        }
        Err(e) => {
            println!("   ⚠ Could not create Fashion-MNIST network: {}", e);
            println!("     This is expected if Fashion-MNIST data is not available for download.");
        }
    }

    println!("\n2. Exploring Fashion-MNIST problem suite variants...");
    
    let problems = fashion_mnist_problems(50); // Small sample size for demo
    println!("   Available Fashion-MNIST problems:");
    
    for (i, problem) in problems.iter().enumerate() {
        if let Some(ref name) = problem.name {
            println!("   {}. {}", i + 1, name);
        } else {
            println!("   {}. {} (family)", i + 1, problem.family);
        }
    }

    println!("\n3. Demonstrating different activation functions...");
    
    let activations = [
        ("ReLU", ActivationType::ReLU),
        ("Logistic", ActivationType::Logistic),
        ("Sinewave", ActivationType::Sinewave),
    ];
    
    for (name, activation) in activations {
        let mut rng = StdRng::seed_from_u64(42);
        match FashionMnistNeuralNetwork::create_single_hidden(
            Some(20), // Very small for quick testing
            16,
            Some(10),
            &mut rng,
            Some(activation),
        ) {
            Ok(network) => {
                println!("   ✓ {} activation: {} parameters", 
                    name, network.dimension());
            }
            Err(e) => {
                println!("   ⚠ {} activation failed: {}", name, e);
            }
        }
    }

    println!("\n=== Fashion-MNIST vs Regular MNIST ===");
    println!("Fashion-MNIST provides an alternative benchmark with:");
    println!("• Clothing items instead of handwritten digits");
    println!("• Same 28x28 image format as MNIST");
    println!("• 10 classes: T-shirt, Trouser, Pullover, Dress, Coat, Sandal, Shirt, Sneaker, Bag, Ankle boot");
    println!("• Generally more challenging than digit classification");
    println!("• Better evaluation of optimization algorithms on realistic image data");

    println!("\nDemo complete! The Fashion-MNIST alternate problem suite is ready for use.");

    Ok(())
}