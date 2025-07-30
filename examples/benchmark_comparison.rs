#!/usr/bin/env -S cargo +nightly -Zscript
//! Benchmark Comparison: OneDNN vs Candle MNIST Implementation
//!
//! This example compares the basic performance characteristics of OneDNN and Candle
//! implementations of MNIST neural network training.
//!
//! To run this benchmark:
//! ```bash
//! # With OneDNN support
//! cargo run --example benchmark_comparison --features onednn --release
//!
//! # Without OneDNN (Candle only)
//! cargo run --example benchmark_comparison --release
//! ```

use qqn_optimizer::{init_logging, MnistNeuralNetwork, OptimizationProblem};
use rand::{rngs::StdRng, SeedableRng};
use std::time::Instant;

#[cfg(feature = "onednn")]
use qqn_optimizer::{
    benchmarks::mnist_onednn::ActivationType as OneDnnActivationType, MnistOneDnnNeuralNetwork,
};

use qqn_optimizer::benchmarks::mnist::ActivationType as CandleActivationType;

#[derive(Debug)]
struct BenchmarkResult {
    name: String,
    setup_time: std::time::Duration,
    initial_loss: f64,
    eval_time_per_call: std::time::Duration,
    grad_time_per_call: std::time::Duration,
    parameter_count: usize,
    memory_usage_estimate: usize,
}

fn main() -> anyhow::Result<()> {
    init_logging(false)?;

    println!("MNIST Neural Network Benchmark: OneDNN vs Candle");
    println!("================================================");

    let samples = 200; // Small dataset for quick comparison

    let mut results = Vec::new();

    // Benchmark Candle implementation
    println!("\n🔥 Benchmarking Candle Implementation...");
    let candle_result = benchmark_candle(samples)?;
    results.push(candle_result);

    // Benchmark OneDNN implementation (if available)
    #[cfg(feature = "onednn")]
    {
        println!("\n⚡ Benchmarking OneDNN Implementation...");
        let onednn_result = benchmark_onednn(samples)?;
        results.push(onednn_result);
    }

    #[cfg(not(feature = "onednn"))]
    {
        println!("\n❌ OneDNN implementation not available");
        println!("   To include OneDNN in the benchmark, run:");
        println!("   cargo run --example benchmark_comparison --features onednn --release");
    }

    // Display results
    display_results(&results);

    Ok(())
}

fn benchmark_candle(samples: usize) -> anyhow::Result<BenchmarkResult> {
    let mut rng = StdRng::seed_from_u64(42);

    // Setup
    let setup_start = Instant::now();
    let network = MnistNeuralNetwork::create(
        Some(samples),
        &[32, 16],
        Some(32),
        &mut rng,
        Some(CandleActivationType::ReLU),
    )?;
    let setup_time = setup_start.elapsed();

    let initial_params = network.initial_point();

    // Measure initial evaluation
    let eval_start = Instant::now();
    let initial_loss = network.evaluate_f64(&initial_params)?;
    let eval_time = eval_start.elapsed();

    // Measure gradient computation
    let grad_start = Instant::now();
    let _ = network.gradient_f64(&initial_params)?;
    let grad_time = grad_start.elapsed();

    // Estimate memory usage (parameters + some overhead)
    let memory_estimate = initial_params.len() * 8 + samples * 784 * 4; // f64 params + f32 data

    Ok(BenchmarkResult {
        name: "Candle".to_string(),
        setup_time,
        initial_loss,
        eval_time_per_call: eval_time,
        grad_time_per_call: grad_time,
        parameter_count: initial_params.len(),
        memory_usage_estimate: memory_estimate,
    })
}

#[cfg(feature = "onednn")]
fn benchmark_onednn(samples: usize) -> anyhow::Result<BenchmarkResult> {
    let mut rng = StdRng::seed_from_u64(42);

    // Setup
    let setup_start = Instant::now();
    let network = MnistOneDnnNeuralNetwork::create(
        Some(samples),
        &[32, 16],
        Some(32),
        &mut rng,
        Some(OneDnnActivationType::ReLU),
    )?;
    let setup_time = setup_start.elapsed();

    let initial_params = network.initial_point();

    // Measure initial evaluation
    let eval_start = Instant::now();
    let initial_loss = network.evaluate_f64(&initial_params)?;
    let eval_time = eval_start.elapsed();

    // Measure gradient computation
    let grad_start = Instant::now();
    let _ = network.gradient_f64(&initial_params)?;
    let grad_time = grad_start.elapsed();

    // Estimate memory usage (parameters + OneDNN overhead)
    let memory_estimate = initial_params.len() * 8 + samples * 784 * 4 + 1024; // Extra for OneDNN

    Ok(BenchmarkResult {
        name: "OneDNN".to_string(),
        setup_time,
        initial_loss,
        eval_time_per_call: eval_time,
        grad_time_per_call: grad_time,
        parameter_count: initial_params.len(),
        memory_usage_estimate: memory_estimate,
    })
}

