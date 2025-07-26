use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::problem_sets::mnist_problems;
use qqn_optimizer::{init_logging, AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, OptimizationProblem, Optimizer, QQNConfig, QQNOptimizer};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false)?;

    let optimizers: Vec<(String, Arc<dyn Optimizer>)> = vec![
        (
            "QQN-Backtracking-Fast".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Backtracking,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 15,
                gradient_scale_factor: 1000.0,
                ..Default::default()
            })),
        ),
        (
            "QQN-Bisection-Fast".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Bisection,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 15,
                gradient_scale_factor: 1000.0,
                ..Default::default()
            })),
        ),
        (
            "QQN-MoreThuente-Fast".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::MoreThuente,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 15,
                gradient_scale_factor: 1000.0,
                ..Default::default()
            })),
        ),
        (
            "QQN-StrongWolfe-Fast".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::StrongWolfe,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 30,
                gradient_scale_factor: 1000.0,
                ..Default::default()
            })),
        ),
        (
            "QQN-StrongWolfe".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::StrongWolfe,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 30,
                ..Default::default()
            })),
        ),
        // (
        //     "QQN-Backtracking".to_string(),
        //     Arc::new(QQNOptimizer::new(QQNConfig {
        //         line_search: LineSearchConfig {
        //             method: LineSearchMethod::Backtracking,
        //             max_iterations: 5,
        //             ..LineSearchConfig::default()
        //         },
        //         lbfgs_history: 15,
        //         gradient_scale_factor: 1.0,
        //         ..Default::default()
        //     })),
        // ),
        // (
        //     "QQN-CubicQuadraticInterpolation".to_string(),
        //     Arc::new(QQNOptimizer::new(QQNConfig {
        //         line_search: LineSearchConfig {
        //             method: LineSearchMethod::CubicQuadraticInterpolation,
        //             max_iterations: 5,
        //             ..LineSearchConfig::default()
        //         },
        //         min_step_persist: 5e-1,
        //         lbfgs_history: 30,
        //         gradient_scale_factor: 1.0,
        //         ..Default::default()
        //     })),
        // ),
        (
            "L-BFGS".to_string(),
            Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                ..LBFGSConfig::lax()
            })),
        ),
        (
            "Adam-Fast".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "cosine".to_string(),
                ..AdamConfig::deep_learning()
            })),
        ),
        (
            "Adam".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.005,
                lr_schedule: "cosine".to_string(),
                ..AdamConfig::deep_learning()
            })),
        ),
    ];

    let result = run_benchmark(
        "results/test_",
        500,
        2,
        Duration::from_secs(60),
        mnist_problems(1000),
        optimizers
    ).await;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    result
}

