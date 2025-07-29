use std::sync::Arc;
use crate::{AdamConfig, AdamOptimizer, LBFGSConfig, LBFGSOptimizer, LineSearchConfig, LineSearchMethod, Optimizer, QQNConfig, QQNOptimizer};
use crate::optimizers::{GDConfig, GDOptimizer, TrustRegionConfig, TrustRegionOptimizer};

pub fn qqn_variants() -> Vec<(String, Arc<dyn Optimizer>)> {
    vec![
        // Top performers from benchmark
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
                    max_iterations: 0,
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
    ]
}

pub fn lbfgs_variants() -> Vec<(String, Arc<dyn Optimizer>)> {
    vec![
        (
            "L-BFGS-Aggressive".to_string(),
            Arc::new(LBFGSOptimizer::new(LBFGSConfig {
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
                history_size: 20,
                line_search: LineSearchConfig {
                    c1: 1e-6,  // Very strict Armijo condition
                    c2: 0.99,  // Very loose curvature condition
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
    ]
}

pub fn gd_variants() -> Vec<(String, Arc<dyn Optimizer>)> {
    vec![
        (
            "GD".to_string(),
            Arc::new(GDOptimizer::new(GDConfig {
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
            "GD-WeightDecay".to_string(),
            Arc::new(GDOptimizer::new(GDConfig {
                learning_rate: 0.005,
                momentum: 0.8,
                weight_decay: 1e-4,
                nesterov: false,
                max_grad_norm: 10.0,
                adaptive_lr: true,
                min_learning_rate: 1e-9,
                verbose: false,
            })),
        ),
        (
            "GD-AdaptiveMomentum".to_string(),
            Arc::new(GDOptimizer::new(GDConfig {
                learning_rate: 0.02,
                momentum: 0.95,
                weight_decay: 1e-5,
                nesterov: true,
                max_grad_norm: 2.0,
                adaptive_lr: true,
                min_learning_rate: 1e-10,
                verbose: false,
            })),
        ),
    ]
}

pub fn adam_variants() -> Vec<(String, Arc<dyn Optimizer>)> {
    vec![
        (
            "Adam-Fast".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.1,
                lr_schedule: "constant".to_string(),
                lr_decay: 0.995,
                min_learning_rate: 1e-6,
                gradient_clip: Some(10.0),
                beta1: 0.9,
                beta2: 0.999,
                epsilon: 1e-8,
                weight_decay: 0.0,
                amsgrad: false,
                max_line_search_iter: 20,
                verbose: false,
            })),
        ),
        (
            "Adam".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.001,
                lr_schedule: "adaptive".to_string(),
                gradient_clip: Some(1.0),
                lr_decay: 0.999,
                min_learning_rate: 1e-8,
                beta1: 0.9,
                beta2: 0.999,
                epsilon: 1e-8,
                weight_decay: 0.0,
                amsgrad: false,
                max_line_search_iter: 20,
                verbose: false,
            })),
        ),
        (
            "Adam-AMSGrad".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
                learning_rate: 0.001,
                lr_schedule: "constant".to_string(),
                lr_decay: 0.999,
                min_learning_rate: 1e-10,
                gradient_clip: Some(5.0),
                beta1: 0.9,
                beta2: 0.999,
                epsilon: 1e-8,
                weight_decay: 1e-4,
                amsgrad: true,
                max_line_search_iter: 15,
                verbose: false,
            })),
        ),
        (
            "Adam-WeightDecay".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
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
            })),
        ),
        (
            "Adam-Robust".to_string(),
            Arc::new(AdamOptimizer::new(AdamConfig {
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
    ]
}

pub fn trust_region_variants() -> Vec<(String, Arc<dyn Optimizer>)> {
    vec![
        (
            "Trust Region-Adaptive".to_string(),
            Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
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
            }))
        ),
        ("Trust Region-Standard".to_string(),
         Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
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
         }))),
        ("Trust Region-Conservative".to_string(),
         Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
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
        ("Trust Region-Aggressive".to_string(),
         Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
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
         }))),
        ("Trust Region-Precise".to_string(),
         Arc::new(TrustRegionOptimizer::new(TrustRegionConfig {
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
         }))),
    ]
}
// Add a new function for comprehensive testing
pub fn benchmark_optimizers() -> Vec<(String, Arc<dyn Optimizer>)> {
    let mut optimizers: Vec<(String, Arc<dyn Optimizer>)> = vec![];
    // Add top QQN performers
    let vec1: Vec<(String, Arc<dyn Optimizer>)> = vec![
        (
            "QQN-GoldenSection".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::GoldenSection,
                    c1: 1e-4,
                    c2: 0.9,
                    max_iterations: 30,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 10,
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
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 10,
                ..Default::default()
            })),
        ),
        (
            "QQN-CubicQuadraticInterpolation".to_string(),
            Arc::new(QQNOptimizer::new(QQNConfig {
                line_search: LineSearchConfig {
                    method: LineSearchMethod::CubicQuadraticInterpolation,
                    c1: 1e-4,
                    c2: 0.1,
                    max_iterations: 25,
                    ..LineSearchConfig::default()
                },
                lbfgs_history: 10,
                ..Default::default()
            })),
        ),
    ];
    optimizers.extend(vec1);
    // Add best L-BFGS variant
    optimizers.push((
        "L-BFGS-Aggressive".to_string(),
        Arc::new(LBFGSOptimizer::new(LBFGSConfig {
            history_size: 5,
            max_step_size: 10.0,
            line_search: LineSearchConfig {
                c1: 1e-3,
                c2: 0.1,
                initial_step: 2.0,
                max_step: 10.0,
                method: LineSearchMethod::Backtracking,
                ..LineSearchConfig::default()
            },
            ..Default::default()
        })),
    ));
    // Add best Adam variant
    optimizers.push((
        "Adam-Fast".to_string(),
        Arc::new(AdamOptimizer::new(AdamConfig {
            learning_rate: 0.1,
            lr_schedule: "constant".to_string(),
            gradient_clip: Some(10.0),
            ..Default::default()
        })),
    ));
    // Add baseline GD
    optimizers.push((
        "GD-WeightDecay".to_string(),
        Arc::new(GDOptimizer::new(GDConfig {
            learning_rate: 0.005,
            momentum: 0.8,
            weight_decay: 1e-4,
            ..Default::default()
        })),
    ));
    optimizers
}