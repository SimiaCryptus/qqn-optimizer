//! Random number generation utilities that work with candle tensors and half-precision types

use candle_core::{DType, Device, Result as CandleResult, Tensor};
use half::{bf16, f16};
use rand::Rng;
/// Random number generator wrapper for optimization algorithms
pub struct RandomGenerator {
    rng: rand::rngs::ThreadRng,
}
impl RandomGenerator {
    /// Create a new random generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }
    /// Generate a random f64 in the given range
    pub fn gen_range(&mut self, range: std::ops::Range<f64>) -> f64 {
        self.rng.gen_range(range)
    }
    /// Generate a random vector of given length with values in range
    pub fn gen_vector(&mut self, len: usize, range: std::ops::Range<f64>) -> Vec<f64> {
        (0..len).map(|_| self.gen_range(range.clone())).collect()
    }
    /// Generate normally distributed random number
    pub fn gen_normal(&mut self, mean: f64, std: f64) -> f64 {
        use rand_distr::{Distribution, Normal};
        let normal = Normal::new(mean, std).unwrap();
        normal.sample(&mut self.rng)
    }
}
impl Default for RandomGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper function to generate random f16 values
fn random_f16<R: Rng>(rng: &mut R) -> f16 {
    f16::from_f32(rng.gen_range(-0.1f32..0.1f32))
}
/// Helper function to generate random bf16 values
fn random_bf16<R: Rng>(rng: &mut R) -> bf16 {
    bf16::from_f32(rng.gen_range(-0.1f32..0.1f32))
}
/// Helper function to generate normal distributed f16 values
fn normal_f16<R: Rng>(rng: &mut R, mean: f64, std: f64) -> f16 {
    use rand_distr::Normal;
    let normal = Normal::new(mean, std).unwrap();
    f16::from_f32(rng.sample(normal) as f32)
}
/// Helper function to generate normal distributed bf16 values
fn normal_bf16<R: Rng>(rng: &mut R, mean: f64, std: f64) -> bf16 {
    use rand_distr::Normal;
    let normal = Normal::new(mean, std).unwrap();
    bf16::from_f32(rng.sample(normal) as f32)
}

