use qqn_optimizer::benchmarks::functions::OptimizationProblem;
use qqn_optimizer::core::optimizer::Optimizer;
use rand::{Rng, SeedableRng};
use std::fs;
use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::standard_optimizers;
use experiment_runner::ExperimentRunner;
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};
use qqn_optimizer::benchmarks::analytic_functions::{AckleyFunction, BealeFunction, MichalewiczFunction, RastriginFunction, RosenbrockFunction, SphereFunction};
use qqn_optimizer::benchmarks::analytic_functions::{GoldsteinPriceFunction, LeviFunction, MatyasFunction, StyblinskiTangFunction};

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
            Box::new(SphereFunction::new(2)),
            Box::new(SphereFunction::new(10)),
            Box::new(RosenbrockFunction::new(2)),
            Box::new(RosenbrockFunction::new(5)),
            Box::new(BealeFunction::new()),
            Box::new(MatyasFunction::new()),
            Box::new(LeviFunction::new()),
            Box::new(GoldsteinPriceFunction::new()),
            Box::new(MichalewiczFunction::new(2)),
            Box::new(MichalewiczFunction::new(5)),
            Box::new(RastriginFunction::new(2)),
            Box::new(RastriginFunction::new(5)),
            Box::new(AckleyFunction::new(2)),
            Box::new(AckleyFunction::new(5)),
            Box::new(StyblinskiTangFunction::new(2)),
            Box::new(StyblinskiTangFunction::new(5)),
        ], standard_optimizers::standard_optimizers()),
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
