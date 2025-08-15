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
use qqn_optimizer::optimizers::{GDConfig, GDOptimizer, TrustRegionConfig, TrustRegionOptimizer};
use qqn_optimizer::problem_sets::{analytic_problems, ml_problems, mnist_problems};
use qqn_optimizer::{
    init_logging, AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig,
    LineSearchMethod, MnistOneDnnNeuralNetwork, OptimizationProblem, Optimizer, QQNConfig,
    QQNOptimizer, RosenbrockFunction,
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
                Some(10000), // 1000 samples for a more substantial test
                &[32, 16],   // Two hidden layers: 32 and 16 neurons
                Some(10000), // Batch size of 32
                &mut rand::rngs::StdRng::seed_from_u64(42),
                Some(ActivationType::Logistic),
            )
            .unwrap();
            let dimensions = Some(network.dimension());
            let optimizers: Vec<(String, Arc<dyn Optimizer>)> = vec![
                (
                    "QQN-GoldenSection".to_string(),
                    Arc::new(QQNOptimizer::new(QQNConfig {
                        line_search: LineSearchConfig {
                            method: LineSearchMethod::GoldenSection,
                            c1: 1e-4,
                            c2: 0.9,
                            max_iterations: 30,
                            initial_step: 1.0,
                            min_step: 1e-10,
                            max_step: 10.0,
                            verbose: false,
                            line_bracket_method: 1,
                        },
                        lbfgs_history: 10,
                        epsilon: 1e-6,
                        ..Default::default()
                    })),
                ),
                (
                    "QQN-Bisection-1".to_string(),
                    Arc::new(QQNOptimizer::new(QQNConfig {
                        line_search: LineSearchConfig {
                            method: LineSearchMethod::Bisection,
                            line_bracket_method: 1,
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
                        ..Default::default()
                    })),
                ),
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
                        ..Default::default()
                    })),
                ),
                (
                    "QQN-StrongWolfe".to_string(),
                    Arc::new(QQNOptimizer::new(QQNConfig {
                        line_search: LineSearchConfig {
                            method: LineSearchMethod::StrongWolfe,
                            c1: 1e-4,
                            c2: 0.1,
                            max_iterations: 20,
                            initial_step: 1.0,
                            min_step: 1e-10,
                            max_step: 10.0,
                            verbose: false,
                            line_bracket_method: 1,
                        },
                        lbfgs_history: 10,
                        epsilon: 1e-6,
                        ..Default::default()
                    })),
                ),
                (
                    "QQN-CubicQuadraticInterpolation".to_string(),
                    Arc::new(QQNOptimizer::new(QQNConfig {
                        line_search: LineSearchConfig {
                            method: LineSearchMethod::CubicQuadraticInterpolation,
                            max_iterations: 5,
                            initial_step: 1.0,
                            min_step: 1e-10,
                            max_step: 10.0,
                            verbose: false,
                            line_bracket_method: 1,
                            ..LineSearchConfig::default()
                        },
                        lbfgs_history: 10,
                        epsilon: 1e-6,
                        ..Default::default()
                    })),
                ),
                (
                    "L-BFGS-Aggressive".to_string(),
                    Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                        name: "L-BFGS-Aggressive".to_string(),
                        history_size: 5,
                        max_step_size: 10.0,
                        max_param_change: 10.0,
                        gradient_clip: 0.0,
                        line_search: LineSearchConfig {
                            c1: 1e-3,
                            c2: 0.1,
                            initial_step: 2.0,
                            max_step: 10.0,
                            method: LineSearchMethod::Backtracking,
                            ..LineSearchConfig::default()
                        },
                        epsilon: 1e-6,
                        max_correction_pairs: 5,
                        min_step_size: 1e-12,
                        enable_recovery: false,
                        recovery_patience: 3,
                        verbose: false,
                    })),
                ),
                (
                    "L-BFGS".to_string(),
                    Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                        name: "L-BFGS".to_string(),
                        history_size: 10,
                        line_search: LineSearchConfig {
                            c1: 1e-4,
                            c2: 0.9,
                            initial_step: 1.0,
                            max_step: 2.0,
                            method: LineSearchMethod::StrongWolfe,
                            ..LineSearchConfig::default()
                        },
                        epsilon: 1e-8,
                        max_correction_pairs: 10,
                        max_step_size: 2.0,
                        min_step_size: 1e-16,
                        max_param_change: 1.0,
                        gradient_clip: 1e3,
                        enable_recovery: true,
                        recovery_patience: 5,
                        verbose: false,
                    })),
                ),
                (
                    "L-BFGS-Conservative".to_string(),
                    Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                        name: "L-BFGS-Conservative".to_string(),
                        history_size: 20,
                        line_search: LineSearchConfig {
                            c1: 1e-6, // Very strict Armijo condition
                            c2: 0.99, // Very loose curvature condition
                            initial_step: 0.1,
                            max_step: 1.0,
                            method: LineSearchMethod::StrongWolfe,
                            max_iterations: 50,
                            ..LineSearchConfig::default()
                        },
                        epsilon: 1e-10,
                        max_correction_pairs: 20,
                        max_step_size: 1.0,
                        min_step_size: 1e-20,
                        max_param_change: 0.1,
                        gradient_clip: 1e2,
                        enable_recovery: true,
                        recovery_patience: 10,
                        verbose: false,
                    })),
                ),
                (
                    "L-BFGS-MoreThuente".to_string(),
                    Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                        name: "L-BFGS-MoreThuente".to_string(),
                        history_size: 15,
                        line_search: LineSearchConfig {
                            c1: 1e-4,
                            c2: 0.4,
                            initial_step: 1.0,
                            max_step: 5.0,
                            method: LineSearchMethod::MoreThuente,
                            max_iterations: 30,
                            ..LineSearchConfig::default()
                        },
                        epsilon: 1e-8,
                        max_correction_pairs: 15,
                        max_step_size: 5.0,
                        min_step_size: 1e-14,
                        max_param_change: 2.0,
                        gradient_clip: 1e4,
                        enable_recovery: true,
                        recovery_patience: 7,
                        verbose: false,
                    })),
                ),
                (
                    "L-BFGS-Limited".to_string(),
                    Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                        name: "L-BFGS-Limited".to_string(),
                        history_size: 3,
                        line_search: LineSearchConfig {
                            c1: 1e-3,
                            c2: 0.8,
                            initial_step: 0.5,
                            max_step: 1.5,
                            method: LineSearchMethod::Backtracking,
                            max_iterations: 15,
                            ..LineSearchConfig::default()
                        },
                        epsilon: 1e-6,
                        max_correction_pairs: 3,
                        max_step_size: 1.5,
                        min_step_size: 1e-10,
                        max_param_change: 0.5,
                        gradient_clip: 10.0,
                        enable_recovery: false,
                        recovery_patience: 2,
                        verbose: false,
                    })),
                ),
                (
                    "GD".to_string(),
                    Arc::new(GDOptimizer::new(GDConfig {
                        name: "GD".to_string(),
                        learning_rate: 0.01,
                        momentum: 0.0,
                        weight_decay: 0.0,
                        nesterov: false,
                        max_grad_norm: 10.0,
                        adaptive_lr: true,
                        min_learning_rate: 1e-7,
                        verbose: false,
                    })),
                ),
                (
                    "GD-Momentum".to_string(),
                    Arc::new(GDOptimizer::new(GDConfig {
                        name: "GD-Momentum".to_string(),
                        learning_rate: 0.01,
                        momentum: 0.9,
                        weight_decay: 0.0,
                        nesterov: false,
                        max_grad_norm: 5.0,
                        adaptive_lr: true,
                        min_learning_rate: 1e-8,
                        verbose: false,
                    })),
                ),
                (
                    "GD-Nesterov".to_string(),
                    Arc::new(GDOptimizer::new(GDConfig {
                        name: "GD-Nesterov".to_string(),
                        learning_rate: 0.01,
                        momentum: 0.9,
                        weight_decay: 0.0,
                        nesterov: true,
                        max_grad_norm: 5.0,
                        adaptive_lr: true,
                        min_learning_rate: 1e-8,
                        verbose: false,
                    })),
                ),
                (
                    "Adam-WeightDecay".to_string(),
                    Arc::new(AdamOptimizer::new(
                        "Adam-WeightDecay".to_string(),
                        AdamConfig {
                            learning_rate: 0.003,
                            lr_schedule: "adaptive".to_string(),
                            lr_decay: 0.998,
                            min_learning_rate: 1e-9,
                            gradient_clip: Some(2.0),
                            beta1: 0.9,
                            beta2: 0.999,
                            epsilon: 1e-8,
                            weight_decay: 1e-3,
                            amsgrad: false,
                            max_line_search_iter: 25,
                            verbose: false,
                        },
                    )),
                ),
                (
                    "Adam-Robust".to_string(),
                    Arc::new(AdamOptimizer::autoname(AdamConfig {
                        learning_rate: 0.01,
                        lr_schedule: "exponential".to_string(),
                        lr_decay: 0.99,
                        min_learning_rate: 1e-7,
                        gradient_clip: Some(1.5),
                        beta1: 0.85,
                        beta2: 0.99,
                        epsilon: 1e-6,
                        weight_decay: 5e-4,
                        amsgrad: true,
                        max_line_search_iter: 30,
                        verbose: false,
                    })),
                ),
                (
                    "Trust Region-Adaptive".to_string(),
                    Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
                        name: "Trust Region-Adaptive".to_string(),
                        initial_radius: 0.5,
                        max_radius: 50.0,
                        min_radius: 1e-8,
                        eta_1: 0.15,
                        eta_2: 0.7,
                        gamma_1: 0.3,
                        gamma_2: 2.5,
                        max_subproblem_iterations: 50,
                        subproblem_tolerance: 1e-6,
                        use_cauchy_fallback: true,
                        verbose: false,
                    })),
                ),
                (
                    "Trust Region-Standard".to_string(),
                    Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
                        name: "Trust Region-Standard".to_string(),
                        initial_radius: 1.0,
                        max_radius: 100.0,
                        min_radius: 1e-10,
                        eta_1: 0.2,
                        eta_2: 0.8,
                        gamma_1: 0.5,
                        gamma_2: 3.0,
                        max_subproblem_iterations: 100,
                        subproblem_tolerance: 1e-8,
                        use_cauchy_fallback: false,
                        verbose: false,
                    })),
                ),
                (
                    "Trust Region-Conservative".to_string(),
                    Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
                        name: "Trust Region-Conservative".to_string(),
                        initial_radius: 0.1,
                        max_radius: 10.0,
                        min_radius: 1e-12,
                        eta_1: 0.1,
                        eta_2: 0.5,
                        gamma_1: 0.2,
                        gamma_2: 2.0,
                        max_subproblem_iterations: 30,
                        subproblem_tolerance: 1e-5,
                        use_cauchy_fallback: true,
                        verbose: false,
                    })),
                ),
                (
                    "Trust Region-Aggressive".to_string(),
                    Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
                        name: "Trust Region-Aggressive".to_string(),
                        initial_radius: 2.0,
                        max_radius: 200.0,
                        min_radius: 1e-6,
                        eta_1: 0.25,
                        eta_2: 0.9,
                        gamma_1: 0.8,
                        gamma_2: 4.0,
                        max_subproblem_iterations: 75,
                        subproblem_tolerance: 1e-7,
                        use_cauchy_fallback: false,
                        verbose: false,
                    })),
                ),
                (
                    "Trust Region-Precise".to_string(),
                    Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
                        name: "Trust Region-Precise".to_string(),
                        initial_radius: 0.25,
                        max_radius: 25.0,
                        min_radius: 1e-15,
                        eta_1: 0.05,
                        eta_2: 0.6,
                        gamma_1: 0.1,
                        gamma_2: 1.5,
                        max_subproblem_iterations: 150,
                        subproblem_tolerance: 1e-10,
                        use_cauchy_fallback: true,
                        verbose: false,
                    })),
                ),
            ];
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
                optimizers,
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
