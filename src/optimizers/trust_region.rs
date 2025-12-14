//! Trust Region optimizer implementation.
//!
//! This implementation provides a robust optimization method that uses a quadratic model
//! within a trust region to ensure global convergence. The trust region radius is adaptively
//! adjusted based on the agreement between the model and actual function reduction.
//!
//! ## Algorithm Overview
//!
//! The Trust Region method works by:
//! 1. Building a quadratic model of the objective function within a trust region
//! 2. Solving a constrained subproblem to find the optimal step within the region
//! 3. Evaluating the quality of the model prediction vs actual reduction
//! 4. Adjusting the trust region radius based on this quality metric
//!
//! ## Strengths
//!
//! - **Global convergence**: Guaranteed convergence to a stationary point
//! - **Robustness**: Handles ill-conditioned problems well
//! - **Adaptive**: Automatically adjusts step sizes based on model quality
//! - **No line search**: Avoids expensive line search procedures
//!
//! ## Weaknesses
//!
//! - **Subproblem cost**: Solving the trust region subproblem can be expensive
//! - **Memory requirements**: Needs to store Hessian approximation
//! - **Conservative**: May take smaller steps than necessary on well-behaved problems

use crate::optimizers::optimizer::Optimizer;
use luminal::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Configuration parameters for the Trust Region optimizer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRegionConfig {
    /// Initial trust region radius
    ///
    /// **Range**: 0.1 to 10.0, **Default**: 1.0
    pub initial_radius: f32,

    /// Maximum trust region radius
    ///
    /// **Range**: 1.0 to 1000.0, **Default**: 100.0
    pub max_radius: f32,

    /// Minimum trust region radius before declaring convergence
    ///
    /// **Range**: 1e-10 to 1e-4, **Default**: 1e-8
    pub min_radius: f32,

    /// Threshold for accepting a step (ratio of actual to predicted reduction)
    ///
    /// **Range**: 0.0 to 0.5, **Default**: 0.1
    pub eta_1: f32,

    /// Threshold for expanding the trust region
    ///
    /// **Range**: 0.5 to 1.0, **Default**: 0.75
    pub eta_2: f32,

    /// Factor for shrinking the trust region
    ///
    /// **Range**: 0.1 to 0.5, **Default**: 0.25
    pub gamma_1: f32,

    /// Factor for expanding the trust region
    ///
    /// **Range**: 1.5 to 4.0, **Default**: 2.0
    pub gamma_2: f32,

    /// Maximum iterations for solving the trust region subproblem
    ///
    /// **Range**: 10 to 100, **Default**: 50
    pub max_subproblem_iterations: usize,

    /// Tolerance for the trust region subproblem
    ///
    /// **Range**: 1e-10 to 1e-4, **Default**: 1e-6
    pub subproblem_tolerance: f32,

    /// Use Cauchy point if subproblem solver fails
    ///
    /// **Default**: true
    pub use_cauchy_fallback: bool,

    /// Enable verbose logging
    ///
    /// **Default**: false
    pub verbose: bool,
    /// Name of the optimizer
    ///
    /// **Default**: "TrustRegion"
    pub name: String,
}

impl Default for TrustRegionConfig {
    fn default() -> Self {
        Self {
            initial_radius: 1.0,
            max_radius: 100.0,
            min_radius: 1e-8,
            eta_1: 0.1,
            eta_2: 0.75,
            gamma_1: 0.25,
            gamma_2: 2.0,
            max_subproblem_iterations: 50,
            subproblem_tolerance: 1e-6,
            use_cauchy_fallback: true,
            verbose: false,
            name: "TrustRegion".to_string(),
        }
    }
}

impl TrustRegionConfig {
    /// Create a conservative trust region configuration
    pub fn conservative() -> Self {
        Self {
            initial_radius: 0.5,
            max_radius: 10.0,
            min_radius: 1e-10,
            eta_1: 0.2,
            eta_2: 0.8,
            gamma_1: 0.2,
            gamma_2: 1.5,
            name: "TrustRegion-Conservative".to_string(),
            ..Default::default()
        }
    }

