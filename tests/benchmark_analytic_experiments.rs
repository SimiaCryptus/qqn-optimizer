use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::optimizer_sets::standard_optimizers;
use crate::experiment_runner::problem_sets::analytic_problems;
use qqn_optimizer::{init_logging, OptimizationProblem};

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false)?;

    let result = run_benchmark(
        "results/functions_",
        1000,
        10,
        Duration::from_secs(60),
        analytic_problems(),
        standard_optimizers()
    ).await;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    result
}

