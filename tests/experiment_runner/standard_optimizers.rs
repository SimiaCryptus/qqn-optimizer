use qqn_optimizer::core::{GDConfig, GDOptimizer};
use qqn_optimizer::{AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, Optimizer, QQNConfig, QQNOptimizer};

pub fn standard_optimizers() -> Vec<(String, Box<dyn Optimizer>)> {
    vec![
        // QQN variants
        (
            "QQN-Default".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig::default())),
        ),
        (
            "QQN-Linear".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                linear_mode: true,
                ..Default::default()
            })),
        ),
        (
            "QQN-Backtracking".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Backtracking,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-Backtracking-Adaptive".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::Backtracking,
                    c1: 1e-4,
                    max_iterations: 100,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 10,
                ..Default::default()
            })),
        ),
        (
            "QQN-Backtracking-Hybrid".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
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
            "QQN-SimpleBracket".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    line_bracket_method: 2,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-StrongWolfe".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::StrongWolfe,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-CubicQuadraticInterpolation".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::CubicQuadraticInterpolation,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-GoldenSection".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::GoldenSection,
                    ..LineSearchConfig::default()
                },
                ..Default::default()
            })),
        ),
        (
            "QQN-MoreThuente".to_string(),
            Box::new(QQNOptimizer::new(QQNConfig {
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
            Box::new(LBFGSOptimizer::new(LBFGSConfig::default())),
        ),
        (
            "L-BFGS-Aggressive".to_string(),
            Box::new(LBFGSOptimizer::new(LBFGSConfig {
                history_size: 5,
                max_step_size: 10.0,
                max_param_change: 10.0,
                gradient_clip: 0.0,
                ..Default::default()
            })),
        ),
        (
            "L-BFGS-Hybrid".to_string(),
            Box::new(LBFGSOptimizer::new(LBFGSConfig {
                history_size: 12,
                max_step_size: 5.0,
                max_param_change: 2.0,
                gradient_clip: 50.0,
                ..Default::default()
            })),
        ),
        (
            "L-BFGS-Adaptive-Memory".to_string(),
            Box::new(LBFGSOptimizer::new(LBFGSConfig {
                history_size: 15,
                max_step_size: 2.0,
                max_param_change: 1.0,
                gradient_clip: 25.0,
                ..Default::default()
            })),
        ),

        // Gradient Descent variants
        (
            "GD".to_string(),
            Box::new(GDOptimizer::new(Default::default())),
        ),
        (
            "GD-Conservative".to_string(),
            Box::new(GDOptimizer::new(GDConfig {
                learning_rate: 0.001,
                momentum: 0.95,
                max_grad_norm: 100.0,
                adaptive_lr: false,
                nesterov: true,
                verbose: false,
                ..Default::default()
            })),
        ),
        (
            "GD-Momentum".to_string(),
            Box::new(GDOptimizer::new(GDConfig {
                momentum: 0.9,
                ..Default::default()
            })),
        ),

        // Adam variants
        (
            "Adam".to_string(),
            Box::new(AdamOptimizer::new(Default::default())),
        ),
        (
            "Adam-Fast".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "constant".to_string(),
                ..Default::default()
            })),
        ),
        (
            "Adam-Fast-Conservative".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.05,
                lr_schedule: "constant".to_string(),
                gradient_clip: Some(1.0),
                ..Default::default()
            })),
        ),
        (
            "Adam-Fast-Aggressive".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.125,
                lr_schedule: "constant".to_string(),
                beta1: 0.85,
                ..Default::default()
            })),
        ),
        (
            "Adam-Fast-Adaptive".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "adaptive".to_string(),
                gradient_clip: Some(2.0),
                ..Default::default()
            })),
        ),
        (
            "Adam-Fast-Momentum".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "constant".to_string(),
                beta1: 0.95,
                beta2: 0.999,
                ..Default::default()
            })),
        ),
        (
            "Adam-Fast-Regularized".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "constant".to_string(),
                weight_decay: 0.01,
                gradient_clip: Some(1.0),
                ..Default::default()
            })),
        ),
        (
            "Adam-DeepLearning".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig::deep_learning())),
        ),
        (
            "Adam-StrictGradientClip".to_string(),
            Box::new(AdamOptimizer::new(AdamConfig {
                gradient_clip: Some(0.1),
                learning_rate: 0.001,
                ..Default::default()
            })),
        ),
    ]
}