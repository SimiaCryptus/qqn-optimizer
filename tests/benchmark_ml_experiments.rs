use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::problem_sets::ml_problems;
use crate::experiment_runner::optimizer_sets;
use optimizer_sets::standard_optimizers;
use qqn_optimizer::{init_logging, OptimizationProblem};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::time::Duration;

mod experiment_runner;

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false).unwrap();
    
    let rng: StdRng = StdRng::seed_from_u64(42);

    let result = run_benchmark(
        "results/ml_",
        1000,
        10,
        Duration::from_secs(60),
        ml_problems(rng),
        standard_optimizers()
    ).await;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    result
}