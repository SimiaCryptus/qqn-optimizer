use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::optimizer_sets::{adam_variants, gd_variants, lbfgs_variants, qqn_line_search_optimizers, qqn_variants, standard_optimizers, trust_region_variants};
use crate::experiment_runner::problem_sets::{analytic_problems, ml_problems, mnist_problems};
use qqn_optimizer::{init_logging, OptimizationProblem};
use qqn_optimizer::benchmarks::evaluation::{disable_no_threshold_mode, enable_no_threshold_mode};

// #[tokio::test]
async fn families() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    // Enable no threshold mode for this test
    enable_no_threshold_mode();

    test("results/families_",{
        let mut problems = analytic_problems();
        problems.extend(ml_problems());
        problems
    }).await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

#[tokio::test]
async fn calibration() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    // Enable no threshold mode for this test
    enable_no_threshold_mode();

    test_all("results/calibration_",{
        let mut problems = analytic_problems();
        problems.extend(ml_problems());
        problems
    }).await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

#[tokio::test]
async fn full_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    // Disable no threshold mode for this test
    disable_no_threshold_mode();

    test_all("results/full_",{
        let mut problems = analytic_problems();
        problems.extend(ml_problems());
        problems
    }).await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

// #[tokio::test]
async fn test_mnist() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    // Enable no threshold mode for this test
    enable_no_threshold_mode();

    test("results/mnist_", mnist_problems(1000)).await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

async fn test_all(
    prefix: &str,
    problems: Vec<Arc<dyn OptimizationProblem>>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let max_evals = 1000;
    let num_runs = 20;
    run_benchmark(
        &format!("{}all_optimizers_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        {
            let mut optimizers = standard_optimizers();
            optimizers.extend(qqn_line_search_optimizers());
            optimizers.extend(lbfgs_variants());
            optimizers.extend(gd_variants());
            optimizers.extend(adam_variants());
            optimizers.extend(trust_region_variants());
            optimizers
        },
    ).await
}

async fn test(
    prefix: &str,
    problems: Vec<Arc<dyn OptimizationProblem>>
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let max_evals = 1000;
    let num_runs = 10;
    run_benchmark(
        &format!("{}qqn_variants_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        qqn_variants(),
    ).await?;

    run_benchmark(
        &format!("{}qqn_variants_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        qqn_line_search_optimizers(),
    ).await?;

    run_benchmark(
        &format!("{}lbfgs_variants_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        lbfgs_variants(),
    ).await?;

    run_benchmark(
        &format!("{}gd_variants_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        gd_variants(),
    ).await?;

    run_benchmark(
        &format!("{}adam_variants_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        adam_variants(),
    ).await?;

    run_benchmark(
        &format!("{}trust_region_variants_", prefix),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        problems.clone(),
        trust_region_variants(),
    ).await?;
    Ok(())
}