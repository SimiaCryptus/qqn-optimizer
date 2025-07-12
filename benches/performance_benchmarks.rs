use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use log::{info, debug, warn};
use qqn_optimizer::core::optimizer::Optimizer;
use qqn_optimizer::core::qqn::{QQNOptimizer, QQNConfig};
use qqn_optimizer::core::lbfgs::{LBFGSOptimizer, LBFGSConfig};
use qqn_optimizer::benchmarks::functions::{RosenbrockFunction, SphereFunction, OptimizationProblem};
use qqn_optimizer::utils::math::compute_magnitude;
use candle_core::{Tensor, Device};
fn init_logger() {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();
}


fn tensor_from_vec(values: Vec<f64>) -> Tensor {
    Tensor::from_slice(&values, &[values.len()], &Device::Cpu).unwrap()
}

fn tensors_to_vec(tensors: &[Tensor]) -> Vec<f64> {
    tensors.iter()
        .flat_map(|t| t.to_vec1::<f64>().unwrap())
        .collect()
}

fn benchmark_qqn_step(c: &mut Criterion) {
    init_logger();
    info!("Starting QQN step benchmarks");
    
    let mut group = c.benchmark_group("optimizer_step");
    
    for dimension in [10, 50, 100, 500].iter() {
        info!("Benchmarking QQN step for dimension {}", dimension);
        let problem = RosenbrockFunction::new(*dimension);
        let config = QQNConfig::default();
        let mut optimizer = QQNOptimizer::new(config);
        
        let x = problem.initial_point();
        let mut params = vec![tensor_from_vec(x.clone())];
        let gradient_vec = problem.gradient(&x).unwrap();
        let gradients = vec![tensor_from_vec(gradient_vec)];
        debug!("Setup complete for dimension {}: initial_point_norm={:.6e}, gradient_norm={:.6e}", 
               dimension, 
               x.iter().map(|&v| v*v).sum::<f64>().sqrt(),
               gradient_vec.iter().map(|&v| v*v).sum::<f64>().sqrt());
        
        group.bench_with_input(
            BenchmarkId::new("QQN", dimension),
            dimension,
            |b, _| {
                b.iter(|| {
                    let _ = optimizer.step(black_box(&mut params), black_box(&gradients));
                })
            },
        );
    }
    
    group.finish();
    info!("QQN step benchmarks completed");
}

fn benchmark_optimizer_comparison(c: &mut Criterion) {
    init_logger();
    info!("Starting optimizer comparison benchmarks");
    
    let mut group = c.benchmark_group("optimizer_comparison");
    let dimension = 100;
    info!("Comparing optimizers on dimension {}", dimension);
    
    let problem = RosenbrockFunction::new(dimension);
    let x = problem.initial_point();
    let gradient_vec = problem.gradient(&x).unwrap();
    debug!("Problem setup: dimension={}, initial_value={:.6e}, gradient_norm={:.6e}", 
           dimension, 
           problem.evaluate(&x).unwrap(),
           gradient_vec.iter().map(|&v| v*v).sum::<f64>().sqrt());
    
    // QQN Benchmark
    {
        info!("Benchmarking QQN optimizer");
        let config = QQNConfig::default();
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![tensor_from_vec(x.clone())];
        let gradients = vec![tensor_from_vec(gradient_vec.clone())];
        
        group.bench_function("QQN_100d", |b| {
            b.iter(|| {
                let _ = optimizer.step(black_box(&mut params), black_box(&gradients));
            })
        });
    }
    
    // L-BFGS Benchmark
    {
        info!("Benchmarking L-BFGS optimizer");
        let config = LBFGSConfig::default();
        let mut optimizer = LBFGSOptimizer::new(config);
        let mut params = vec![tensor_from_vec(x.clone())];
        let gradients = vec![tensor_from_vec(gradient_vec.clone())];
        
        group.bench_function("LBFGS_100d", |b| {
            b.iter(|| {
                let _ = optimizer.step(black_box(&mut params), black_box(&gradients));
            })
        });
    }
    
    group.finish();
    info!("Optimizer comparison benchmarks completed");
}

fn benchmark_magnitude_computation(c: &mut Criterion) {
    init_logger();
    info!("Starting magnitude computation benchmarks");
    
    let mut group = c.benchmark_group("magnitude_computation");
    
    for size in [100, 1000, 10000].iter() {
        info!("Benchmarking magnitude computation for size {}", size);
        let values: Vec<f64> = (0..*size).map(|i| (i as f64) * 0.1).collect();
        let tensors = vec![tensor_from_vec(values)];
        debug!("Created tensor with {} elements for magnitude computation", size);
        
        group.bench_with_input(
            BenchmarkId::new("compute_magnitude", size),
            size,
            |b, _| {
                b.iter(|| {
                    let _ = compute_magnitude(black_box(&tensors));
                })
            },
        );
    }
    
    group.finish();
    info!("Magnitude computation benchmarks completed");
}

