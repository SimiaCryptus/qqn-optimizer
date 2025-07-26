use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::optimizer_sets::{adam_variants, gd_variants, lbfgs_variants, qqn_line_search_optimizers, qqn_variants, standard_optimizers, trust_region_variants};
use crate::experiment_runner::problem_sets::{analytic_problems, ml_problems, mnist_problems};
use qqn_optimizer::{init_logging, OptimizationProblem};

#[tokio::test]
async fn test_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false)?;

    test("results/tests_",{
        let mut problems = analytic_problems();
        problems.extend(ml_problems());
        problems
    }).await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

#[tokio::test]
async fn test_mnist() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false)?;

    test("results/mnist_", mnist_problems(1000)).await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

async fn test(
    prefix: &str,
    problems: Vec<Arc<dyn OptimizationProblem>>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    // run_benchmark(
    //     &format!("{}standard_optimizers_", prefix),
    //     1000,
    //     10,
    //     Duration::from_secs(60),
    //     problems.clone(),
    //     standard_optimizers(),
    // ).await?;

    run_benchmark(
        &format!("{}qqn_variants_", prefix),
        1000,
        10,
        Duration::from_secs(60),
        problems.clone(),
        qqn_variants(),
    ).await?;

    run_benchmark(
        &format!("{}qqn_variants_", prefix),
        1000,
        10,
        Duration::from_secs(60),
        problems.clone(),
        qqn_line_search_optimizers(),
    ).await?;

    run_benchmark(
        &format!("{}lbfgs_variants_", prefix),
        1000,
        10,
        Duration::from_secs(60),
        problems.clone(),
        lbfgs_variants(),
    ).await?;

    run_benchmark(
        &format!("{}gd_variants_", prefix),
        1000,
        10,
        Duration::from_secs(60),
        problems.clone(),
        gd_variants(),
    ).await?;

    run_benchmark(
        &format!("{}adam_variants_", prefix),
        1000,
        10,
        Duration::from_secs(60),
        problems.clone(),
        adam_variants(),
    ).await?;

    run_benchmark(
        &format!("{}trust_region_variants_", prefix),
        1000,
        10,
        Duration::from_secs(60),
        problems.clone(),
        trust_region_variants(),
    ).await?;
    Ok(())
}
