fn main() {
    println!("Hello, world!");
}

use anyhow::Result;
use candle_core::{Result as CandleResult, Tensor};
/// Compute the magnitude (L2 norm) of a vector of tensors
pub fn compute_magnitude(tensors: &[Tensor]) -> CandleResult<f64> {
    let mut sum_squares = 0.0;
    for tensor in tensors {
        let values = tensor.to_vec1::<f64>()?;
        for &value in &values {
            sum_squares += value * value;
        }
    }
    Ok(sum_squares.sqrt())
}
/// Compute dot product between two tensor vectors
pub fn dot_product(a: &[Tensor], b: &[Tensor]) -> CandleResult<f64> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            "Vector dimensions must match".to_string(),
        ));
    }
    let mut result = 0.0;
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        let values_a = tensor_a.to_vec1::<f64>()?;
        let values_b = tensor_b.to_vec1::<f64>()?;
        if values_a.len() != values_b.len() {
            return Err(candle_core::Error::Msg(
                "Tensor dimensions must match".to_string(),
            ));
        }
        for (&val_a, &val_b) in values_a.iter().zip(values_b.iter()) {
            result += val_a * val_b;
        }
    }
    Ok(result)
}
/// Compute dot product between two f64 vectors
pub fn dot_product_f64(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Vector dimensions must match"));
    }
    let mut result = 0.0;
    for (&val_a, &val_b) in a.iter().zip(b.iter()) {
        result += val_a * val_b;
    }
    Ok(result)
}

/// Add two tensor vectors element-wise
pub fn vector_add(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    if a.len() != b.len() {
        return Err(candle_core::Error::Msg(
            "Vector dimensions must match".to_string(),
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
            "Vector dimensions must match".to_string(),
        ));
    }
    let mut result = Vec::with_capacity(a.len());
    for (tensor_a, tensor_b) in a.iter().zip(b.iter()) {
        result.push(tensor_a.sub(tensor_b)?);
    }
    Ok(result)
}
/// Scale a tensor vector by a scalar
pub fn vector_scale(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    let mut result = Vec::with_capacity(tensors.len());
    for tensor in tensors {
        let scale_tensor = Tensor::new(scale, tensor.device())?;
        result.push(tensor.broadcast_mul(&scale_tensor)?);
    }
    Ok(result)
}
/// Scale tensors by a scalar factor
pub fn scale_tensors(tensors: &[Tensor], scale: f64) -> CandleResult<Vec<Tensor>> {
    vector_scale(tensors, scale)
}
/// Combine two tensor vectors element-wise (addition)
pub fn combine_tensors(a: &[Tensor], b: &[Tensor]) -> CandleResult<Vec<Tensor>> {
    vector_add(a, b)
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
/// Compute angle between two vectors in radians
pub fn angle_between(a: &[f64], b: &[f64]) -> Result<f64> {
    if a.len() != b.len() {
        return Err(anyhow::anyhow!("Vector dimensions must match"));
    }
    let dot = dot_product_f64(a, b)?;
    let norm_a = norm_l2(a);
    let norm_b = norm_l2(b);
    if norm_a == 0.0 || norm_b == 0.0 {
        return Err(anyhow::anyhow!("Cannot compute angle with zero vector"));
    }
    let cos_angle = dot / (norm_a * norm_b);
    Ok(cos_angle.clamp(-1.0, 1.0).acos())
}
/// Compute relative difference between two magnitudes
pub fn magnitude_relative_difference(mag1: f64, mag2: f64) -> f64 {
    let max_mag = mag1.max(mag2);
    if max_mag == 0.0 {
        0.0
    } else {
        (mag1 - mag2).abs() / max_mag
    }
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
    fn test_dot_product_tensors() -> CandleResult<()> {
        let device = Device::Cpu;
        let a = vec![Tensor::from_slice(&[1.0, 2.0], &[2], &device)?];
        let b = vec![Tensor::from_slice(&[3.0, 4.0], &[2], &device)?];
        let result = dot_product(&a, &b)?;
        assert_relative_eq!(result, 11.0, epsilon = 1e-10); // 1*3 + 2*4 = 11
        Ok(())
    }
    #[test]
    fn test_dot_product_f64() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let result = dot_product_f64(&a, &b).unwrap();
        assert_relative_eq!(result, 32.0, epsilon = 1e-10); // 1*4 + 2*5 + 3*6 = 32
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
    fn test_norms() {
        let values = vec![3.0, 4.0];
        assert_relative_eq!(norm_l2(&values), 5.0, epsilon = 1e-10);
        assert_relative_eq!(norm_l1(&values), 7.0, epsilon = 1e-10);
        assert_relative_eq!(norm_inf(&values), 4.0, epsilon = 1e-10);
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
    }
}
