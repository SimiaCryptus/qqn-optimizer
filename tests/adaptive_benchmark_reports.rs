use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use qqn_optimizer::benchmarks::evaluation::{
    disable_no_threshold_mode, enable_no_threshold_mode, ProblemSpec,
};
use qqn_optimizer::experiment_runner::adaptive_runner::run_adaptive_benchmark;
use qqn_optimizer::experiment_runner::parameter_evolution::OptimizerType;
use qqn_optimizer::problem_sets::{analytic_problems};
use qqn_optimizer::{init_logging, OptimizationProblem, RosenbrockFunction, SphereFunction};
use tokio::task::LocalSet;

/// Test adaptive evolution on simple analytic problems
// #[tokio::test]
async fn test_adaptive_simple_problems() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    disable_no_threshold_mode();
    // enable_no_threshold_mode();

    let local = LocalSet::new();
    local
        .run_until(async move {
            // Use simple problems for quick testing
            // let problems = vec![
            //     ProblemSpec::new(
            //         Arc::new(SphereFunction::new(10)),
            //         "Sphere-10".to_string(),
            //         Some(10),
            //         42,
            //     ),
            //     ProblemSpec::new(
            //         Arc::new(RosenbrockFunction::new(5)),
            //         "Rosenbrock-5".to_string(),
            //         Some(5),
            //         42,
            //     ),
            // ];
            let problems = analytic_problems();

            // Test with a small population and few generations
            run_adaptive_benchmark(
                "results/adaptive_simple_",
                1000, // max_evals
                3,    // num_runs for final championship
                Duration::from_secs(60),
                10, // population_size
                50, // num_generations
                1,  // evaluation_runs per genome
                problems,
                vec![
                    OptimizerType::QQN,
                    OptimizerType::LBFGS,
                    OptimizerType::Adam,
                    OptimizerType::TrustRegion,
                    OptimizerType::GD,
                ],
            )
        })
        .await?;

    tokio::task::yield_now().await;
    Ok(())
}

/// Test adaptive evolution on analytic problems with more generations
// #[tokio::test]
#[ignore] // Run with --ignored flag for longer tests
async fn test_adaptive_analytic_full() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    disable_no_threshold_mode();

    let local = LocalSet::new();
    local
        .run_until(async move {
            let problems = analytic_problems();

            run_adaptive_benchmark(
                "results/adaptive_analytic_",
                1000, // max_evals
                10,   // num_runs for final championship
                Duration::from_secs(300),
                20, // population_size
                10, // num_generations
                5,  // evaluation_runs per genome
                problems,
                vec![
                    OptimizerType::QQN,
                    OptimizerType::LBFGS,
                    OptimizerType::GD,
                    OptimizerType::Adam,
                    OptimizerType::TrustRegion,
                ],
            )
        })
        .await?;

    tokio::task::yield_now().await;
    Ok(())
}

/// Quick smoke test for adaptive evolution
// #[tokio::test]
async fn test_adaptive_smoke() -> Result<(), Box<dyn Error + Send + Sync>> {
    //init_logging(true)?; // Enable verbose logging for debugging
    enable_no_threshold_mode();

    let local = LocalSet::new();
    local
        .run_until(async move {
            // Single simple problem for smoke testing
            let problems = vec![ProblemSpec::new(
                Arc::new(SphereFunction::new(5)),
                "Sphere-5".to_string(),
                Some(5),
                42,
            )];

            run_adaptive_benchmark(
                "results/adaptive_smoke_",
                100, // max_evals (very small)
                2,   // num_runs for final championship
                Duration::from_secs(30),
                5, // population_size (tiny)
                3, // num_generations (few)
                2, // evaluation_runs per genome
                problems,
                vec![OptimizerType::QQN, OptimizerType::Adam],
            )
        })
        .await?;

    tokio::task::yield_now().await;
    Ok(())
}

/// Test adaptive evolution focusing on QQN variants only
// #[tokio::test]
async fn test_adaptive_qqn_only() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    enable_no_threshold_mode();

    let local = LocalSet::new();
    local
        .run_until(async move {
            let problems = vec![ProblemSpec::new(
                Arc::new(RosenbrockFunction::new(10)),
                "Rosenbrock-10".to_string(),
                Some(10),
                42,
            )];

            // Focus on evolving QQN parameters only
            run_adaptive_benchmark(
                "results/adaptive_qqn_focus_",
                1000, // max_evals
                10,   // num_runs for final championship
                Duration::from_secs(120),
                20, // larger population_size for QQN
                10, // more generations
                5,  // evaluation_runs per genome
                problems,
                vec![OptimizerType::QQN], // Only QQN
            )
        })
        .await?;

    tokio::task::yield_now().await;
    Ok(())
}

/// Test adaptive evolution with very small budget
// #[tokio::test]
async fn test_adaptive_low_budget() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    enable_no_threshold_mode();

    let local = LocalSet::new();
    local
        .run_until(async move {
            let problems = analytic_problems()
                .into_iter()
                .take(2) // Only first 2 problems
                .collect();

            run_adaptive_benchmark(
                "results/adaptive_low_budget_",
                50, // very low max_evals
                3,  // few runs for final championship
                Duration::from_secs(60),
                8, // small population_size
                4, // few generations
                2, // minimal evaluation_runs
                problems,
                vec![OptimizerType::Adam, OptimizerType::GD],
            )
        })
        .await?;

    tokio::task::yield_now().await;
    Ok(())
}
