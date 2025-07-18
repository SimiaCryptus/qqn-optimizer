//! Mathematical utilities and tensor operations for optimization algorithms.
//!
//! This module provides:
//! - Vector operations (dot product, norms, scaling)
//! - Tensor magnitude computations
//! - Numerical stability utilities
//! - Common mathematical functions for optimization

use anyhow::{anyhow, Result};
use candle_core::{Device, Result as CandleResult, Tensor};
use log::{debug, warn};

/// Create a 1D tensor from a Vec<f64>
pub fn create_1d_tensor(values: &[f64], device: &Device) -> CandleResult<Tensor> {
    Tensor::new(values, device)
}

pub(crate) fn tensors_to_f64(tensors: &[Tensor]) -> CandleResult<Vec<f64>> {
    let mut result = Vec::new();
    for tensor in tensors {
        let values = tensor.flatten_all()?.to_vec1::<f64>()?;
        result.extend(values);
    }
    Ok(result)
}

/// Compute the magnitude (L2 norm) of a vector of tensors
pub fn compute_magnitude(tensors: &[Tensor]) -> CandleResult<f64> {
    if tensors.is_empty() {
        return Ok(0.0);
    }

    // Use compensated summation for better numerical stability
    let mut sum_of_squares = 0.0;
    let mut compensation = 0.0;
    let mut max_abs = 0.0_f64;
    let mut count = 0usize;
    // First pass: find maximum absolute value for scaling
    for tensor in tensors {
        let values = tensor.flatten_all()?.to_vec1::<f64>()?;
        for &val in &values {
            if !val.is_finite() {
                warn!("Tensor contains non-finite value: {}", val);
                return Ok(f64::INFINITY);
            }
            max_abs = max_abs.max(val.abs());
            count += 1;
        }
    }
    // Handle empty tensors
    if count == 0 {
        return Ok(0.0);
    }
    // Use scaling to prevent overflow/underflow
    let scale = if max_abs > 1e100 || (max_abs > 0.0 && max_abs < 1e-100) {
        1.0 / max_abs
    } else {
        1.0
    };

    for tensor in tensors {
        let values = tensor.flatten_all()?.to_vec1::<f64>()?;
        for &val in &values {
            // Kahan summation algorithm
            let scaled_val = val * scale;
            let square = scaled_val * scaled_val;
            let y = square - compensation;
            let t = sum_of_squares + y;
            compensation = (t - sum_of_squares) - y;
            sum_of_squares = t;
        }
    }

    if sum_of_squares.is_nan() {
        warn!("Sum of squares is NaN, returning infinity");
        return Ok(f64::INFINITY);
    }
    if sum_of_squares.is_infinite() {
        warn!("Sum of squares is infinite, returning infinity");
        return Ok(f64::INFINITY);
    }
    if sum_of_squares < 0.0 {
        warn!("Sum of squares is negative due to numerical errors, using absolute value");
        return Ok(sum_of_squares.abs().sqrt());
    }

    // Scale back the result
    Ok(sum_of_squares.sqrt() / scale)
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
        return Err(anyhow!(
            "Vectors must have same length for dot product: {} != {}",
            a.len(),
            b.len()
        ));
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

/// Trait for differentiable functions that can compute both value and gradients
pub trait DifferentiableFunction: Send + Sync {
    /// Evaluate the function at the given point
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64>;
    /// Compute gradients at the given point
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>>;
}

pub fn tensor_from_vec(values: Vec<f64>) -> Tensor {
    Tensor::from_vec(values.clone(), values.len(), &Device::Cpu).unwrap()
}

pub fn tensors_to_vec(tensors: &[Tensor]) -> Vec<f64> {
    tensors
        .iter()
        .flat_map(|t| t.flatten_all().unwrap().to_vec1::<f64>().unwrap())
        .collect()
}

/// Wrapper for separate objective and gradient functions
pub struct SeparateFunctions<F, G>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
    G: Fn(&[Tensor]) -> CandleResult<Vec<Tensor>> + Send + Sync,
{
    objective_fn: F,
    gradient_fn: G,
}

impl<F, G> SeparateFunctions<F, G>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
    G: Fn(&[Tensor]) -> CandleResult<Vec<Tensor>> + Send + Sync,
{
    pub fn new(objective_fn: F, gradient_fn: G) -> Self {
        Self {
            objective_fn,
            gradient_fn,
        }
    }
}
impl<F, G> DifferentiableFunction for SeparateFunctions<F, G>
where
    F: Fn(&[Tensor]) -> CandleResult<f64> + Send + Sync,
    G: Fn(&[Tensor]) -> CandleResult<Vec<Tensor>> + Send + Sync,
{
    fn evaluate(&self, params: &[Tensor]) -> CandleResult<f64> {
        (self.objective_fn)(params)
    }
    fn gradient(&self, params: &[Tensor]) -> CandleResult<Vec<Tensor>> {
        (self.gradient_fn)(params)
    }
}

pub fn log_tensor(tensors: &[Tensor]) {
    for (i, tensor) in tensors.iter().enumerate() {
        match tensor.flatten_all().and_then(|t| t.to_vec1::<f64>()) {
            Ok(values) => {
                debug!(
                    "  Tensor[{}]: shape={:?}, values={:?}",
                    i,
                    tensor.shape(),
                    values
                );
                debug!(
                    "  Tensor[{}]: shape={:?}, dtype={:?}, device={:?}",
                    i,
                    tensor.shape(),
                    tensor.dtype(),
                    tensor.device()
                );
                if values.len() <= 10 {
                    debug!("    Full data: {:?}", values);
                } else {
                    debug!(
                        "    First 5: {:?}, Last 5: {:?}",
                        &values[..5],
                        &values[values.len() - 5..]
                    );
                }

                // Log statistics
                let mean = values.iter().sum::<f64>() / values.len() as f64;
                let variance =
                    values.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
                let min_val = values.iter().fold(f64::INFINITY, |a, &b| a.min(b));
                let max_val = values.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
                let l2_norm = values.iter().map(|x| x * x).sum::<f64>().sqrt();

                debug!(
                    "    Stats: mean={:.6e}, std={:.6e}, min={:.6e}, max={:.6e}, norm={:.6e}",
                    mean,
                    variance.sqrt(),
                    min_val,
                    max_val,
                    l2_norm
                );
            }
            Err(e) => {
                debug!(
                    "  Tensor[{}]: shape={:?}, error reading values: {}",
                    i,
                    tensor.shape(),
                    e
                );
            }
        }
    }
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

/// Check if a value is within a tolerance of zero
pub fn is_zero(value: f64, tolerance: f64) -> bool {
    value.abs() < tolerance
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use candle_core::Device;
    #[test]
    fn test_f64_to_tensors() -> CandleResult<()> {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0];
        let tensors = [create_1d_tensor(&values, &Device::Cpu)?].to_vec();
        assert_eq!(tensors.len(), 1);
        Ok(())
    }
    #[test]
    fn test_tensors_to_f64() -> CandleResult<()> {
        let device = Device::Cpu;
        let tensors = vec![
            Tensor::from_slice(&[1.0, 2.0, 3.0, 4.0], &[2, 2], &device)?,
            Tensor::from_slice(&[5.0, 6.0, 7.0], &[3], &device)?,
        ];
        let values = tensors_to_f64(&tensors)?;
        assert_eq!(values, vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0]);
        Ok(())
    }
    #[test]
    fn test_compute_magnitude_edge_cases() -> CandleResult<()> {
        let device = Device::Cpu;
        // Test empty tensors
        let empty_tensors: Vec<Tensor> = vec![];
        assert_eq!(compute_magnitude(&empty_tensors)?, 0.0);
        // Test with zero values
        let zero_tensors = vec![Tensor::zeros(&[3], candle_core::DType::F64, &device)?];
        assert_eq!(compute_magnitude(&zero_tensors)?, 0.0);
        // Test with very large values (testing overflow prevention)
        let large_values = vec![1e100, 2e100, 3e100];
        let large_tensors = vec![Tensor::from_slice(&large_values, &[3], &device)?];
        let magnitude = compute_magnitude(&large_tensors)?;
        assert!(magnitude.is_finite());
        assert!(magnitude > 0.0);
        Ok(())
    }
    #[test]
    fn test_dot_product_f64() -> Result<()> {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = dot_product_f64(&a, &b)?;
        assert_relative_eq!(result, 32.0, epsilon = 1e-10); // 1*4 + 2*5 + 3*6 = 32
        // Test mismatched lengths
        let c = vec![1.0, 2.0];
        assert!(dot_product_f64(&a, &c).is_err());
        // Test empty vectors
        let empty: Vec<f64> = vec![];
        assert_eq!(dot_product_f64(&empty, &empty)?, 0.0);
        Ok(())
    }
    #[test]
    fn test_compute_parameter_change() -> CandleResult<()> {
        let device = Device::Cpu;
        let p0 = vec![
            Tensor::from_slice(&[1.0, 2.0], &[2], &device)?,
            Tensor::from_slice(&[3.0, 4.0], &[2], &device)?,
        ];
        let p1 = vec![
            Tensor::from_slice(&[2.0, 4.0], &[2], &device)?,
            Tensor::from_slice(&[6.0, 8.0], &[2], &device)?,
        ];
        let change = compute_parameter_change(&p0, &p1)?;
        // Change is sqrt((1^2 + 2^2 + 3^2 + 4^2)) = sqrt(30) ≈ 5.477
        assert_relative_eq!(change, 30.0_f64.sqrt(), epsilon = 1e-10);
        // Test mismatched lengths
        let p2 = vec![Tensor::from_slice(&[1.0], &[1], &device)?];
        assert!(compute_parameter_change(&p0, &p2).is_err());
        // Test no change
        let no_change = compute_parameter_change(&p0, &p0)?;
        assert_relative_eq!(no_change, 0.0, epsilon = 1e-10);
        Ok(())
    }
    #[test]
    fn test_scale_tensors_alias() -> CandleResult<()> {
        let device = Device::Cpu;
        let tensors = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let scaled = scale_tensors(&tensors, 3.0)?;
        let values = scaled[0].to_vec1::<f64>()?;
        assert_relative_eq!(values[0], 3.0, epsilon = 1e-10);
        assert_relative_eq!(values[1], 6.0, epsilon = 1e-10);
        Ok(())
    }
    #[test]
    fn test_combine_tensors_alias() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let b = vec![Tensor::from_slice(&[3.0, 4.0], &[2], &device)?];
        let combined = combine_tensors(&a, &b)?;
        let values = combined[0].to_vec1::<f64>()?;
        assert_relative_eq!(values[0], 4.0, epsilon = 1e-10);
        assert_relative_eq!(values[1], 6.0, epsilon = 1e-10);
        Ok(())
    }
    #[test]
    fn test_dot_product_error_cases() -> CandleResult<()> {
        let device = Device::Cpu;
        // Test mismatched vector lengths
        let a = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let b = vec![
            Tensor::from_slice(&[3.0, 4.0], &[2], &device)?,
            Tensor::from_slice(&[5.0], &[1], &device)?,
        ];
        assert!(dot_product(&a, &b).is_err());
        // Test mismatched tensor shapes
        let c = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let d = vec![Tensor::from_slice(&[3.0, 4.0, 5.0], &[3], &device)?];
        assert!(dot_product(&c, &d).is_err());
        Ok(())
    }
    #[test]
    fn test_vector_operations_errors() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[1.0], &[1], &device)?];
        let b = vec![
            Tensor::from_slice(&[2.0], &[1], &device)?,
            Tensor::from_slice(&[3.0], &[1], &device)?,
        ];
        // Test mismatched lengths for various operations
        assert!(vector_add(&a, &b).is_err());
        assert!(vector_subtract(&a, &b).is_err());
        Ok(())
    }

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
    fn test_is_zero() {
        assert!(is_zero(1e-10, 1e-8));
        assert!(!is_zero(1e-6, 1e-8));
    }
}

pub fn compute_parameter_change(p0: &[Tensor], p1: &[Tensor]) -> CandleResult<f64> {
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
