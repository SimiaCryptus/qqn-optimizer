use crate::benchmarks::analytic_functions::{
    BarrierFunction, GoldsteinPriceFunction, IllConditionedRosenbrock, LeviFunction,
    MatyasFunction, NoisySphere, PenaltyFunctionI, SparseQuadratic, SparseRosenbrock,
    StyblinskiTangFunction, TrigonometricFunction,
};
use crate::benchmarks::evaluation::ProblemSpec;
use crate::benchmarks::ml_problems::{generate_linear_regression_data, generate_svm_data};
use crate::benchmarks::mnist::ActivationType;
#[cfg(feature = "onednn")]
use crate::benchmarks::mnist_onednn;
use crate::benchmarks::{
    BoothFunction, GriewankFunction, HimmelblauFunction, LevyFunction, MichalewiczFunction,
    SchwefelFunction, ZakharovFunction,
};
#[cfg(feature = "onednn")]
use crate::MnistOneDnnNeuralNetwork;
use crate::{
    AckleyFunction, BealeFunction, LinearRegression, LogisticRegression, MnistNeuralNetwork,
    NeuralNetworkTraining, RastriginFunction, RosenbrockFunction, SphereFunction,
    SupportVectorMachine,
};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::sync::Arc;
/// Standard dimensions used for testing optimization problems
const STANDARD_DIMS: &[usize] = &[2, 5, 10];
/// Helper macro to create problems with multiple dimensions
macro_rules! create_dim_problems {
    ($func:ident, $name:expr, $seed:expr) => {
        STANDARD_DIMS
            .iter()
            .map(|&dim| {
                ProblemSpec::new(Arc::new($func::new(dim)), $name.to_string(), Some(dim), $seed)
            })
            .collect::<Vec<_>>()
    };
}
/// Creates a comprehensive set of analytic benchmark problems for testing optimization algorithms

pub fn analytic_problems() -> Vec<ProblemSpec> {
    vec![
        ProblemSpec::new(
            Arc::new(SphereFunction::new(2)),
            "Sphere".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SphereFunction::new(10)),
            "Sphere".to_string(),
            Some(10),
            42,
        ),
    ]
    .into_iter()
    .chain(create_dim_problems!(RosenbrockFunction, "Rosenbrock", 42))
    .chain(create_dim_problems!(MichalewiczFunction, "Michalewicz", 42))
    .chain(create_dim_problems!(RastriginFunction, "Rastrigin", 42))
    .chain(create_dim_problems!(AckleyFunction, "Ackley", 42))
    .chain(create_dim_problems!(StyblinskiTangFunction, "StyblinskiTang", 42))
    .chain(vec![
        // 2D-only functions
        ProblemSpec::new(
            Arc::new(RastriginFunction::new(2)),
            "Rastrigin".to_string(),
           Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(RastriginFunction::new(5)),
            "Rastrigin".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(RastriginFunction::new(10)),
            "Rastrigin".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(AckleyFunction::new(2)),
            "Ackley".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(AckleyFunction::new(5)),
            "Ackley".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(AckleyFunction::new(10)),
            "Ackley".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(StyblinskiTangFunction::new(2)),
            "StyblinskiTang".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(StyblinskiTangFunction::new(5)),
            "StyblinskiTang".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(StyblinskiTangFunction::new(10)),
            "StyblinskiTang".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(BealeFunction::new()),
            "Beale".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(LeviFunction::new()),
            "Levi".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(GoldsteinPriceFunction::new()),
            "GoldsteinPrice".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(MatyasFunction::new()),
            "Matyas".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(HimmelblauFunction::new()),
            "Himmelblau".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(BoothFunction::new()),
            "Booth".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(GriewankFunction::new(2)),
            "Griewank".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(GriewankFunction::new(5)),
            "Griewank".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(GriewankFunction::new(10)),
            "Griewank".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SchwefelFunction::new(2)),
            "Schwefel".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SchwefelFunction::new(5)),
            "Schwefel".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SchwefelFunction::new(10)),
            "Schwefel".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(LevyFunction::new(2)),
            "LevyN".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(LevyFunction::new(5)),
            "LevyN".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(LevyFunction::new(10)),
            "LevyN".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(ZakharovFunction::new(2)),
            "Zakharov".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(ZakharovFunction::new(5)),
            "Zakharov".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(ZakharovFunction::new(10)),
            "Zakharov".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(IllConditionedRosenbrock::new(2, 100.0)),
            "IllConditionedRosenbrock".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(IllConditionedRosenbrock::new(5, 100.0)),
            "IllConditionedRosenbrock".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(IllConditionedRosenbrock::new(10, 100.0)),
            "IllConditionedRosenbrock".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(TrigonometricFunction::new(2)),
            "Trigonometric".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(TrigonometricFunction::new(5)),
            "Trigonometric".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(TrigonometricFunction::new(10)),
            "Trigonometric".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(PenaltyFunctionI::new(2)),
            "PenaltyI".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(PenaltyFunctionI::new(5)),
            "PenaltyI".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(PenaltyFunctionI::new(10)),
            "PenaltyI".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(BarrierFunction::new(2)),
            "Barrier".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(BarrierFunction::new(5)),
            "Barrier".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(BarrierFunction::new(10)),
            "Barrier".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(NoisySphere::new(2, 0.01)),
            "NoisySphere".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(NoisySphere::new(5, 0.01)),
            "NoisySphere".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(NoisySphere::new(10, 0.01)),
            "NoisySphere".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SparseRosenbrock::new(4)),
            "SparseRosenbrock".to_string(),
            Some(4),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SparseRosenbrock::new(10)),
            "SparseRosenbrock".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SparseQuadratic::new(5)),
            "SparseQuadratic".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(SparseQuadratic::new(10)),
            "SparseQuadratic".to_string(),
            Some(10),
            42,
        ),
    ])
    .chain(create_dim_problems!(GriewankFunction, "Griewank", 42))
    .chain(create_dim_problems!(SchwefelFunction, "Schwefel", 42))
    .chain(create_dim_problems!(LevyFunction, "LevyN", 42))
    .chain(create_dim_problems!(ZakharovFunction, "Zakharov", 42))
    .chain(create_dim_problems!(IllConditionedRosenbrock, "IllConditionedRosenbrock", 42))
    .chain(create_dim_problems!(TrigonometricFunction, "Trigonometric", 42))
    .chain(create_dim_problems!(PenaltyFunctionI, "PenaltyI", 42))
    .chain(create_dim_problems!(BarrierFunction, "Barrier", 42))
    .chain(create_dim_problems!(NoisySphere, "NoisySphere", 42))
    .collect()
}
/// Configuration for ML problem optimal values
struct MLOptimalValues {
    logistic_100_5: f64,
    logistic_200_10: f64,
    linear_100_5: f64,
    linear_200_10: f64,
    nn_small: f64,
    nn_large: f64,
    svm_100_5: f64,
    svm_200_10: f64,
}
const ML_OPTIMAL_VALUES: MLOptimalValues = MLOptimalValues {
    logistic_100_5: 3.15e-1,
    logistic_200_10: 3.23e-1,
    linear_100_5: 7.15e-2,
    linear_200_10: 4.82e-1,
    nn_small: 1.40e-1,
    nn_large: 3.82e-2,
    svm_100_5: 6.43e-1,
    svm_200_10: 6.86e-1,
};
/// Creates a set of machine learning problems for benchmarking

