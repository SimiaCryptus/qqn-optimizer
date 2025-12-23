use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use rand::prelude::StdRng;
use rand::{rng, SeedableRng};
use qqn_optimizer::benchmarks::evaluation::{disable_no_threshold_mode, ProblemSpec};
use qqn_optimizer::experiment_runner::experiment_runner::run_benchmark;
use qqn_optimizer::optimizer_sets::{
    adam_variants, gd_variants, lbfgs_variants, qqn_variants, trust_region_variants,
};
use qqn_optimizer::problem_sets::analytic_problems;
use qqn_optimizer::{
    init_logging
    , SphereFunction,
};
use tokio::task::LocalSet;
use qqn_optimizer::benchmarks::mnist::MnistProblem;

// #[tokio::test]
async fn full_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    disable_no_threshold_mode();
    LocalSet::new()
        .run_until(async move {
            let mut optimizers = qqn_variants();
            optimizers.extend(lbfgs_variants());
            optimizers.extend(gd_variants());
            optimizers.extend(adam_variants());
            optimizers.extend(trust_region_variants());
            run_benchmark(
                &"results/full_all_optimizers_",
                5000,
                5,
                Duration::from_secs(600),
                Some(8),
                all_problems().clone(),
                optimizers,
                2e-1,
            )
        })
        .await?;
    tokio::task::yield_now().await; // Explicitly flush any pending async operations
    Ok(())
}

#[tokio::test]
async fn mnist_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    disable_no_threshold_mode();
    LocalSet::new()
        .run_until(async move {
            let mut optimizers = qqn_variants();
            // optimizers.extend(lbfgs_variants());
            optimizers.extend(gd_variants());
            optimizers.extend(adam_variants());
            // optimizers.extend(trust_region_variants());
            let mut rng = StdRng::seed_from_u64(42);
            run_benchmark(
                &"results/mnist_all_optimizers_",
                5000,
                5,
                Duration::from_secs(600),
                Some(8),
                vec![ProblemSpec::new(
                    Arc::new(MnistProblem::new(
                        1000,
                        10,
                        &mut rng
                    )),
                    "Sphere".to_string(),
                    Some(2),
                    42,
                )],
                optimizers,
                2e-1,
            )
        })
        .await?;
    tokio::task::yield_now().await; // Explicitly flush any pending async operations
    Ok(())
}

// #[tokio::test]
async fn full_test_sync() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    disable_no_threshold_mode();
    let mut optimizers = qqn_variants();
    optimizers.extend(lbfgs_variants());
    optimizers.extend(gd_variants());
    optimizers.extend(adam_variants());
    optimizers.extend(trust_region_variants());
    run_benchmark(
        &"results/full_all_optimizers_",
        5000,
        3,
        Duration::from_secs(600),
        Some(8),
        all_problems(),
        optimizers,
        2e-1,
    )
    .expect("Benchmarking failed");
    tokio::task::yield_now().await; // Explicitly flush any pending async operations
    Ok(())
}

fn all_problems() -> Vec<ProblemSpec> {
    let mut problems = analytic_problems();
    problems
}

#[allow(dead_code)]
fn test(prefix: &str, problems: Vec<ProblemSpec>) -> Result<(), Box<dyn Error + Send + Sync>> {
    let max_evals = 1000;
    let num_runs = 10;
    run_benchmark(
        &format!("{prefix}qqn_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        qqn_variants(),
        2e-1,
    );

    run_benchmark(
        &format!("{prefix}qqn_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        qqn_variants(),
        2e-1,
    );

    run_benchmark(
        &format!("{prefix}lbfgs_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        lbfgs_variants(),
        2e-1,
    );

    run_benchmark(
        &format!("{prefix}gd_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        gd_variants(),
        2e-1,
    );

    run_benchmark(
        &format!("{prefix}adam_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        adam_variants(),
        2e-1,
    );

    run_benchmark(
        &format!("{prefix}trust_region_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        trust_region_variants(),
        2e-1,
    );
    Ok(())
}
