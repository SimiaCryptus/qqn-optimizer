use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use qqn_optimizer::benchmarks::evaluation::{
    disable_no_threshold_mode, enable_no_threshold_mode, ProblemSpec,
};
use qqn_optimizer::benchmarks::mnist_onednn::ActivationType;
use qqn_optimizer::experiment_runner::experiment_runner::run_benchmark;
use qqn_optimizer::optimizer_sets::{
    adam_variants, gd_variants, lbfgs_variants, qqn_variants, trust_region_variants,
};
use qqn_optimizer::problem_sets::{analytic_problems, ml_problems, mnist_problems};
use qqn_optimizer::{
    init_logging, LineSearchConfig, LineSearchMethod, MnistOneDnnNeuralNetwork,
    OptimizationProblem, Optimizer, QQNConfig, QQNOptimizer, RosenbrockFunction,
};
use rand::SeedableRng;
use tokio::task::LocalSet;

// #[tokio::test]
#[allow(dead_code)]
async fn calibration() -> Result<(), Box<dyn Error + Send + Sync>> {
    // init_logging(false)?;
    // Enable no threshold mode for this test
    enable_no_threshold_mode();

    let local = LocalSet::new();
    local
        .run_until(async move {
            let problems = {
                let mut problems = analytic_problems();
                problems.extend(ml_problems());
                problems
            };
            let prefix = &"results/calibration_";
            let max_cpu = Some(8);
            let time_limit = Duration::from_secs(600);
            run_benchmark(
                &format!("{prefix}all_optimizers_"),
                1000,
                10,
                time_limit,
                max_cpu,
                problems.clone(),
                all_optimizers(),
                2e-1,
            )
            .await
        })
        .await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

// #[tokio::test]
async fn full_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    disable_no_threshold_mode();
    LocalSet::new()
        .run_until(async move {
            run_benchmark(
                &"results/full_all_optimizers_",
                5000,
                20,
                Duration::from_secs(600),
                Some(8),
                all_problems().clone(),
                all_optimizers(),
                2e-1,
            )
            .await
        })
        .await?;
    tokio::task::yield_now().await; // Explicitly flush any pending async operations
    Ok(())
}

#[tokio::test]
async fn one_test() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(true)?;
    disable_no_threshold_mode();
    LocalSet::new()
        .run_until(async move {
            let network = MnistOneDnnNeuralNetwork::create(
                Some(1000), // 1000 samples for a more substantial test
                &[32, 16],  // Two hidden layers: 32 and 16 neurons
                Some(1000), // Batch size of 32
                &mut rand::rngs::StdRng::seed_from_u64(42),
                Some(ActivationType::Logistic),
            )
            .unwrap();
            let dimensions = Some(network.dimension());
            run_benchmark(
                &"results/one_test_",
                1000,
                1,
                Duration::from_secs(600),
                Some(8),
                vec![
                    // ProblemSpec::new(
                    //     Arc::new(RosenbrockFunction::new(10)),
                    //     "Rosenbrock".to_string(),
                    //     Some(10),
                    //     42,
                    // ),
                    ProblemSpec::new(
                        Arc::new(network),
                        "MnistOneDnnNeuralNetwork".to_string(),
                        dimensions,
                        42,
                    ),
                ],
                vec![
                    (
                        "QQN-Bisection-2".to_string(),
                        Arc::new(QQNOptimizer::new(QQNConfig {
                            line_search: LineSearchConfig {
                                method: LineSearchMethod::Bisection,
                                line_bracket_method: 2,
                                c1: 1e-4,
                                c2: 0.9,
                                max_iterations: 20,
                                initial_step: 1.0,
                                min_step: 1e-10,
                                max_step: 10.0,
                                verbose: false,
                            },
                            lbfgs_history: 10,
                            epsilon: 1e-6,
                            verbose: false,
                            ..Default::default()
                        })),
                    ),
                    // (
                    //     "Adam-Fast".to_string(),
                    //     Arc::new(AdamOptimizer::new(AdamConfig {
                    //         learning_rate: 0.1,
                    //         lr_schedule: "constant".to_string(),
                    //         lr_decay: 0.995,
                    //         min_learning_rate: 1e-6,
                    //         gradient_clip: Some(10.0),
                    //         beta1: 0.9,
                    //         beta2: 0.999,
                    //         epsilon: 1e-8,
                    //         weight_decay: 0.0,
                    //         amsgrad: false,
                    //         max_line_search_iter: 20,
                    //         verbose: false,
                    //     })),
                    // ),
                ],
                2e-1,
            )
            .await
        })
        .await?;
    tokio::task::yield_now().await; // Explicitly flush any pending async operations
    Ok(())
}

fn all_problems() -> Vec<ProblemSpec> {
    let mut problems = analytic_problems();
    problems.extend(ml_problems());
    problems
}

fn all_optimizers() -> Vec<(String, Arc<dyn Optimizer>)> {
    let mut optimizers = qqn_variants();
    optimizers.extend(lbfgs_variants());
    optimizers.extend(gd_variants());
    optimizers.extend(adam_variants());
    optimizers.extend(trust_region_variants());
    optimizers
}

// #[tokio::test]
#[allow(dead_code)]
async fn test_mnist() -> Result<(), Box<dyn Error + Send + Sync>> {
    init_logging(false)?;
    // Enable no threshold mode for this test
    enable_no_threshold_mode();

    LocalSet::new()
        .run_until(async move { test("results/mnist_", mnist_problems(1000)).await })
        .await?;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    Ok(())
}

#[allow(dead_code)]
async fn test(
    prefix: &str,
    problems: Vec<ProblemSpec>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
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
    )
    .await?;

    run_benchmark(
        &format!("{prefix}qqn_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        qqn_variants(),
        2e-1,
    )
    .await?;

    run_benchmark(
        &format!("{prefix}lbfgs_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        lbfgs_variants(),
        2e-1,
    )
    .await?;

    run_benchmark(
        &format!("{prefix}gd_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        gd_variants(),
        2e-1,
    )
    .await?;

    run_benchmark(
        &format!("{prefix}adam_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        adam_variants(),
        2e-1,
    )
    .await?;

    run_benchmark(
        &format!("{prefix}trust_region_variants_"),
        max_evals,
        num_runs,
        Duration::from_secs(60),
        Some(1),
        problems.clone(),
        trust_region_variants(),
        2e-1,
    )
    .await?;
    Ok(())
}
