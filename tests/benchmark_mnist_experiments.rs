use crate::experiment_runner::{standard_optimizers, ExperimentRunner};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::benchmarks::mnist::MnistNeuralNetwork;
use qqn_optimizer::{init_logging, AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, LinearRegression, LogisticRegression, NeuralNetworkTraining, OptimizationProblem, QQNConfig, QQNOptimizer, SupportVectorMachine};
use std::fs;
use std::sync::Arc;
use std::time::Duration;
use rand::prelude::StdRng;
use rand::SeedableRng;
use qqn_optimizer::core::{GDConfig, GDOptimizer};

mod experiment_runner;

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging().unwrap();

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/benchmark/results{}", timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir).unwrap();
    println!("Creating benchmark results in: {}", output_dir.display());

    let mut rng: StdRng = rand::rngs::StdRng::seed_from_u64(42);
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(output_dir.to_string_lossy().to_string(), BenchmarkConfig {
            max_iterations: 1000,
            maximum_function_calls: 1000,
            min_improvement_percent: 1e-7,
            time_limit: DurationWrapper::from(Duration::from_secs(60)),
            num_runs: 1,
        }).run_comparative_benchmarks(vec![
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(Some(1000), 20, &mut rng)
                        .expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),
            // Arc::new(
            //     {
            //         let mut network = MnistNeuralNetwork::create(Some(10000), 30, &mut rng)
            //             .expect("Failed to create MNIST neural network");
            //         network.set_optimal_value(Option::from(0.05));
            //         network
            //     },
            // ),
        ], vec![
            // QQN variants
            (
                "QQN-Bisection".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig::default())),
            ),
            (
                "QQN-Backtracking-Hybrid".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        method: LineSearchMethod::Backtracking,
                        c1: 1e-3,
                        c2: 0.9,
                        max_iterations: 75,
                        ..LineSearchConfig::default()
                    },
                    lbfgs_history: 15,
                    ..Default::default()
                })),
            ),
            (
                "QQN-SimpleBracket".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        line_bracket_method: 2,
                        ..LineSearchConfig::default()
                    },
                    ..Default::default()
                })),
            ),
            (
                "QQN-StrongWolfe".to_string(),
                Arc::new(QQNOptimizer::new(QQNConfig {
                    line_search: LineSearchConfig {
                        method: LineSearchMethod::StrongWolfe,
                        ..LineSearchConfig::default()
                    },
                    ..Default::default()
                })),
            ),
            (
                "L-BFGS".to_string(),
                Arc::new(LBFGSOptimizer::new(LBFGSConfig::default())),
            ),
            (
                "L-BFGS-Aggressive".to_string(),
                Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                    history_size: 5,
                    max_step_size: 10.0,
                    max_param_change: 10.0,
                    gradient_clip: 0.0,
                    ..Default::default()
                })),
            ),
            (
                "L-BFGS-Hybrid".to_string(),
                Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                    history_size: 12,
                    max_step_size: 5.0,
                    max_param_change: 2.0,
                    gradient_clip: 50.0,
                    ..Default::default()
                })),
            ),
            (
                "GD".to_string(),
                Arc::new(GDOptimizer::new(Default::default())),
            ),
            (
                "GD-Conservative".to_string(),
                Arc::new(GDOptimizer::new(GDConfig {
                    learning_rate: 0.001,
                    momentum: 0.95,
                    max_grad_norm: 100.0,
                    adaptive_lr: false,
                    nesterov: true,
                    verbose: false,
                    ..Default::default()
                })),
            ),
            (
                "GD-Momentum".to_string(),
                Arc::new(GDOptimizer::new(GDConfig {
                    momentum: 0.9,
                    ..Default::default()
                })),
            ),

            (
                "Adam-Fast".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.1,
                    lr_schedule: "constant".to_string(),
                    ..Default::default()
                })),
            ),
            (
                "Adam-Fast-Conservative".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.05,
                    lr_schedule: "constant".to_string(),
                    gradient_clip: Some(1.0),
                    ..Default::default()
                })),
            ),
            (
                "Adam-Fast-Aggressive".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.125,
                    lr_schedule: "constant".to_string(),
                    beta1: 0.85,
                    ..Default::default()
                })),
            ),
            (
                "Adam-Fast-Adaptive".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.1,
                    lr_schedule: "adaptive".to_string(),
                    gradient_clip: Some(2.0),
                    ..Default::default()
                })),
            ),
            (
                "Adam-Fast-Momentum".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.1,
                    lr_schedule: "constant".to_string(),
                    beta1: 0.95,
                    beta2: 0.999,
                    ..Default::default()
                })),
            ),
            (
                "Adam-Fast-Regularized".to_string(),
                Arc::new(AdamOptimizer::new(AdamConfig {
                    learning_rate: 0.1,
                    lr_schedule: "constant".to_string(),
                    weight_decay: 0.01,
                    gradient_clip: Some(1.0),
                    ..Default::default()
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
    assert!(output_dir.join("benchmark_report.html").exists());
    assert!(output_dir.join("detailed_results.csv").exists());
    assert!(output_dir.join("summary_statistics.csv").exists());

    // Read and verify HTML content
    let html_content = fs::read_to_string(output_dir.join("benchmark_report.html"))?;
    assert!(html_content.contains("QQN Optimizer"));
    // assert!(html_content.contains("Executive Summary"));
    assert!(html_content.contains("Statistical Analysis"));
    // assert!(html_content.contains("Performance Profiles"));

    println!(
        "Comprehensive benchmark report generated at: {}",
        output_dir.display()
    );
    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}