/// Generate a random tensor with the given shape and data type
pub fn random_tensor(shape: &[usize], dtype: DType, device: &Device) -> CandleResult<Tensor> {
    let mut rng = rand::thread_rng();

    match dtype {
        DType::F32 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f32> = (0..size).map(|_| rng.gen_range(-0.1f32..0.1f32)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::F64 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f64> = (0..size).map(|_| rng.gen_range(-0.1f64..0.1f64)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::F16 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f16> = (0..size).map(|_| random_f16(&mut rng)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::BF16 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<bf16> = (0..size).map(|_| random_bf16(&mut rng)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        _ => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f32> = (0..size).map(|_| rng.gen_range(-0.1f32..0.1f32)).collect();
            let f32_tensor = Tensor::from_slice(&data, shape, device)?;
            f32_tensor.to_dtype(dtype)
        }
    }
}

/// Generate random normal tensor with given mean and std
pub fn random_normal_tensor(
    shape: &[usize],
    mean: f64,
    std: f64,
    dtype: DType,
    device: &Device,
) -> CandleResult<Tensor> {
    use rand_distr::{Distribution, Normal};
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mean, std).map_err(|e| {
        candle_core::Error::Msg(format!("Failed to create normal distribution: {}", e))
    })?;

    match dtype {
        DType::F32 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f32> = (0..size).map(|_| normal.sample(&mut rng) as f32).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::F64 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f64> = (0..size).map(|_| normal.sample(&mut rng)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::F16 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f16> = (0..size).map(|_| normal_f16(&mut rng, mean, std)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::BF16 => {
            let size = shape.iter().product::<usize>();
            let data: Vec<bf16> = (0..size)
                .map(|_| normal_bf16(&mut rng, mean, std))
                .collect();
            Tensor::from_slice(&data, shape, device)
        }
        _ => {
            let size = shape.iter().product::<usize>();
            let data: Vec<f32> = (0..size).map(|_| normal.sample(&mut rng) as f32).collect();
            let f32_tensor = Tensor::from_slice(&data, shape, device)?;
            f32_tensor.to_dtype(dtype)
        }
    }
}

/// Generate random tensor with Xavier/Glorot initialization
pub fn xavier_normal_tensor(
    shape: &[usize],
    dtype: DType,
    device: &Device,
) -> CandleResult<Tensor> {
    // Calculate fan_in and fan_out for Xavier initialization
    let fan_in = if shape.len() >= 2 { shape[1] } else { 1 };
    let fan_out = if shape.len() >= 1 { shape[0] } else { 1 };
    let std = (2.0 / (fan_in + fan_out) as f64).sqrt();
    random_normal_tensor(shape, 0.0, std, dtype, device)
}
/// Generate random starting point for optimization
pub fn random_starting_point(dimension: usize, bounds: Option<(Vec<f64>, Vec<f64>)>) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    match bounds {
        Some((lower, upper)) => (0..dimension)
            .map(|i| {
                let low = lower.get(i).copied().unwrap_or(-1.0);
                let high = upper.get(i).copied().unwrap_or(1.0);
                rng.gen_range(low..high)
            })
            .collect(),
        None => (0..dimension).map(|_| rng.gen_range(-1.0..1.0)).collect(),
    }
}
/// Generate multiple random starting points
pub fn random_starting_points(
    dimension: usize,
    count: usize,
    bounds: Option<(Vec<f64>, Vec<f64>)>,
) -> Vec<Vec<f64>> {
    (0..count)
        .map(|_| random_starting_point(dimension, bounds.clone()))
        .collect()
}

/// Generate random tensor with He initialization
pub fn he_normal_tensor(shape: &[usize], dtype: DType, device: &Device) -> CandleResult<Tensor> {
    // Calculate fan_in for He initialization
    let fan_in = if shape.len() >= 2 { shape[1] } else { 1 };
    let std = (2.0 / fan_in as f64).sqrt();
    random_normal_tensor(shape, 0.0, std, dtype, device)
}

/// Generate random uniform tensor with explicit range support for half-precision
pub fn random_uniform_tensor(
    shape: &[usize],
    low: f64,
    high: f64,
    dtype: DType,
    device: &Device,
) -> CandleResult<Tensor> {
    let mut rng = rand::thread_rng();
    let size = shape.iter().product::<usize>();

    match dtype {
        DType::F32 => {
            let data: Vec<f32> = (0..size)
                .map(|_| rng.gen_range(low as f32..high as f32))
                .collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::F64 => {
            let data: Vec<f64> = (0..size).map(|_| rng.gen_range(low..high)).collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::F16 => {
            let data: Vec<f16> = (0..size)
                .map(|_| {
                    let val = rng.gen_range(low as f32..high as f32);
                    f16::from_f32(val)
                })
                .collect();
            Tensor::from_slice(&data, shape, device)
        }
        DType::BF16 => {
            let data: Vec<bf16> = (0..size)
                .map(|_| {
                    let val = rng.gen_range(low as f32..high as f32);
                    bf16::from_f32(val)
                })
                .collect();
            Tensor::from_slice(&data, shape, device)
        }
        _ => {
            let data: Vec<f32> = (0..size)
                .map(|_| rng.gen_range(low as f32..high as f32))
                .collect();
            let f32_tensor = Tensor::from_slice(&data, shape, device)?;
            f32_tensor.to_dtype(dtype)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candle_core::Device;

    #[test]
    fn test_random_tensor_f32() {
        let device = Device::Cpu;
        let tensor = random_tensor(&[2, 3], DType::F32, &device).unwrap();
        assert_eq!(tensor.shape().dims(), &[2, 3]);
        assert_eq!(tensor.dtype(), DType::F32);
    }

    #[test]
    fn test_random_tensor_f16() {
        let device = Device::Cpu;
        let tensor = random_tensor(&[2, 3], DType::F16, &device).unwrap();
        assert_eq!(tensor.shape().dims(), &[2, 3]);
        assert_eq!(tensor.dtype(), DType::F16);
    }

    #[test]
    fn test_random_tensor_bf16() {
        let device = Device::Cpu;
        let tensor = random_tensor(&[2, 3], DType::BF16, &device).unwrap();
        assert_eq!(tensor.shape().dims(), &[2, 3]);
        assert_eq!(tensor.dtype(), DType::BF16);
    }

    #[test]
    fn test_random_normal_tensor() {
        let device = Device::Cpu;
        let tensor = random_normal_tensor(&[10, 10], 0.0, 1.0, DType::F32, &device).unwrap();
        assert_eq!(tensor.shape().dims(), &[10, 10]);

        // Basic check that tensor was created successfully
        assert_eq!(tensor.dtype(), DType::F32);
    }

    #[test]
    fn test_xavier_normal_tensor() {
        let device = Device::Cpu;
        let tensor = xavier_normal_tensor(&[100, 50], DType::F32, &device).unwrap();
        assert_eq!(tensor.shape().dims(), &[100, 50]);
        assert_eq!(tensor.dtype(), DType::F32);
    }

    #[test]
    fn test_he_normal_tensor() {
        let device = Device::Cpu;
        let tensor = he_normal_tensor(&[100, 50], DType::F32, &device).unwrap();
        assert_eq!(tensor.shape().dims(), &[100, 50]);
        assert_eq!(tensor.dtype(), DType::F32);
    }
    #[test]
    fn test_half_precision_tensors() {
        let device = Device::Cpu;
        // Test F16
        let f16_tensor = random_tensor(&[5, 5], DType::F16, &device).unwrap();
        assert_eq!(f16_tensor.dtype(), DType::F16);
        // Test BF16
        let bf16_tensor = random_tensor(&[5, 5], DType::BF16, &device).unwrap();
        assert_eq!(bf16_tensor.dtype(), DType::BF16);
        // Test normal distribution with half precision
        let f16_normal = random_normal_tensor(&[3, 3], 0.0, 1.0, DType::F16, &device).unwrap();
        assert_eq!(f16_normal.dtype(), DType::F16);
        let bf16_normal = random_normal_tensor(&[3, 3], 0.0, 1.0, DType::BF16, &device).unwrap();
        assert_eq!(bf16_normal.dtype(), DType::BF16);
    }
    #[test]
    fn test_random_uniform_tensor() {
        let device = Device::Cpu;
        // Test F32
        let f32_tensor = random_uniform_tensor(&[5, 5], -1.0, 1.0, DType::F32, &device).unwrap();
        assert_eq!(f32_tensor.dtype(), DType::F32);
        // Test F16
        let f16_tensor = random_uniform_tensor(&[3, 3], -0.5, 0.5, DType::F16, &device).unwrap();
        assert_eq!(f16_tensor.dtype(), DType::F16);
        // Test BF16
        let bf16_tensor = random_uniform_tensor(&[3, 3], -0.5, 0.5, DType::BF16, &device).unwrap();
        assert_eq!(bf16_tensor.dtype(), DType::BF16);
    }
    #[test]
    fn test_helper_functions() {
        let mut rng = rand::thread_rng();
        // Test f16 generation
        let f16_val = random_f16(&mut rng);
        assert!(f16_val.to_f32() >= -0.1 && f16_val.to_f32() <= 0.1);
        // Test bf16 generation
        let bf16_val = random_bf16(&mut rng);
        assert!(bf16_val.to_f32() >= -0.1 && bf16_val.to_f32() <= 0.1);
        // Test normal distribution
        let f16_normal = normal_f16(&mut rng, 0.0, 1.0);
        assert!(f16_normal.is_finite());
        let bf16_normal = normal_bf16(&mut rng, 0.0, 1.0);
        assert!(bf16_normal.is_finite());
    }
}
