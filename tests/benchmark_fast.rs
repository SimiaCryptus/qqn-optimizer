use qqn_optimizer::benchmarks::functions::OptimizationProblem;
use qqn_optimizer::core::optimizer::Optimizer;
use rand::{Rng, SeedableRng};
use std::fs;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
use experiment_runner::ExperimentRunner;
use qqn_optimizer::benchmarks::analytic_functions::RosenbrockFunction;
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::core::GDOptimizer;
use qqn_optimizer::{init_logging, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, MnistNeuralNetwork, QQNConfig, QQNOptimizer};

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging()?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/benchmark/results{}", timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);
    fs::create_dir_all(&output_dir)?;
    println!("Creating benchmark results in: {}", output_dir.display());

    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        ExperimentRunner::new(output_dir.to_string_lossy().to_string(), BenchmarkConfig {
            max_iterations: 10000,
            maximum_function_calls: 10000,
            min_improvement_percent: 1e-3,
            time_limit: DurationWrapper::from(Duration::from_secs(60)),
            num_runs: 1,
        }).run_comparative_benchmarks(vec![
            Arc::new(
                {
                    let mut network = MnistNeuralNetwork::create(Some(100), 20)
                        .expect("Failed to create MNIST neural network");
                    network.set_optimal_value(Option::from(0.05));
                    network
                },
            ),  
        ], vec![
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