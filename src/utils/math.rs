//! Mathematical utilities and tensor operations for optimization algorithms.
//!
//! This module provides:
//! - Vector operations (dot product, norms, scaling)
//! - Tensor magnitude computations
//! - Numerical stability utilities
//! - Common mathematical functions for optimization

use anyhow::{anyhow, Result};
use candle_core::{Result as CandleResult, Tensor};

/// Compute the magnitude (L2 norm) of a vector of tensors
pub fn compute_magnitude(tensors: &[Tensor]) -> CandleResult<f64> {
    let mut sum_of_squares = 0.0;

    for tensor in tensors {
        let values = tensor.flatten_all()?.to_vec1::<f64>()?;
        for &val in &values {
            sum_of_squares += val * val;
        }
    }

    Ok(sum_of_squares.sqrt())
}

/// Compute dot product between two tensor vectors
pub fn dot_product(a: &[Tensor], b: &[Tensor]) -> CandleResult<f64> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            "Tensor vectors must have same length for dot product".to_string(),
        ));
    }

    let mut result = 0.0;

    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        let values_a = tensor_a.flatten_all()?.to_vec1::<f64>()?;
        let values_b = tensor_b.flatten_all()?.to_vec1::<f64>()?;

        if values_a.len() != values_b.len() {
            return Err(candle_core::Error::Msg(
                "Tensors must have same number of elements for dot product".to_string(),
            ));
        }

        for (val_a, val_b) in values_a.iter().zip(values_b.iter()) {
            result += val_a * val_b;
        }
    }

    Ok(result)
}
/// Compute dot product between two f64 slices
pub fn dot_product_f64(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(anyhow!("Vectors must have same length for dot product"));
    }
    let result = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    Ok(result)
}

/// Add two tensor vectors element-wise
pub fn vector_add(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            "Tensor vectors must have same length for addition".to_string(),
        ));
    }

    let mut result = Vec::with_capacity(a.len());

    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        result.push(tensor_a.add(tensor_b)?);
    }

    Ok(result)
}

/// Subtract two tensor vectors element-wise (a - b)
pub fn vector_subtract(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            "Tensor vectors must have same length for subtraction".to_string(),
        ));
    }

    let mut result = Vec::with_capacity(a.len());

    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        result.push(tensor_a.sub(tensor_b)?);
    }

    Ok(result)
}

/// Scale a tensor vector by a scalar value
pub fn vector_scale(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    let mut result = Vec::with_capacity(tensors.len());

    for tensor in tensors {
        let scale_tensor = Tensor::new(scale, tensor.device())?;
        result.push(tensor.broadcast_mul(&scale_tensor)?);
    }

    Ok(result)
}

/// Scale tensors by a scalar (alias for vector_scale for consistency)
pub fn scale_tensors(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    vector_scale(tensors, scale)
}

/// Combine two tensor vectors (alias for vector_add for consistency)
pub fn combine_tensors(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    vector_add(a, b)
}

/// Compute the relative difference between two magnitudes
pub fn magnitude_relative_difference(mag1: f64, mag2: f64) -> f64 {
    let max_mag = mag1.max(mag2);
    if max_mag < f64::EPSILON {
        0.0
    } else {
        (mag1 - mag2).abs() / max_mag
    }
}

/// Compute L2 norm of a vector
pub fn norm_l2(values: &[f64]) -> f64 {
    values.iter().map(|x| x * x).sum::<f64>().sqrt()
}

/// Compute L1 norm of a vector
pub fn norm_l1(values: &[f64]) -> f64 {
    values.iter().map(|x| x.abs()).sum()
}

/// Compute infinity norm of a vector
pub fn norm_inf(values: &[f64]) -> f64 {
    values.iter().map(|x| x.abs()).fold(0.0, f64::max)
}

/// Check if all values in a vector are finite
pub fn is_finite(values: &[f64]) -> bool {
    values.iter().all(|x| x.is_finite())
}

/// Clamp vector values to a range
pub fn clamp_vector(values: &[f64], min_val: f64, max_val: f64) -> Vec<f64> {
    values.iter().map(|&x| x.clamp(min_val, max_val)).collect()
}

/// Linear interpolation between two values
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

/// Compute angle between two vectors (in radians)
pub fn angle_between(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(anyhow!("Vectors must have same length"));
    }

    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f64>();
    let norm_a = norm_l2(a);
    let norm_b = norm_l2(b);

    if norm_a < f64::EPSILON || norm_b < f64::EPSILON {
        return Err(anyhow!("Cannot compute angle with zero-length vector"));
    }

    let cos_angle = (dot / (norm_a * norm_b)).clamp(-1.0, 1.0);
    Ok(cos_angle.acos())
}

/// Normalize a vector to unit length
pub fn normalize(values: &[f64]) -> Result<Vec<f64>> {
    let norm = norm_l2(values);
    if norm < f64::EPSILON {
        return Err(anyhow!("Cannot normalize zero-length vector"));
    }

    Ok(values.iter().map(|x| x / norm).collect())
}

