use crate::experiment_runner::{standard_optimizers, ExperimentRunner};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::benchmarks::mnist::MnistNeuralNetwork;
use qqn_optimizer::{init_logging, LinearRegression, LogisticRegression, NeuralNetworkTraining, OptimizationProblem, SupportVectorMachine};
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use rand::prelude::StdRng;
use qqn_optimizer::benchmarks::ml_problems::{generate_linear_regression_data, generate_svm_data};
use standard_optimizers::standard_optimizers;
use rand::SeedableRng;

mod experiment_runner;

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging().unwrap();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/ml_{}", timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir).unwrap();
    println!("Creating benchmark results in: {}", output_dir.display());

    let mut rng: StdRng = StdRng::seed_from_u64(42);
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(output_dir.to_string_lossy().to_string(), BenchmarkConfig {
            max_iterations: 1000,
            maximum_function_calls: 1000,
            min_improvement_percent: 1e-7,
            time_limit: DurationWrapper::from(Duration::from_secs(60)),
            num_runs: 10,
        }).run_comparative_benchmarks(vec![
            Arc::new(
                {
                    let mut regression = LogisticRegression::synthetic(100, 5, &mut rng)
                        .expect("Failed to create synthetic logistic regression");
                    regression.set_optimal_value(Option::from(3.5e-1));
                    regression
                },
            ),
            Arc::new(
                {
                    let mut regression = LogisticRegression::synthetic(200, 10, &mut rng)
                        .expect("Failed to create synthetic logistic regression");
                    regression.set_optimal_value(Option::from(3.4e-1));
                    regression
                },
            ),
            Arc::new({
                let mut regression = LinearRegression::new(
                    generate_linear_regression_data(100, 5, &mut rng).0,
                    generate_linear_regression_data(100, 5, &mut rng).1,
                    0.01,
                ).expect("Failed to create linear regression");
                regression.set_optimal_value(Option::from(1.8e1));
                regression
            }),
            Arc::new({
                let mut regression = LinearRegression::new(
                    generate_linear_regression_data(200, 10, &mut rng).0,
                    generate_linear_regression_data(200, 10, &mut rng).1,
                    0.01,
                ).expect("Failed to create linear regression");
                regression.set_optimal_value(Option::from(1.0e2));
                regression
            }),
            Arc::new(
                {
                    let mut training = NeuralNetworkTraining::mlp_classification(vec![5, 10, 3], &mut rng)
                        .expect("Failed to create MLP");
                    // training.set_optimal_value(Option::from(1.7e-1));
                    training
                },
            ),
            Arc::new(
                {
                    let mut training = NeuralNetworkTraining::mlp_classification(vec![10, 20, 5], &mut rng)
                        .expect("Failed to create MLP");
                    training.set_optimal_value(Option::from(1.0e-1));
                    training
                },
            ),
            Arc::new(SupportVectorMachine::new(
                generate_svm_data(100, 5, &mut rng).0,
                generate_svm_data(100, 5, &mut rng).1,
                1.0,
            ).expect("Failed to create SVM")),
            Arc::new(SupportVectorMachine::new(
                generate_svm_data(200, 10, &mut rng).0,
                generate_svm_data(200, 10, &mut rng).1,
                1.0,
            ).expect("Failed to create SVM")),
        ], standard_optimizers()),
    ).await;

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
    assert!(output_dir.join("benchmark_report.md").exists());
    assert!(fs::read_to_string(output_dir.join("benchmark_report.md"))?.contains("QQN Optimizer"));

    println!(
        "Comprehensive benchmark report generated at: {}",
        output_dir.display()
    );
    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}