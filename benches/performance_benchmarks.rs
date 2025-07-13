use candle_core::{Device, Tensor};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use qqn_optimizer::benchmarks::functions::{OptimizationProblem, RosenbrockFunction};
use qqn_optimizer::core::lbfgs::{LBFGSConfig, LBFGSOptimizer};
use qqn_optimizer::core::optimizer::Optimizer;
use qqn_optimizer::core::qqn::{QQNConfig, QQNOptimizer};
use qqn_optimizer::utils::math::compute_magnitude;

fn tensor_from_vec(values: Vec<f64>) -> Tensor {
    Tensor::from_slice(&values, &[values.len()], &Device::Cpu).unwrap()
}

fn tensors_to_vec(tensors: &[Tensor]) -> Vec<f64> {
    tensors.iter()
        .flat_map(|t| t.to_vec1::<f64>().unwrap())
        .collect()
}

fn benchmark_qqn_step(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimizer_step");

    for dimension in [10, 50, 100, 500].iter() {
        let problem = RosenbrockFunction::new(*dimension);
        let config = QQNConfig::default();
        let mut optimizer = QQNOptimizer::new(config);

        let x = problem.initial_point();
        let mut params = vec![tensor_from_vec(x.clone())];
        let gradient_vec = problem.gradient(&x).unwrap();
        let gradients = vec![tensor_from_vec(gradient_vec)];
        let objective_fn = |params: &[Tensor]| -> candle_core::Result<f64> {
            let param_vec = tensors_to_vec(params);
            problem.evaluate(&param_vec).map_err(|e| candle_core::Error::Msg(e.to_string()))
        };

        group.bench_with_input(
            BenchmarkId::new("QQN", dimension),
            dimension,
            |b, _| {
                b.iter(|| {
                    let _ = optimizer.step(black_box(&mut params), black_box(&gradients), &objective_fn);
                })
            },
        );
    }

    group.finish();
}

fn benchmark_optimizer_comparison(c: &mut Criterion) {
    let mut group = c.benchmark_group("optimizer_comparison");
    let dimension = 100;

    let problem = RosenbrockFunction::new(dimension);
    let x = problem.initial_point();
    let gradient_vec = problem.gradient(&x).unwrap();
    let objective_fn = |params: &[Tensor]| -> candle_core::Result<f64> {
        let param_vec = tensors_to_vec(params);
        problem.evaluate(&param_vec).map_err(|e| candle_core::Error::Msg(e.to_string()))
    };

    // QQN Benchmark
    {
        let config = QQNConfig::default();
        let mut optimizer = QQNOptimizer::new(config);
        let mut params = vec![tensor_from_vec(x.clone())];
        let gradients = vec![tensor_from_vec(gradient_vec.clone())];

        group.bench_function("QQN_100d", |b| {
            b.iter(|| {
                let _ = optimizer.step(black_box(&mut params), black_box(&gradients), &objective_fn);
            })
        });
    }

    // L-BFGS Benchmark
    {
        let config = LBFGSConfig::default();
        let mut optimizer = LBFGSOptimizer::new(config);
        let mut params = vec![tensor_from_vec(x.clone())];
        let gradients = vec![tensor_from_vec(gradient_vec.clone())];

        group.bench_function("LBFGS_100d", |b| {
            b.iter(|| {
                let _ = optimizer.step(black_box(&mut params), black_box(&gradients), &objective_fn);
            })
        });
    }

    group.finish();
}

fn benchmark_magnitude_computation(c: &mut Criterion) {
    let mut group = c.benchmark_group("magnitude_computation");

    for size in [100, 1000, 10000].iter() {
        let values: Vec<f64> = (0..*size).map(|i| (i as f64) * 0.1).collect();
        let tensors = vec![tensor_from_vec(values)];

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
}

fn benchmark_quadratic_path_evaluation(c: &mut Criterion) {
    let mut group = c.benchmark_group("quadratic_path");

    let dimension = 1000;
    let gradient_values: Vec<f64> = (0..dimension).map(|i| (i as f64) * 0.01).collect();
    let lbfgs_values: Vec<f64> = (0..dimension).map(|i| (i as f64) * 0.02).collect();

    let gradient_tensors = vec![tensor_from_vec(gradient_values)];
    let lbfgs_tensors = vec![tensor_from_vec(lbfgs_values)];

    let config = QQNConfig::default();
    let optimizer = QQNOptimizer::new(config);
    let quadratic_path = optimizer.create_quadratic_path(&gradient_tensors, &lbfgs_tensors).unwrap();

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
}

fn benchmark_full_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_optimization");
    group.sample_size(10); // Fewer samples for longer benchmarks

    let dimension = 50;
    let max_iterations = 100;

    // QQN full optimization
    group.bench_function("QQN_full_rosenbrock", |b| {
        b.iter(|| {
            let problem = RosenbrockFunction::new(dimension);
            let config = QQNConfig::default();
            let mut optimizer = QQNOptimizer::new(config);

            let mut x = problem.initial_point();
            let mut params = vec![tensor_from_vec(x.clone())];
            let objective_fn = |params: &[Tensor]| -> candle_core::Result<f64> {
                let param_vec = tensors_to_vec(params);
                problem.evaluate(&param_vec).map_err(|e| candle_core::Error::Msg(e.to_string()))
            };

            for _ in 0..max_iterations {
                let gradient_vec = problem.gradient(&x).unwrap();
                let gradients = vec![tensor_from_vec(gradient_vec)];

                let _ = optimizer.step(&mut params, &gradients, &objective_fn);
                x = tensors_to_vec(&params);

                let current_value = problem.evaluate(&x).unwrap();
                if current_value < 1e-6 {
                    break;
                }
            }
        })
    });

    // L-BFGS full optimization
    group.bench_function("LBFGS_full_rosenbrock", |b| {
        b.iter(|| {
            let problem = RosenbrockFunction::new(dimension);
            let config = LBFGSConfig::default();
            let mut optimizer = LBFGSOptimizer::new(config);

            let mut x = problem.initial_point();
            let mut params = vec![tensor_from_vec(x.clone())];
            let objective_fn = |params: &[Tensor]| -> candle_core::Result<f64> {
                let param_vec = tensors_to_vec(params);
                problem.evaluate(&param_vec).map_err(|e| candle_core::Error::Msg(e.to_string()))
            };

            for _ in 0..max_iterations {
                let gradient_vec = problem.gradient(&x).unwrap();
                let gradients = vec![tensor_from_vec(gradient_vec)];

                let _ = optimizer.step(&mut params, &gradients, &objective_fn);
                x = tensors_to_vec(&params);

                let current_value = problem.evaluate(&x).unwrap();
                if current_value < 1e-6 {
                    break;
                }
            }
        })
    });

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_usage");

    // Test memory allocation patterns
    for dimension in [100, 500, 1000].iter() {
        group.bench_with_input(
            BenchmarkId::new("optimizer_creation", dimension),
            dimension,
            |b, &dim| {
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