/// Compute the Euclidean distance between two points
pub fn euclidean_distance(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(anyhow!("Points must have same dimension"));
    }

    let sum_sq_diff = a
        .iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>();

    Ok(sum_sq_diff.sqrt())
}

/// Check if a value is within a tolerance of zero
pub fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}

/// Safe division that returns zero if denominator is too small
pub fn safe_divide(numerator: f64, denominator: f64, epsilon: f64) -> f64 {
    if denominator.abs() < epsilon {
        0.0
    } else {
        numerator / denominator
    }
}

/// Compute the condition number estimate for numerical stability
pub fn condition_number_estimate(values: &[f64]) -> f64 {
    if values.is_empty() {
        return f64::INFINITY;
    }

    let max_val = values.iter().map(|x| x.abs()).fold(0.0, f64::max);
    let min_val = values
        .iter()
        .map(|x| x.abs())
        .filter(|&x| x > f64::EPSILON)
        .fold(f64::INFINITY, f64::min);

    if min_val == f64::INFINITY || min_val < f64::EPSILON {
        f64::INFINITY
    } else {
        max_val / min_val
    }
}

/// Compute weighted average of values
pub fn weighted_average(values: &[f64], weights: &[f64]) -> Result<f64> {
    if values.len() != weights.len() {
        return Err(anyhow!("Values and weights must have same length"));
    }

    let weighted_sum: f64 = values.iter().zip(weights.iter()).map(|(v, w)| v * w).sum();
    let weight_sum: f64 = weights.iter().sum();

    if weight_sum < f64::EPSILON {
        return Err(anyhow!("Sum of weights must be positive"));
    }

    Ok(weighted_sum / weight_sum)
}

/// Compute moving average with given window size
pub fn moving_average(values: &[f64], window_size: usize) -> Vec<f64> {
    if window_size == 0 || values.is_empty() {
        return Vec::new();
    }

    let mut result = Vec::new();

    for i in 0..values.len() {
        let start = if i + 1 >= window_size {
            i + 1 - window_size
        } else {
            0
        };
        let end = i + 1;
        let window = &values[start..end];
        let avg = window.iter().sum::<f64>() / window.len() as f64;
        result.push(avg);
    }

    result
}

