fn main() {
    println!("Hello, world!");
}

//! Mathematical utility functions for optimization algorithms.
use crate::core::OptResult;
/// Compute the magnitude (L2 norm) of a vector
pub fn compute_magnitude(vector: &[f64]) -> f64 {
    vector.iter().map(|x| x * x).sum::<f64>().sqrt()
}
/// Compute the relative difference between two magnitudes
pub fn magnitude_relative_difference(mag1: f64, mag2: f64) -> f64 {
    if mag2.abs() < f64::EPSILON {
        if mag1.abs() < f64::EPSILON {
            0.0
        } else {
            f64::INFINITY
        }
    } else {
        (mag1 - mag2).abs() / mag2.abs()
    }
}
/// Scale tensors by a scalar value
pub fn scale_tensors(tensors: &mut [f64], scale: f64) {
    for tensor in tensors.iter_mut() {
        *tensor *= scale;
    }
}
/// Combine two tensors with given weights
pub fn combine_tensors(a: &[f64], b: &[f64], weight_a: f64, weight_b: f64) -> OptResult<Vec<f64>> {
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Tensor dimensions don't match: {} vs {}", a.len(), b.len()));
    }
    Ok(a.iter()
        .zip(b.iter())
        .map(|(x, y)| weight_a * x + weight_b * y)
        .collect())
}
/// Compute dot product of two vectors
pub fn tensor_dot_product(a: &[f64], b: &[f64]) -> OptResult<f64> {
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Vector dimensions don't match: {} vs {}", a.len(), b.len()));
    }
    Ok(a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
}
/// Alias for tensor_dot_product for backward compatibility
pub fn dot_product(a: &[f64], b: &[f64]) -> OptResult<f64> {
    tensor_dot_product(a, b)
}
/// Normalize a vector to unit length
pub fn normalize_vector(vector: &mut [f64]) -> OptResult<f64> {
    let magnitude = compute_magnitude(vector);
    if magnitude < f64::EPSILON {
        return Err(anyhow::anyhow!("Cannot normalize zero vector"));
    }
    for component in vector.iter_mut() {
        *component /= magnitude;
    }
    Ok(magnitude)
}
/// Compute the parameter change magnitude
pub fn compute_parameter_change(old_params: &[f64], new_params: &[f64]) -> OptResult<f64> {
    if old_params.len() != new_params.len() {
        return Err(anyhow::anyhow!("Parameter vectors have different lengths"));
    }
    let diff: Vec<f64> = old_params.iter()
        .zip(new_params.iter())
        .map(|(old, new)| new - old)
        .collect();
    Ok(compute_magnitude(&diff))
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_magnitude() {
        let vector = vec![3.0, 4.0];
        assert_eq!(compute_magnitude(&vector), 5.0);
        let zero_vector = vec![0.0, 0.0];
        assert_eq!(compute_magnitude(&zero_vector), 0.0);
    }
    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = dot_product(&a, &b).unwrap();
        assert_eq!(result, 32.0); // 1*4 + 2*5 + 3*6 = 32
    }
    #[test]
    fn test_normalize_vector() {
        let mut vector = vec![3.0, 4.0];
        let magnitude = normalize_vector(&mut vector).unwrap();
        assert_eq!(magnitude, 5.0);
        assert!((vector[0] - 0.6).abs() < 1e-10);
        assert!((vector[1] - 0.8).abs() < 1e-10);
    }
    #[test]
    fn test_combine_tensors() {
        let a = vec![1.0, 2.0];
        let b = vec![3.0, 4.0];
        let result = combine_tensors(&a, &b, 0.5, 0.5).unwrap();
        assert_eq!(result, vec![2.0, 3.0]);
    }
}
use candle_core::{Tensor, Result as CandleResult};
/// Compute parameter change between current and previous parameters
pub fn compute_parameter_change(current: &[Tensor], previous: &[Tensor]) -> CandleResult<f64> {
    let mut total_change = 0.0;
    for (curr, prev) in current.iter().zip(previous.iter()) {
        let diff = curr.sub(prev)?;
        let change = compute_magnitude(&[diff])?;
        total_change += change * change;
    }
    Ok(total_change.sqrt())
}
 use candle_core::{Device, Result as CandleResult, Tensor};
 /// Scale all tensors in a vector by a scalar value
 pub fn scale_tensors(tensors: &[Tensor], scalar: f64) -> CandleResult<Vec<Tensor>> {
     tensors.iter()
         .map(|t| {
             let scalar_tensor = Tensor::new(scalar, t.device())?;
             t.broadcast_mul(&scalar_tensor)
         })
         .collect()
 }
 /// Combine two tensor vectors element-wise
 pub fn combine_tensors(tensors1: &[Tensor], tensors2: &[Tensor]) -> CandleResult<Vec<Tensor>> {
     tensors1.iter()
         .zip(tensors2.iter())
         .map(|(t1, t2)| t1.add(t2))
         .collect()
 }
 /// Compute the magnitude (L2 norm) of a tensor vector
 pub fn compute_magnitude(tensors: &[Tensor]) -> CandleResult<f64> {
     let mut sum = 0.0;
     for tensor in tensors {
         let squared = tensor.sqr()?;
         let sum_tensor = squared.sum_all()?;
         sum += sum_tensor.to_scalar::<f64>()?;
     }
     Ok(sum.sqrt())
 }
 /// Compute relative difference between two magnitudes
 pub fn magnitude_relative_difference(mag1: f64, mag2: f64) -> f64 {
     if mag2 == 0.0 {
         if mag1 == 0.0 {
             0.0
         } else {
             f64::INFINITY
         }
     } else {
         (mag1 - mag2).abs() / mag2
     }
 }
 /// Compute dot product between two tensor vectors
 pub fn dot_product(tensors1: &[Tensor], tensors2: &[Tensor]) -> CandleResult<f64> {
     let mut sum = 0.0;
     for (t1, t2) in tensors1.iter().zip(tensors2.iter()) {
         let product = t1.mul(t2)?;
         let sum_tensor = product.sum_all()?;
         sum += sum_tensor.to_scalar::<f64>()?;
     }
     Ok(sum)
 }
 /// Add two tensor vectors element-wise
 pub fn vector_add(tensors1: &[Tensor], tensors2: &[Tensor]) -> CandleResult<Vec<Tensor>> {
     tensors1.iter()
         .zip(tensors2.iter())
         .map(|(t1, t2)| t1.add(t2))
         .collect()
 }
 /// Subtract two tensor vectors element-wise
 pub fn vector_subtract(tensors1: &[Tensor], tensors2: &[Tensor]) -> CandleResult<Vec<Tensor>> {
     tensors1.iter()
         .zip(tensors2.iter())
         .map(|(t1, t2)| t1.sub(t2))
         .collect()
 }
 /// Scale a tensor vector by a scalar
 pub fn vector_scale(tensors: &[Tensor], scalar: f64) -> CandleResult<Vec<Tensor>> {
     scale_tensors(tensors, scalar)
 }
