use qqn_optimizer::benchmarks::functions::OptimizationProblem;
use qqn_optimizer::core::optimizer::Optimizer;
use rand::{Rng, SeedableRng};
use std::fs;
use std::time::Duration;

mod experiment_runner;
use experiment_runner::ExperimentRunner;
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::core::GDOptimizer;
use qqn_optimizer::{AdamOptimizer, LBFGSConfig, LBFGSOptimizer, QQNConfig, QQNOptimizer};
use qqn_optimizer::benchmarks::analytic_functions::RosenbrockFunction;

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // init_logging()?;
    // Use a persistent directory with timestamp to avoid conflicts
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/benchmark/results{}", timestamp);
    let output_dir = std::path::PathBuf::from(&output_dir_name);

    // Create the directory if it doesn't exist
    fs::create_dir_all(&output_dir)?;
    println!("Creating benchmark results in: {}", output_dir.display());

    let output_dir1 = output_dir.to_string_lossy().to_string();
    let runner = ExperimentRunner::new(output_dir1, BenchmarkConfig {
        max_iterations: 10000,
        maximum_function_calls: 10000,
        tolerance: 1e-8,
        time_limit: DurationWrapper::from(Duration::from_secs(60)),
        num_runs: 10,
    });

    // Wrap the main execution in a timeout to prevent hanging
    let result = tokio::time::timeout(
        Duration::from_secs(30000),
        runner.run_comparative_benchmarks(vec![
            Box::new(RosenbrockFunction::new(5)),
        ], vec![
            (
                "QQN-Default".to_string(),
                Box::new(QQNOptimizer::new(QQNConfig::default())),
            ),
            (
                "L-BFGS".to_string(),
                Box::new(LBFGSOptimizer::new(LBFGSConfig::default())),
            ),
            (
                "GD".to_string(),
                Box::new(GDOptimizer::new(Default::default())),
            ),
            (
                "Adam".to_string(),
                Box::new(AdamOptimizer::new(Default::default())),
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