fn benchmark_quadratic_path_evaluation(c: &mut Criterion) {
    init_logger();
    info!("Starting quadratic path evaluation benchmarks");
    
    let mut group = c.benchmark_group("quadratic_path");
    
    let dimension = 1000;
    info!("Benchmarking quadratic path evaluation for dimension {}", dimension);
    let gradient_values: Vec<f64> = (0..dimension).map(|i| (i as f64) * 0.01).collect();
    let lbfgs_values: Vec<f64> = (0..dimension).map(|i| (i as f64) * 0.02).collect();
    
    let gradient_tensors = vec![tensor_from_vec(gradient_values)];
    let lbfgs_tensors = vec![tensor_from_vec(lbfgs_values)];
    
    let config = QQNConfig::default();
    let optimizer = QQNOptimizer::new(config);
    let quadratic_path = optimizer.create_quadratic_path(&gradient_tensors, &lbfgs_tensors).unwrap();
    debug!("Created quadratic path for {} dimensional vectors", dimension);
    
    group.bench_function("evaluate_path", |b| {
        b.iter(|| {
            let _ = quadratic_path.evaluate(black_box(0.5));
        })
    });
    
    group.bench_function("evaluate_derivative", |b| {
        b.iter(|| {
            let _ = quadratic_path.derivative(black_box(0.5));
        })
    });
    
    group.finish();
    info!("Quadratic path evaluation benchmarks completed");
}

fn benchmark_full_optimization(c: &mut Criterion) {
    init_logger();
    info!("Starting full optimization benchmarks");
    
    let mut group = c.benchmark_group("full_optimization");
    group.sample_size(10); // Fewer samples for longer benchmarks
    
    let dimension = 50;
    let max_iterations = 100;
    info!("Full optimization benchmark: dimension={}, max_iterations={}", dimension, max_iterations);
    
    // QQN full optimization
    group.bench_function("QQN_full_rosenbrock", |b| {
        b.iter(|| {
            debug!("Starting QQN full optimization run");
            let problem = RosenbrockFunction::new(dimension);
            let config = QQNConfig::default();
            let mut optimizer = QQNOptimizer::new(config);
            
            let mut x = problem.initial_point();
            let mut params = vec![tensor_from_vec(x.clone())];
            let mut converged_iteration = None;
            
            for _ in 0..max_iterations {
                let gradient_vec = problem.gradient(&x).unwrap();
                let gradients = vec![tensor_from_vec(gradient_vec)];
                
                let _ = optimizer.step(&mut params, &gradients);
                x = tensors_to_vec(&params);
                
                let current_value = problem.evaluate(&x).unwrap();
                if current_value < 1e-6 {
                    converged_iteration = Some(iteration);
                    break;
                }
            }
            if let Some(iter) = converged_iteration {
                debug!("QQN converged at iteration {}", iter);
            } else {
                debug!("QQN did not converge within {} iterations", max_iterations);
            }
        })
    });
    
    // L-BFGS full optimization
    group.bench_function("LBFGS_full_rosenbrock", |b| {
        b.iter(|| {
            debug!("Starting L-BFGS full optimization run");
            let problem = RosenbrockFunction::new(dimension);
            let config = LBFGSConfig::default();
            let mut optimizer = LBFGSOptimizer::new(config);
            
            let mut x = problem.initial_point();
            let mut params = vec![tensor_from_vec(x.clone())];
            let mut converged_iteration = None;
            
            for _ in 0..max_iterations {
                let gradient_vec = problem.gradient(&x).unwrap();
                let gradients = vec![tensor_from_vec(gradient_vec)];
                
                let _ = optimizer.step(&mut params, &gradients);
                x = tensors_to_vec(&params);
                
                let current_value = problem.evaluate(&x).unwrap();
                if current_value < 1e-6 {
                    converged_iteration = Some(iteration);
                    break;
                }
            }
            if let Some(iter) = converged_iteration {
                debug!("L-BFGS converged at iteration {}", iter);
            } else {
                debug!("L-BFGS did not converge within {} iterations", max_iterations);
            }
        })
    });
    
    group.finish();
    info!("Full optimization benchmarks completed");
}

fn benchmark_memory_usage(c: &mut Criterion) {
    init_logger();
    info!("Starting memory usage benchmarks");
    
    let mut group = c.benchmark_group("memory_usage");
    
    // Test memory allocation patterns
    for dimension in [100, 500, 1000].iter() {
        info!("Benchmarking memory usage for dimension {}", dimension);
        
        group.bench_with_input(
            BenchmarkId::new("optimizer_creation", dimension),
            dimension,
            |b, &dim| {
                debug!("Benchmarking optimizer creation for dimension {}", dim);
                b.iter(|| {
                    let config = QQNConfig::default();
                    let optimizer = QQNOptimizer::new(config);
                    black_box(optimizer);
                })
            },
        );
        
        group.bench_with_input(
            BenchmarkId::new("tensor_operations", dimension),
            dimension,
            |b, &dim| {
                debug!("Benchmarking tensor operations for dimension {}", dim);
                let values: Vec<f64> = (0..dim).map(|i| i as f64).collect();
                let tensors = vec![tensor_from_vec(values)];
                
                b.iter(|| {
                    let magnitude = compute_magnitude(black_box(&tensors)).unwrap();
                    black_box(magnitude);
                })
            },
        );
    }
    
    group.finish();
    info!("Memory usage benchmarks completed");
}

criterion_group!(
    benches,
    benchmark_qqn_step,
    benchmark_optimizer_comparison,
    benchmark_magnitude_computation,
    benchmark_quadratic_path_evaluation,
    benchmark_full_optimization,
    benchmark_memory_usage
);

criterion_main!(benches);