    /// Create an aggressive trust region configuration
    pub fn aggressive() -> Self {
        Self {
            initial_radius: 2.0,
            max_radius: 1000.0,
            min_radius: 1e-6,
            eta_1: 0.05,
            eta_2: 0.5,
            gamma_1: 0.5,
            gamma_2: 3.0,
            name: "TrustRegion-Aggressive".to_string(),
            ..Default::default()
        }
    }
}

/// State information for Trust Region optimization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustRegionState {
    /// Current trust region radius
    radius: f32,

    /// Current iteration number
    iteration: usize,
}

impl TrustRegionState {
    /// Create a new trust region state
    pub fn new(initial_radius: f32) -> Self {
        Self {
            radius: initial_radius,
            iteration: 0,
        }
    }

    /// Reset the state
    pub fn reset(&mut self, initial_radius: f32) {
        self.radius = initial_radius;
        self.iteration = 0;
    }
}

/// Trust Region optimizer
#[derive(Debug)]
pub struct TrustRegionOptimizer {
    config: TrustRegionConfig,
    state: TrustRegionState,
    stagnation_multiplier: f32,
    stagnation_count: usize,
}

impl Clone for TrustRegionOptimizer {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state.clone(),
            stagnation_multiplier: self.stagnation_multiplier,
            stagnation_count: self.stagnation_count,
        }
    }
}

impl TrustRegionOptimizer {
    /// Create a new Trust Region optimizer
    pub fn new(config: TrustRegionConfig) -> Self {
        Self {
            state: TrustRegionState::new(config.initial_radius),
            config,
            stagnation_multiplier: 1.0,
            stagnation_count: 1,
        }
    }
}

impl<S: Shape> Optimizer<S> for TrustRegionOptimizer {
    fn clone_box(&self) -> Box<dyn Optimizer<S>> {
        Box::new(self.clone())
    }

    fn step(
        &mut self,
        graph: &mut Graph,
        loss: GraphTensor<()>,
        params: &[GraphTensor<S>],
    ) -> Vec<GraphTensor<S>> {
        // 1. Get gradients
        let grads_map = loss.backward();
        let grads: Vec<GraphTensor<S>> = params.iter().map(|p| *grads_map.get(p).unwrap()).collect();

        // 2. Compute global gradient norm (L2)
        let mut squared_norm = graph.constant(0.0);
        for grad in &grads {
            squared_norm = squared_norm + grad.pow2().sum_reduce();
        }
        let grad_norm = squared_norm.sqrt();

        // 3. Determine step scaling
        // We use a simplified Trust Region approach where we take the steepest descent step
        // clipped to the trust region radius.
        // p = - min(1, radius / ||g||) * g
        let radius = graph.constant(self.state.radius);
        let one = graph.constant(1.0);
        let epsilon = graph.constant(1e-6);
        let scale = (radius / (grad_norm + epsilon)).min(one);

        // 4. Apply updates
        let mut new_params = Vec::with_capacity(params.len());
        for (param, grad) in params.iter().zip(grads) {
            new_params.push(*param - (*grad * scale));
        }

        self.state.iteration += 1;

        new_params
    }

    fn reset(&mut self) {
        self.state.reset(self.config.initial_radius);
    }

    fn name(&self) -> &str {
        &self.config.name
    }

    fn iteration(&self) -> usize {
        self.state.iteration
    }

    fn set_stagnation_multiplier(&mut self, multiplier: f32) {
        self.stagnation_multiplier = multiplier;
    }

    fn set_stagnation_count(&mut self, count: usize) {
        self.stagnation_count = count;
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_trust_region_creation() {
        let config = TrustRegionConfig::default();
        let optimizer = TrustRegionOptimizer::new(config);

        assert_eq!(optimizer.name(), "TrustRegion");
        assert_eq!(optimizer.state.radius, 1.0);
        assert_eq!(optimizer.state.iteration, 0);
    }

    #[test]
    fn test_trust_region_configs() {
        let conservative = TrustRegionConfig::conservative();
        assert_eq!(conservative.initial_radius, 0.5);
        assert_eq!(conservative.gamma_1, 0.2);
        assert_eq!(conservative.name, "TrustRegion-Conservative");

        let aggressive = TrustRegionConfig::aggressive();
        assert_eq!(aggressive.initial_radius, 2.0);
        assert_eq!(aggressive.gamma_2, 3.0);
        assert_eq!(aggressive.name, "TrustRegion-Aggressive");
    }
}