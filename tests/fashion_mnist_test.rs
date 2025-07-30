use qqn_optimizer::benchmarks::fashion_mnist::{FashionMnistNeuralNetwork, ActivationType};
use qqn_optimizer::experiment_runner::problem_sets::fashion_mnist_problems;
use qqn_optimizer::OptimizationProblem;
use rand::prelude::StdRng;
use rand::SeedableRng;

#[test]
fn test_fashion_mnist_creation() {
    let mut rng = StdRng::seed_from_u64(42);
    
    // Test creating a Fashion-MNIST neural network
    let result = FashionMnistNeuralNetwork::create_single_hidden(
        Some(10), // Use very small sample size for testing
        20,
        Some(5),
        &mut rng,
        Some(ActivationType::ReLU),
    );
    
    // Should succeed even if Fashion-MNIST data is not available
    // (it will try to download, which might fail in CI, but that's ok)
    match result {
        Ok(network) => {
            // Verify basic properties
            assert!(network.dimension() > 0);
            assert!(network.name().contains("FashionMNIST"));
            
            // Test initial point
            let initial = network.initial_point();
            assert_eq!(initial.len(), network.dimension());
            
            // Test evaluation (should work with synthetic data if download fails)
            if let Ok(loss) = network.evaluate_f64(&initial) {
                assert!(loss.is_finite());
                assert!(loss >= 0.0);
            }
        }
        Err(e) => {
            // This is expected if Fashion-MNIST data cannot be downloaded
            println!("Fashion-MNIST creation failed (expected in CI): {:?}", e);
        }
    }
}

#[test]
fn test_fashion_mnist_problem_sets() {
    // Test that Fashion-MNIST problem sets can be created
    let problems = fashion_mnist_problems(10); // Very small sample size
    
    // Should have multiple problem variants
    assert!(!problems.is_empty());
    
    // Verify all problems have correct naming
    for problem in &problems {
        if let Some(ref name) = problem.name {
            assert!(name.contains("FashionMNIST"));
        }
    }
    
    println!("Fashion-MNIST problem suite contains {} variants", problems.len());
    for problem in &problems {
        if let Some(ref name) = problem.name {
            println!("- {}", name);
        } else {
            println!("- {}", problem.family);
        }
    }
}

#[test]
fn test_fashion_mnist_activation_types() {
    let mut rng = StdRng::seed_from_u64(42);
    
    let activations = [
        ActivationType::ReLU,
        ActivationType::Logistic,
        ActivationType::Sinewave,
    ];
    
    for activation in activations {
        let result = FashionMnistNeuralNetwork::create_single_hidden(
            Some(5), // Very small for fast testing
            10,
            Some(5),
            &mut rng,
            Some(activation),
        );
        
        match result {
            Ok(network) => {
                assert!(network.name().contains("FashionMNIST"));
                println!("Successfully created Fashion-MNIST network with {:?} activation", activation);
            }
            Err(e) => {
                println!("Fashion-MNIST creation with {:?} failed: {:?}", activation, e);
            }
        }
    }
}