//! Mathematical utilities for tensor operations and numerical computations.
use candle_core::{Device, Result as CandleResult, Tensor};
use crate::core::OptResult;
/// Compute the L2 magnitude (norm) of a tensor vector.
pub fn compute_magnitude(tensors: &[Tensor]) -> CandleResult<f64> {
    let mut sum = 0.0;
    for tensor in tensors {
        let flat = tensor.flatten_all()?;
        let values = flat.to_vec1::<f64>()?;
        for val in values {
            sum += val * val;
        }
    }
    Ok(sum.sqrt())
}
/// Compute the relative difference between two magnitudes.
/// Returns |a - b| / max(|a|, |b|)
pub fn magnitude_relative_difference(a: f64, b: f64) -> f64 {
    let diff = (a - b).abs();
    let max_mag = a.abs().max(b.abs());
    if max_mag > 0.0 {
        diff / max_mag
    } else {
        0.0
    }
}
/// Scale a vector of tensors by a scalar factor.
pub fn scale_tensors(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    let mut result = Vec::with_capacity(tensors.len());
    for tensor in tensors {
        let scale_tensor = Tensor::new(scale, tensor.device())?;
        result.push(tensor.broadcast_mul(&scale_tensor)?);
    }
    Ok(result)
}
/// Combine two tensor vectors by element-wise addition.
pub fn combine_tensors(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            format!("Tensor vectors must have same length: {} vs {}", a.len(), b.len())
        ));
    }
    let mut result = Vec::with_capacity(a.len());
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        result.push(tensor_a.add(tensor_b)?);
    }
    Ok(result)
}
/// Compute the dot product between two tensor vectors.
pub fn tensor_dot_product(a: &[Tensor], b: &[Tensor]) -> CandleResult<f64> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            format!("Tensor vectors must have same length: {} vs {}", a.len(), b.len())
        ));
    }
    let mut sum = 0.0;
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        let product = tensor_a.mul(tensor_b)?;
        let flat = product.flatten_all()?;
        let values = flat.to_vec1::<f64>()?;
        sum += values.iter().sum::<f64>();
    }
    Ok(sum)
}
/// Normalize a vector of tensors to unit magnitude.
pub fn normalize_vector(tensors: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    let magnitude = compute_magnitude(tensors)?;
    if magnitude > 0.0 {
        scale_tensors(tensors, 1.0 / magnitude)
    } else {
        Ok(tensors.to_vec())
    }
}
/// Compute dot product between two vectors (simple f64 version).
pub fn dot_product(a: &[Tensor], b: &[Tensor]) -> CandleResult<f64> {
    tensor_dot_product(a, b)
}
/// Compute the magnitude of parameter change.
pub fn compute_parameter_change(old_params: &[Tensor], new_params: &[Tensor]) -> CandleResult<f64> {
    if old_params.len() != new_params.len() {
        return Err(candle_core::Error::Msg(
            format!("Parameter vectors must have same length: {} vs {}", old_params.len(), new_params.len())
        ));
    }
    let mut sum = 0.0;
    for (old, new) in old_params.iter().zip(new_params.iter()) {
        let diff = new.sub(old)?;
        let flat = diff.flatten_all()?;
        let values = flat.to_vec1::<f64>()?;
        for val in values {
            sum += val * val;
        }
    }
    Ok(sum.sqrt())
}
/// Add two tensor vectors element-wise.
pub fn vector_add(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    combine_tensors(a, b)
}
/// Subtract two tensor vectors element-wise (a - b).
pub fn vector_subtract(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            format!("Tensor vectors must have same length: {} vs {}", a.len(), b.len())
        ));
    }
    let mut result = Vec::with_capacity(a.len());
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        result.push(tensor_a.sub(tensor_b)?);
    }
    Ok(result)
}
/// Scale a vector of tensors by a scalar factor (alias for scale_tensors).
pub fn vector_scale(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    scale_tensors(tensors, scale)
}
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    #[test]
    fn test_compute_magnitude() -> CandleResult<()> {
        let device = Device::Cpu;
        let tensors = vec![
            Tensor::from_slice(&[3.0, 4.0], &[2], &device)?,
        ];
        let magnitude = compute_magnitude(&tensors)?;
        assert_relative_eq!(magnitude, 5.0, epsilon = 1e-10);
        Ok(())
    }
    #[test]
    fn test_magnitude_relative_difference() {
        assert_relative_eq!(magnitude_relative_difference(10.0, 9.0), 0.1, epsilon = 1e-10);
        assert_relative_eq!(magnitude_relative_difference(5.0, 5.0), 0.0, epsilon = 1e-10);
        assert_relative_eq!(magnitude_relative_difference(0.0, 0.0), 0.0, epsilon = 1e-10);
    }
    #[test]
    fn test_scale_tensors() -> CandleResult<()> {
        let device = Device::Cpu;
        let tensors = vec![
            Tensor::from_slice(&[1.0, 2.0], &[2], &device)?,
        ];
        let scaled = scale_tensors(&tensors, 2.0)?;
        let values = scaled[0].to_vec1::<f64>()?;
        assert_relative_eq!(values[0], 2.0, epsilon = 1e-10);
        assert_relative_eq!(values[1], 4.0, epsilon = 1e-10);
        Ok(())
    }
    #[test]
    fn test_combine_tensors() -> CandleResult<()> {
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
    fn test_tensor_dot_product() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let b = vec![Tensor::from_slice(&[3.0, 4.0], &[2], &device)?];
        let dot = tensor_dot_product(&a, &b)?;
        assert_relative_eq!(dot, 11.0, epsilon = 1e-10); // 1*3 + 2*4 = 11
        Ok(())
    }
    #[test]
    fn test_normalize_vector() -> CandleResult<()> {
        let device = Device::Cpu;
        let tensors = vec![
            Tensor::from_slice(&[3.0, 4.0], &[2], &device)?,
        ];
        let normalized = normalize_vector(&tensors)?;
        let magnitude = compute_magnitude(&normalized)?;
        assert_relative_eq!(magnitude, 1.0, epsilon = 1e-10);
        Ok(())
    }
    #[test]
    fn test_vector_operations() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[5.0, 6.0], &[2], &device)?];
        let b = vec![Tensor::from_slice(&[2.0, 3.0], &[2], &device)?];
        // Test subtraction
        let diff = vector_subtract(&a, &b)?;
        let diff_values = diff[0].to_vec1::<f64>()?;
        assert_relative_eq!(diff_values[0], 3.0, epsilon = 1e-10);
        assert_relative_eq!(diff_values[1], 3.0, epsilon = 1e-10);
        // Test addition
        let sum = vector_add(&a, &b)?;
        let sum_values = sum[0].to_vec1::<f64>()?;
        assert_relative_eq!(sum_values[0], 7.0, epsilon = 1e-10);
        assert_relative_eq!(sum_values[1], 9.0, epsilon = 1e-10);
        Ok(())
    }
}