/// Compute exponential moving average
pub fn exponential_moving_average(values: &[f64], alpha: f64) -> Vec<f64> {
    if values.is_empty() || !(0.0..=1.0).contains(&alpha) {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(values.len());
    let mut ema = values[0];
    result.push(ema);

    for &value in &values[1..] {
        ema = alpha * value + (1.0 - alpha) * ema;
        result.push(ema);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use candle_core::Device;

    #[test]
    fn test_compute_magnitude() -> CandleResult<()> {
        let device = Device::Cpu;
        let tensors = vec![Tensor::from_slice(&[3.0, 4.0], &[2], &device)?];

        let magnitude = compute_magnitude(&tensors)?;
        assert_relative_eq!(magnitude, 5.0, epsilon = 1e-10);

        Ok(())
    }

    #[test]
    fn test_dot_product() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let b = vec![Tensor::from_slice(&[3.0, 4.0], &[2], &device)?];

        let result = dot_product(&a, &b)?;
        assert_relative_eq!(result, 11.0, epsilon = 1e-10); // 1*3 + 2*4 = 11

        Ok(())
    }

    #[test]
    fn test_vector_operations() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let b = vec![Tensor::from_slice(&[3.0, 4.0], &[2], &device)?];

        // Test addition
        let sum = vector_add(&a, &b)?;
        let sum_values = sum[0].to_vec1::<f64>()?;
        assert_relative_eq!(sum_values[0], 4.0, epsilon = 1e-10);
        assert_relative_eq!(sum_values[1], 6.0, epsilon = 1e-10);

        // Test subtraction
        let diff = vector_subtract(&a, &b)?;
        let diff_values = diff[0].to_vec1::<f64>()?;
        assert_relative_eq!(diff_values[0], -2.0, epsilon = 1e-10);
        assert_relative_eq!(diff_values[1], -2.0, epsilon = 1e-10);

        // Test scaling
        let scaled = vector_scale(&a, 2.0)?;
        let scaled_values = scaled[0].to_vec1::<f64>()?;
        assert_relative_eq!(scaled_values[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(scaled_values[1], 4.0, epsilon = 1e-10);

        Ok(())
    }

    #[test]
    fn test_magnitude_relative_difference() {
        assert_relative_eq!(
            magnitude_relative_difference(10.0, 8.0),
            0.2,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            magnitude_relative_difference(0.0, 0.0),
            0.0,
            epsilon = 1e-10
        );
        assert_relative_eq!(
            magnitude_relative_difference(5.0, 5.0),
            0.0,
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_norms() {
        let values = vec![3.0, -4.0, 0.0, 5.0];

        assert_relative_eq!(
            norm_l2(&values),
            (9.0 + 16.0 + 0.0 + 25.0_f64).sqrt(),
            epsilon = 1e-10
        );
        assert_relative_eq!(norm_l1(&values), 12.0, epsilon = 1e-10);
        assert_relative_eq!(norm_inf(&values), 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_is_finite() {
        assert!(is_finite(&[1.0, 2.0, 3.0]));
        assert!(!is_finite(&[1.0, f64::NAN, 3.0]));
        assert!(!is_finite(&[1.0, f64::INFINITY, 3.0]));
    }

    #[test]
    fn test_clamp_vector() {
        let values = vec![-2.0, 0.5, 3.0, 10.0];
        let clamped = clamp_vector(&values, 0.0, 5.0);
        assert_eq!(clamped, vec![0.0, 0.5, 3.0, 5.0]);
    }

    #[test]
    fn test_lerp() {
        assert_relative_eq!(lerp(0.0, 10.0, 0.5), 5.0, epsilon = 1e-10);
        assert_relative_eq!(lerp(2.0, 8.0, 0.25), 3.5, epsilon = 1e-10);
    }

    #[test]
    fn test_angle_between() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        let angle = angle_between(&a, &b).unwrap();
        assert_relative_eq!(angle, std::f64::consts::PI / 2.0, epsilon = 1e-10);

        let c = vec![1.0, 0.0];
        let d = vec![1.0, 0.0];
        let angle2 = angle_between(&c, &d).unwrap();
        assert_relative_eq!(angle2, 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_normalize() {
        let values = vec![3.0, 4.0];
        let normalized = normalize(&values).unwrap();
        assert_relative_eq!(normalized[0], 0.6, epsilon = 1e-10);
        assert_relative_eq!(normalized[1], 0.8, epsilon = 1e-10);
        assert_relative_eq!(norm_l2(&normalized), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_euclidean_distance() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        let distance = euclidean_distance(&a, &b).unwrap();
        assert_relative_eq!(distance, 5.0, epsilon = 1e-10);
    }

    #[test]
    fn test_weighted_average() {
        let values = vec![1.0, 2.0, 3.0];
        let weights = vec![1.0, 2.0, 1.0];
        let avg = weighted_average(&values, &weights).unwrap();
        assert_relative_eq!(avg, 2.0, epsilon = 1e-10); // (1*1 + 2*2 + 3*1) / (1+2+1) = 8/4 = 2
    }

    #[test]
    fn test_moving_average() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let ma = moving_average(&values, 3);
        assert_eq!(ma.len(), 5);
        assert_relative_eq!(ma[0], 1.0, epsilon = 1e-10); // [1] -> 1
        assert_relative_eq!(ma[1], 1.5, epsilon = 1e-10); // [1,2] -> 1.5
        assert_relative_eq!(ma[2], 2.0, epsilon = 1e-10); // [1,2,3] -> 2
        assert_relative_eq!(ma[3], 3.0, epsilon = 1e-10); // [2,3,4] -> 3
        assert_relative_eq!(ma[4], 4.0, epsilon = 1e-10); // [3,4,5] -> 4
    }

    #[test]
    fn test_exponential_moving_average() {
        let values = vec![1.0, 2.0, 3.0];
        let ema = exponential_moving_average(&values, 0.5);
        assert_eq!(ema.len(), 3);
        assert_relative_eq!(ema[0], 1.0, epsilon = 1e-10);
        assert_relative_eq!(ema[1], 1.5, epsilon = 1e-10); // 0.5*2 + 0.5*1 = 1.5
        assert_relative_eq!(ema[2], 2.25, epsilon = 1e-10); // 0.5*3 + 0.5*1.5 = 2.25
    }

    #[test]
    fn test_safe_divide() {
        assert_relative_eq!(safe_divide(10.0, 2.0, 1e-8), 5.0, epsilon = 1e-10);
        assert_relative_eq!(safe_divide(10.0, 1e-10, 1e-8), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_is_zero() {
        assert!(is_zero(1e-10, 1e-8));
        assert!(!is_zero(1e-6, 1e-8));
    }
}

pub(crate) fn compute_parameter_change(p0: &[Tensor], p1: &Vec<Tensor>) -> CandleResult<f64> {
    if p0.len() != p1.len() {
        return Err(candle_core::Error::Msg(
            "Parameter vectors must have the same length".to_string(),
        ));
    }

    let mut sum_of_squares = 0.0;
    for (tensor0, tensor1) in p0.iter().zip(p1.iter()) {
        let diff = tensor1.sub(tensor0)?;
        let values = diff.flatten_all()?.to_vec1::<f64>()?;
        for &val in &values {
            sum_of_squares += val * val;
        }
    }

    Ok(sum_of_squares.sqrt())
}
/// Compute magnitude relative difference as defined in the QQN paper
/// ρ = |‖d_LBFGS‖ - ‖g‖| / (‖d_LBFGS‖ + ‖g‖)
pub fn magnitude_relative_difference_qqn(grad_magnitude: f64, lbfgs_magnitude: f64) -> f64 {
    // Handle edge cases
    if !grad_magnitude.is_finite() || !lbfgs_magnitude.is_finite() {
        return f64::INFINITY;
    }

    let denominator = grad_magnitude + lbfgs_magnitude;
    if denominator < f64::EPSILON {
        0.0 // Both magnitudes are essentially zero
    } else {
        (lbfgs_magnitude - grad_magnitude).abs() / denominator
    }
}