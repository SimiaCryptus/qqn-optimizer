use std::fs;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::standard_optimizers;
use experiment_runner::ExperimentRunner;
use qqn_optimizer::benchmarks::analytic_functions::{AckleyFunction, BealeFunction, MichalewiczFunction, RastriginFunction, RosenbrockFunction, SphereFunction};
use qqn_optimizer::benchmarks::analytic_functions::{GoldsteinPriceFunction, LeviFunction, MatyasFunction, StyblinskiTangFunction};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, DurationWrapper};

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // init_logging()?;
    
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
            min_improvement_percent: 1e-7,
            time_limit: DurationWrapper::from(Duration::from_secs(60)),
            num_runs: 10,
        }).run_comparative_benchmarks(vec![
            Arc::new(SphereFunction::new(2)),
            Arc::new(SphereFunction::new(10)),
            Arc::new(RosenbrockFunction::new(2)),
            Arc::new(RosenbrockFunction::new(5)),
            Arc::new(RosenbrockFunction::new(10)),
            Arc::new(MichalewiczFunction::new(2)),
            Arc::new(MichalewiczFunction::new(5)),
            Arc::new(MichalewiczFunction::new(10)),
            Arc::new(RastriginFunction::new(2)),
            Arc::new(RastriginFunction::new(5)),
            Arc::new(RastriginFunction::new(10)),
            Arc::new(AckleyFunction::new(2)),
            Arc::new(AckleyFunction::new(5)),
            Arc::new(AckleyFunction::new(10)),
            Arc::new(StyblinskiTangFunction::new(2)),
            Arc::new(StyblinskiTangFunction::new(5)),
            Arc::new(StyblinskiTangFunction::new(10)),
            Arc::new(BealeFunction::new()),
            Arc::new(LeviFunction::new()),
            Arc::new(GoldsteinPriceFunction::new()),
            Arc::new(MatyasFunction::new()),
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
    assert!(output_dir.join("benchmark_report.md").exists());
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
