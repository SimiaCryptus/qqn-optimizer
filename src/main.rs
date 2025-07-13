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
