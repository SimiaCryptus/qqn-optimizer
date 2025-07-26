use crate::experiment_runner::{ExperimentRunner};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::benchmarks::mnist::{ActivationType, MnistNeuralNetwork};
use qqn_optimizer::{init_logging, AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, QQNConfig, QQNOptimizer};
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use rand::prelude::StdRng;
use rand::SeedableRng;

mod experiment_runner;

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false).unwrap();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/mnist_{}", timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir).unwrap();
    println!("Creating benchmark results in: {}", output_dir.display());

    let mut rng: StdRng = StdRng::seed_from_u64(42);
    let samples = 3000;
    let max_evals = 400;
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(output_dir.to_string_lossy().to_string(), BenchmarkConfig {
            max_iterations: max_evals,
            maximum_function_calls: max_evals,
            time_limit: DurationWrapper::from(Duration::from_secs(60)),
            num_runs: 2,
            ..BenchmarkConfig::default()
        }).run_comparative_benchmarks(vec![
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(
                        Some(samples),
                        &[
                            20,
                        ],
                        Some(samples),
                        &mut rng,
                        Some(ActivationType::ReLU)
                    ).expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(
                        Some(samples),
                        &[
                            // 20
                            20,
                        ],
                        Some(samples),
                        &mut rng,
                        Some(ActivationType::Logistic)
                    ).expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(
                        Some(samples),
                        &[
                            20,
                        ],
                        Some(samples),
                        &mut rng,
                        Some(ActivationType::Sinewave)
                    ).expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(
                        Some(samples),
                        &[
                            20, 20, 20
                        ],
                        Some(samples),
                        &mut rng,
                        Some(ActivationType::ReLU)
                    ).expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(
                        Some(samples),
                        &[
                            // 20
                            20, 20, 20
                        ],
                        Some(samples),
                        &mut rng,
                        Some(ActivationType::Logistic)
                    ).expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(
                        Some(samples),
                        &[
                            20, 20, 20
                        ],
                        Some(samples),
                        &mut rng,
                        Some(ActivationType::Sinewave)
                    ).expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
        ], vec![
            (
                "QQN-Backtracking-Fast".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        method: LineSearchMethod::Backtracking,
                        max_iterations: 5,
                        ..LineSearchConfig::default()
                    },
                    lbfgs_history: 15,
                    gradient_scale_factor: 1000.0,
                    ..Default::default()
                })),
            ),
            (
                "QQN-CubicQuadraticInterpolation-Fast".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        method: LineSearchMethod::CubicQuadraticInterpolation,
                        max_iterations: 5,
                        ..LineSearchConfig::default()
                    },
                    min_step_persist: 5e-1,
                    lbfgs_history: 30,
                    gradient_scale_factor: 1000.0,
                    ..Default::default()
                })),
            ),
            (
                "QQN-Backtracking".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        method: LineSearchMethod::Backtracking,
                        max_iterations: 5,
                        ..LineSearchConfig::default()
                    },
                    lbfgs_history: 15,
                    gradient_scale_factor: 1.0,
                    ..Default::default()
                })),
            ),
            (
                "QQN-CubicQuadraticInterpolation".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        method: LineSearchMethod::CubicQuadraticInterpolation,
                        max_iterations: 5,
                        ..LineSearchConfig::default()
                    },
                    min_step_persist: 5e-1,
                    lbfgs_history: 30,
                    gradient_scale_factor: 1.0,
                    ..Default::default()
                })),
            ),
            (
                "L-BFGS".to_string(),
                Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                    ..LBFGSConfig::lax()
                })),
            ),
            (
                "Adam-Fast".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.1,
                    lr_schedule: "adaptive".to_string(),
                    ..AdamConfig::deep_learning()
                })),
            ),
            (
                "Adam".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.005,
                    lr_schedule: "adaptive".to_string(),
                    ..AdamConfig::deep_learning()
                })),
            ),
        ]),
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