use crate::experiment_runner::{standard_optimizers, ExperimentRunner};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::benchmarks::mnist::MnistNeuralNetwork;
use qqn_optimizer::{init_logging, LinearRegression, LogisticRegression, NeuralNetworkTraining, OptimizationProblem, SupportVectorMachine};
use rand::{Rng, SeedableRng};
use std::fs;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
/// Create ML-specific test problems for benchmarking
fn create_ml_test_problems() -> Vec<Arc<dyn OptimizationProblem>> {
    vec![
        Arc::new(
            LogisticRegression::synthetic(100, 5)
                .expect("Failed to create synthetic logistic regression"),
        ),
        Arc::new(
            LogisticRegression::synthetic(200, 10)
                .expect("Failed to create synthetic logistic regression"),
        ),
        Arc::new(LinearRegression::new(
            generate_linear_regression_data(100, 5).0,
            generate_linear_regression_data(100, 5).1,
            0.01,
        ).expect("Failed to create linear regression")),
        Arc::new(LinearRegression::new(
            generate_linear_regression_data(200, 10).0,
            generate_linear_regression_data(200, 10).1,
            0.01,
        ).expect("Failed to create linear regression")),
        Arc::new(
            NeuralNetworkTraining::mlp_classification(vec![5, 10, 3])
                .expect("Failed to create MLP"),
        ),
        Arc::new(
            NeuralNetworkTraining::mlp_classification(vec![10, 20, 5])
                .expect("Failed to create MLP"),
        ),
        Arc::new(
            MnistNeuralNetwork::create(Some(100), 20)
                .expect("Failed to create MNIST neural network"),
        ),
        Arc::new(
            MnistNeuralNetwork::create(Some(200), 30)
                .expect("Failed to create MNIST neural network"),
        ),
        Arc::new(SupportVectorMachine::new(
            generate_svm_data(100, 5).0,
            generate_svm_data(100, 5).1,
            1.0,
        ).expect("Failed to create SVM")),
        Arc::new(SupportVectorMachine::new(
            generate_svm_data(200, 10).0,
            generate_svm_data(200, 10).1,
            1.0,
        ).expect("Failed to create SVM")),
    ]
}
/// Generate synthetic linear regression data
fn generate_linear_regression_data(
    n_samples: usize,
    n_features: usize,
) -> (Vec<Vec<f64>>, Vec<f64>) {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    let true_weights: Vec<f64> = (0..n_features).map(|i| (i as f64 + 1.0) * 0.5).collect();
    for _ in 0..n_samples {
        let x: Vec<f64> = (0..n_features)
            .map(|_| rng.random_range(-2.0..2.0))
            .collect();
        let y: f64 = x
            .iter()
            .zip(true_weights.iter())
            .map(|(xi, wi)| xi * wi)
            .sum::<f64>()
            + rng.random_range(-0.1..0.1);
        x_data.push(x);
        y_data.push(y);
    }
    (x_data, y_data)
}
/// Generate synthetic SVM data
fn generate_svm_data(n_samples: usize, n_features: usize) -> (Vec<Vec<f64>>, Vec<f64>) {
    use rand::Rng;
    let mut rng = rand::rng();
    let mut x_data = Vec::new();
    let mut y_data = Vec::new();
    for _ in 0..n_samples {
        let x: Vec<f64> = (0..n_features)
            .map(|_| rng.random_range(-2.0..2.0))
            .collect();
        let decision_value: f64 = x
            .iter()
            .enumerate()
            .map(|(i, xi)| xi * (i as f64 + 1.0) * 0.3)
            .sum();
        let y = if decision_value > 0.0 { 1.0 } else { -1.0 };
        x_data.push(x);
        y_data.push(y);
    }
    (x_data, y_data)
}


#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging().unwrap();
    // Use a persistent directory with timestamp to avoid conflicts
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/benchmark/results{}", timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&output_dir);
    println!("Creating benchmark results in: {}", output_dir.display());

    let output_dir1 = output_dir.to_string_lossy().to_string();
    let runner = ExperimentRunner::new(output_dir1, BenchmarkConfig {
        max_iterations: 1000,
        maximum_function_calls: 1000,
        min_improvement_percent: 1e-3,
        time_limit: DurationWrapper::from(Duration::from_secs(60)),
        num_runs: 1,
    });

    // Create ML-specific test problems
    let test_problems = create_ml_test_problems();

    // Wrap the main execution in a timeout to prevent hanging
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        runner.run_comparative_benchmarks(test_problems, standard_optimizers::standard_optimizers()),
    )
        .await;

    match result {
        Ok(Ok(())) => {
            println!("Benchmark completed successfully");
        }
        Ok(Err(e)) => {
            eprintln!("Benchmark failed: {}", e);
            return Err(e.into());
        }
        Err(_) => {
            eprintln!("Benchmark timed out");
            return Err("Benchmark execution timed out".into());
        }
    }

    // Verify outputs were generated
    assert!(output_dir.join("benchmark_report.html").exists());
    assert!(output_dir.join("detailed_results.csv").exists());
    assert!(output_dir.join("summary_statistics.csv").exists());

    // Read and verify HTML content
    let html_content = fs::read_to_string(output_dir.join("benchmark_report.html"))?;
    assert!(html_content.contains("QQN Optimizer"));
    assert!(html_content.contains("Executive Summary"));
    assert!(html_content.contains("Statistical Analysis"));
    assert!(html_content.contains("Performance Profiles"));

    println!(
        "Comprehensive benchmark report generated at: {}",
        output_dir.display()
    );
    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}