pub fn ml_problems() -> Vec<ProblemSpec> {
    let mut rng = StdRng::seed_from_u64(42);
    
    vec![
        ProblemSpec::new(
            Arc::new({
                let mut regression =
                    LogisticRegression::synthetic(100, 5, &mut rng)
                        .expect("Failed to create synthetic logistic regression");
                regression.set_optimal_value(Some(ML_OPTIMAL_VALUES.logistic_100_5));
                regression
            }),
            "LogisticRegression".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut regression =
                    LogisticRegression::synthetic(200, 10, &mut rng)
                        .expect("Failed to create synthetic logistic regression");
                regression.set_optimal_value(Some(ML_OPTIMAL_VALUES.logistic_200_10));
                regression
            }),
            "LogisticRegression".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let (x, y) = generate_linear_regression_data(100, 5, &mut rng);
                let mut regression = LinearRegression::new(
                    x,
                    y,
                    0.01,
                )
                .expect("Failed to create linear regression");
                regression.set_optimal_value(Some(ML_OPTIMAL_VALUES.linear_100_5));
                regression
            }),
            "LinearRegression".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let (x, y) = generate_linear_regression_data(200, 10, &mut rng);
                let mut regression = LinearRegression::new(
                    x,
                    y,
                    0.01,
                )
                .expect("Failed to create linear regression");
                regression.set_optimal_value(Some(ML_OPTIMAL_VALUES.linear_200_10));
                regression
            }),
            "LinearRegression".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut training = NeuralNetworkTraining::mlp_classification(
                    vec![5, 10, 3],
                    &mut rng,
                )
                .expect("Failed to create MLP");
                training.set_optimal_value(Some(ML_OPTIMAL_VALUES.nn_small));
                training
            }),
            "NeuralNetwork".to_string(),
            None,
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut training = NeuralNetworkTraining::mlp_classification(
                    vec![10, 20, 5],
                    &mut rng,
                )
                .expect("Failed to create MLP");
                training.set_optimal_value(Some(ML_OPTIMAL_VALUES.nn_large));
                training
            }),
            "NeuralNetwork".to_string(),
            None,
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let (x, y) = generate_svm_data(100, 5, &mut rng);
                let mut svm = SupportVectorMachine::new(
                    x,
                    y,
                    1.0,
                )
                .expect("Failed to create SVM");
                svm.set_optimal_value(Some(ML_OPTIMAL_VALUES.svm_100_5));
                svm
            }),
            "SVM".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let (x, y) = generate_svm_data(200, 10, &mut rng);
                let mut svm = SupportVectorMachine::new(
                    x,
                    y,
                    1.0,
                )
                .expect("Failed to create SVM");
                svm.set_optimal_value(Some(ML_OPTIMAL_VALUES.svm_200_10));
                svm
            }),
            "SVM".to_string(),
            Some(10),
            42,
        ),
    ]
}
/// Creates MNIST neural network problems with various architectures
/// 
/// # Arguments
/// * `samples` - Number of training samples to use