fn display_results(results: &[BenchmarkResult]) {
    println!("\n📊 Benchmark Results");
    println!("==================");

    // Header
    println!(
        "{:<12} {:<12} {:<12} {:<12} {:<12} {:<12} {:<12}",
        "Backend", "Setup (ms)", "Init Loss", "Eval (μs)", "Grad (μs)", "Params", "Memory (KB)"
    );
    println!("{}", "-".repeat(84));

    // Results
    for result in results {
        println!(
            "{:<12} {:<12.1} {:<12.6} {:<12.0} {:<12.0} {:<12} {:<12.1}",
            result.name,
            result.setup_time.as_secs_f64() * 1000.0,
            result.initial_loss,
            result.eval_time_per_call.as_secs_f64() * 1_000_000.0,
            result.grad_time_per_call.as_secs_f64() * 1_000_000.0,
            result.parameter_count,
            result.memory_usage_estimate as f64 / 1024.0
        );
    }

    // Performance comparison
    if results.len() >= 2 {
        println!("\n🏆 Performance Comparison");
        println!("=======================");

        let candle = &results[0];
        let onednn = &results[1];

        let eval_speedup =
            candle.eval_time_per_call.as_secs_f64() / onednn.eval_time_per_call.as_secs_f64();
        let grad_speedup =
            candle.grad_time_per_call.as_secs_f64() / onednn.grad_time_per_call.as_secs_f64();
        let setup_speedup = candle.setup_time.as_secs_f64() / onednn.setup_time.as_secs_f64();

        println!("OneDNN vs Candle speedup:");
        println!(
            "  - Network setup: {:.2}x {}",
            setup_speedup,
            speedup_emoji(setup_speedup)
        );
        println!(
            "  - Function evaluation: {:.2}x {}",
            eval_speedup,
            speedup_emoji(eval_speedup)
        );
        println!(
            "  - Gradient computation: {:.2}x {}",
            grad_speedup,
            speedup_emoji(grad_speedup)
        );

        // Architecture verification
        if candle.parameter_count == onednn.parameter_count {
            println!("  - ✅ Parameter counts match: {}", candle.parameter_count);
        } else {
            println!(
                "  - ⚠️  Parameter count mismatch: {} vs {}",
                candle.parameter_count, onednn.parameter_count
            );
        }

        // Loss comparison
        let loss_diff = (candle.initial_loss - onednn.initial_loss).abs();
        if loss_diff < 0.1 {
            println!(
                "  - ✅ Initial losses similar: {:.6} vs {:.6}",
                candle.initial_loss, onednn.initial_loss
            );
        } else {
            println!("  - ⚠️  Initial loss difference: {:.6}", loss_diff);
        }
    }

    println!("\n💡 Implementation Details:");
    for result in results {
        println!("  {}:", result.name);
        match result.name.as_str() {
            "Candle" => {
                println!("    - Uses Candle tensor operations");
                println!("    - Automatic differentiation for gradients");
                println!("    - Rayon for parallel batch processing");
                println!("    - Cross-platform compatibility");
            }
            "OneDNN" => {
                println!("    - Uses Intel OneDNN primitives");
                println!("    - Optimized CPU GEMM operations");
                println!("    - Hardware-aware memory layouts");
                println!("    - Finite differences for gradients (demo)");
            }
            _ => {}
        }
    }

    println!("\n📋 Notes:");
    println!("  - This is a micro-benchmark with a small dataset");
    println!("  - OneDNN performance improves significantly with larger problems");
    println!("  - Gradient computation uses finite differences in OneDNN demo");
    println!("  - Results may vary based on CPU architecture and system load");
    println!("  - For production use, test with your specific problem sizes");

    #[cfg(feature = "onednn")]
    println!("  - OneDNN feature is enabled and functional");

    #[cfg(not(feature = "onednn"))]
    println!("  - OneDNN feature is not enabled in this build");
}

fn speedup_emoji(speedup: f64) -> &'static str {
    if speedup > 2.0 {
        "🚀"
    } else if speedup > 1.5 {
        "⚡"
    } else if speedup > 1.1 {
        "✅"
    } else if speedup > 0.9 {
        "➖"
    } else {
        "🐌"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_candle() {
        let result = benchmark_candle(10);
        assert!(result.is_ok());

        let benchmark = result.unwrap();
        assert_eq!(benchmark.name, "Candle");
        assert!(benchmark.initial_loss > 0.0);
        assert!(benchmark.parameter_count > 0);
    }

    #[cfg(feature = "onednn")]
    #[test]
    fn test_benchmark_onednn() {
        let result = benchmark_onednn(10);
        assert!(result.is_ok());

        let benchmark = result.unwrap();
        assert_eq!(benchmark.name, "OneDNN");
        assert!(benchmark.initial_loss > 0.0);
        assert!(benchmark.parameter_count > 0);
    }
}
