use qqn_optimizer::core::{GDConfig, GDOptimizer};
use qqn_optimizer::{AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, Optimizer, QQNConfig, QQNOptimizer};
use std::sync::Arc;

pub fn standard_optimizers() -> Vec<(String, Arc<dyn Optimizer>)> {
    vec![
        // QQN variants
        (
            "QQN-Bisection-1".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Bisection,
                    line_bracket_method: 1,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-Bisection-2".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Bisection,
                    line_bracket_method: 2,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-Backtracking".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Backtracking,
                    c1: 1e-3,
                    c2: 0.9,
                    max_iterations: 75,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 15,
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
                ..Default::default()
            })),
        ),
        (
            "QQN-CubicQuadraticInterpolation".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::CubicQuadraticInterpolation,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-GoldenSection".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::GoldenSection,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-MoreThuente".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::MoreThuente,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),

        // L-BFGS variants
        (
            "L-BFGS".to_string(),
            Arc::new(LBFGSOptimizer::new(LBFGSConfig::default())),
        ),
        (
            "L-BFGS-Aggressive".to_string(),
            Arc::new(LBFGSOptimizer::new(LBFGSConfig {
                history_size: 5,
                max_step_size: 10.0,
                max_param_change: 10.0,
                gradient_clip: 0.0,
                ..Default::default()
            })),
        ),

        // Gradient Descent variants
        (
            "GD".to_string(),
            Arc::new(GDOptimizer::new(Default::default())),
        ),

        // Adam variants
        (
            "Adam-Fast".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "constant".to_string(),
                ..Default::default()
            })),
        ),
        (
            "Adam-Fast-Conservative".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.05,
                lr_schedule: "constant".to_string(),
                gradient_clip: Some(1.0),
                ..Default::default()
            })),
        ),
    ]
}