pub fn mnist_problems(samples: usize) -> Vec<ProblemSpec> {
    let mut rng = StdRng::seed_from_u64(42);
    const MNIST_OPTIMAL_VALUE: f64 = 0.05;
    
    vec![
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistNeuralNetwork::create(
                    Some(samples),
                    &[20],
                    Some(samples),
                    &mut rng,
                    Some(ActivationType::ReLU),
                )
                .expect("Failed to create MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST".to_string(),
            None,
            42,
        )
        .with_name("MNIST_ReLU_20".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistNeuralNetwork::create(
                    Some(samples),
                    &[20],
                    Some(samples),
                    &mut rng,
                    Some(ActivationType::Logistic),
                )
                .expect("Failed to create MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST".to_string(),
            None,
            42,
        )
        .with_name("MNIST_Logistic_20".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(ActivationType::ReLU),
                )
                .expect("Failed to create MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST".to_string(),
            None,
            42,
        )
        .with_name("MNIST_ReLU_20x3".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(ActivationType::Logistic),
                )
                .expect("Failed to create MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST".to_string(),
            None,
            42,
        )
        .with_name("MNIST_Logistic_20x3".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(ActivationType::Logistic),
                )
                .expect("Failed to create MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST".to_string(),
            None,
            42,
        )
        .with_name("MNIST_Logistic_20x5".to_string()),
    ]
}

#[cfg(feature = "onednn")]
/// Creates MNIST neural network problems using OneDNN backend
/// 
/// # Arguments
/// * `samples` - Number of training samples to use
pub fn mnist_onednn_problems(samples: usize) -> Vec<ProblemSpec> {
    let mut rng = StdRng::seed_from_u64(42);
    const MNIST_OPTIMAL_VALUE: f64 = 0.05;
    
    vec![
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistOneDnnNeuralNetwork::create(
                    Some(samples),
                    &[20],
                    Some(samples),
                    &mut rng,
                    Some(mnist_onednn::ActivationType::ReLU),
                )
                .expect("Failed to create OneDNN MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST_OneDNN".to_string(),
            None,
            42,
        )
        .with_name("MNIST_OneDNN_ReLU_20".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistOneDnnNeuralNetwork::create(
                    Some(samples),
                    &[20],
                    Some(samples),
                    &mut rng,
                    Some(mnist_onednn::ActivationType::Logistic),
                )
                .expect("Failed to create OneDNN MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST_OneDNN".to_string(),
            None,
            42,
        )
        .with_name("MNIST_OneDNN_Logistic_20".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistOneDnnNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(mnist_onednn::ActivationType::ReLU),
                )
                .expect("Failed to create OneDNN MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST_OneDNN".to_string(),
            None,
            42,
        )
        .with_name("MNIST_OneDNN_ReLU_20x3".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistOneDnnNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(mnist_onednn::ActivationType::Tanh),
                )
                .expect("Failed to create OneDNN MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST_OneDNN".to_string(),
            None,
            42,
        )
        .with_name("MNIST_OneDNN_Tanh_20x3".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = MnistOneDnnNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(mnist_onednn::ActivationType::Tanh),
                )
                .expect("Failed to create OneDNN MNIST neural network");
                network.set_optimal_value(Some(MNIST_OPTIMAL_VALUE));
                network
            }),
            "MNIST_OneDNN".to_string(),
            None,
            42,
        )
        .with_name("MNIST_OneDNN_Tanh_20x5".to_string()),
    ]
}

#[cfg(not(feature = "onednn"))]
/// Stub implementation when OneDNN feature is not enabled
pub fn mnist_onednn_problems(_samples: usize) -> Vec<ProblemSpec> {
    vec![] // Return empty vector when OneDNN feature is not enabled
}