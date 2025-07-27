use qqn_optimizer::benchmarks::functions::OptimizationProblem;
use qqn_optimizer::core::optimizer::Optimizer;
use rand::{Rng, SeedableRng};
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::problem_sets::analytic_problems;
use qqn_optimizer::{init_logging, LineSearchConfig, LineSearchMethod, QQNConfig, QQNOptimizer};

#[tokio::test]
async fn test_comprehensive_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false)?;

    let optimizers: Vec<(String, Arc<dyn Optimizer>)> = vec![
        (
            "QQN-2".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::StrongWolfe,
                    c2: 0.5,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 15,
                ..Default::default()
            })),
        ),
        (
            "QQN-6".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::StrongWolfe,
                    c2: 0.01,
                    c1: 1e-9,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 15,
                ..Default::default()
            })),
        ),
        // (
        //     "Adam".to_string(),
        //     Arc::new(AdamOptimizer::new(AdamConfig {
        //         learning_rate: 0.01,
        //         lr_schedule: "adaptive".to_string(),
        //         verbose: true,
        //         ..Default::default()
        //     })),
        // ),
    ];

    let result = run_benchmark(
        "results/test_",
        10000,
        10,
        Duration::from_secs(60),
        analytic_problems(),
        optimizers
    ).await;

    // Explicitly flush any pending async operations
    tokio::task::yield_now().await;

    result
}

