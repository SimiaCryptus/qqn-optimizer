//! Mathematical utilities and tensor operations for optimization algorithms.
//!
//! This module provides:
//! - Vector operations (dot product, norms, scaling)
//! - Tensor magnitude computations
//! - Numerical stability utilities
//! - Common mathematical functions for optimization

use luminal::prelude::{Graph, GraphTensor, Shape};

/// Trait for building compute graphs for objective functions
pub trait GraphFunction {
    /// Constructs a compute graph for the objective function.
    ///
    /// # Arguments
    /// * `cx` - The graph context to build upon
    /// * `params` - The input parameter tensors
    ///
    /// # Returns
    /// * The loss tensor (scalar)
    fn build<S: Shape>(&self, cx: &mut Graph, params: &[GraphTensor<S>]) -> GraphTensor<S>;
}


/// Compute dot product between two f32 slices
pub fn dot_product(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

/// Compute L2 norm of an f32 slice
pub fn norm(a: &[f32]) -> f32 {
    dot_product(a, a).sqrt()
}

/// Subtract two f32 slices: a - b
pub fn sub(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x - y).collect()
}

/// Add two f32 slices: a + b
pub fn add(a: &[f32], b: &[f32]) -> Vec<f32> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

/// Scale an f32 slice: a * s
pub fn scale(a: &[f32], s: f32) -> Vec<f32> {
    a.iter().map(|x| x * s).collect()
}