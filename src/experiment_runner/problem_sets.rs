use crate::benchmarks::analytic_functions::{
    BarrierFunction, GoldsteinPriceFunction, IllConditionedRosenbrock, LeviFunction,
    MatyasFunction, NoisySphere, PenaltyFunctionI, SparseQuadratic, SparseRosenbrock,
    StyblinskiTangFunction, TrigonometricFunction,
};
use crate::benchmarks::evaluation::ProblemSpec;
use crate::benchmarks::ml_problems::{generate_linear_regression_data, generate_svm_data};
use crate::benchmarks::mnist::ActivationType;
use crate::benchmarks::fashion_mnist::ActivationType as FashionActivationType;
use crate::benchmarks::{
    BoothFunction, GriewankFunction, HimmelblauFunction, LevyFunction, MichalewiczFunction,
    SchwefelFunction, ZakharovFunction,
};
use crate::{
    AckleyFunction, BealeFunction, LinearRegression, LogisticRegression, MnistNeuralNetwork,
    FashionMnistNeuralNetwork, NeuralNetworkTraining, RastriginFunction, RosenbrockFunction, 
    SphereFunction, SupportVectorMachine,
};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::sync::Arc;

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
        ProblemSpec::new(
            Arc::new(RosenbrockFunction::new(2)),
            "Rosenbrock".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(RosenbrockFunction::new(5)),
            "Rosenbrock".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(RosenbrockFunction::new(10)),
            "Rosenbrock".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(MichalewiczFunction::new(2)),
            "Michalewicz".to_string(),
            Some(2),
            42,
        ),
        ProblemSpec::new(
            Arc::new(MichalewiczFunction::new(5)),
            "Michalewicz".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new(MichalewiczFunction::new(10)),
            "Michalewicz".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new(RastriginFunction::new(2)),
            "Rastrigin".to_string(),
            Some(2),
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
    ]
}

pub fn ml_problems() -> Vec<ProblemSpec> {
    vec![
        ProblemSpec::new(
            Arc::new({
                let mut regression =
                    LogisticRegression::synthetic(100, 5, &mut StdRng::seed_from_u64(42))
                        .expect("Failed to create synthetic logistic regression");
                regression.set_optimal_value(Option::from(3.15e-1));
                regression
            }),
            "LogisticRegression".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut regression =
                    LogisticRegression::synthetic(200, 10, &mut StdRng::seed_from_u64(42))
                        .expect("Failed to create synthetic logistic regression");
                regression.set_optimal_value(Option::from(3.23e-1));
                regression
            }),
            "LogisticRegression".to_string(),
            Some(10),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut regression = LinearRegression::new(
                    generate_linear_regression_data(100, 5, &mut StdRng::seed_from_u64(42)).0,
                    generate_linear_regression_data(100, 5, &mut StdRng::seed_from_u64(42)).1,
                    0.01,
                )
                .expect("Failed to create linear regression");
                regression.set_optimal_value(Option::from(7.15e-2));
                regression
            }),
            "LinearRegression".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut regression = LinearRegression::new(
                    generate_linear_regression_data(200, 10, &mut StdRng::seed_from_u64(42)).0,
                    generate_linear_regression_data(200, 10, &mut StdRng::seed_from_u64(42)).1,
                    0.01,
                )
                .expect("Failed to create linear regression");
                regression.set_optimal_value(Option::from(4.82e-1));
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
                    &mut StdRng::seed_from_u64(42),
                )
                .expect("Failed to create MLP");
                training.set_optimal_value(Option::from(1.40e-1));
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
                    &mut StdRng::seed_from_u64(42),
                )
                .expect("Failed to create MLP");
                training.set_optimal_value(Option::from(3.82e-2));
                training
            }),
            "NeuralNetwork".to_string(),
            None,
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut svm = SupportVectorMachine::new(
                    generate_svm_data(100, 5, &mut StdRng::seed_from_u64(42)).0,
                    generate_svm_data(100, 5, &mut StdRng::seed_from_u64(42)).1,
                    1.0,
                )
                .expect("Failed to create SVM");
                svm.set_optimal_value(Option::from(6.43e-1));
                svm
            }),
            "SVM".to_string(),
            Some(5),
            42,
        ),
        ProblemSpec::new(
            Arc::new({
                let mut svm = SupportVectorMachine::new(
                    generate_svm_data(200, 10, &mut StdRng::seed_from_u64(42)).0,
                    generate_svm_data(200, 10, &mut StdRng::seed_from_u64(42)).1,
                    1.0,
                )
                .expect("Failed to create SVM");
                svm.set_optimal_value(Option::from(6.86e-1));
                svm
            }),
            "SVM".to_string(),
            Some(10),
            42,
        ),
    ]
}

pub fn mnist_problems(samples: usize) -> Vec<ProblemSpec> {
    let mut rng = StdRng::seed_from_u64(42);
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
                network.set_optimal_value(Option::from(0.05));
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
                network.set_optimal_value(Option::from(0.05));
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
                network.set_optimal_value(Option::from(0.05));
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
                network.set_optimal_value(Option::from(0.05));
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
                network.set_optimal_value(Option::from(0.05));
                network
            }),
            "MNIST".to_string(),
            None,
            42,
        )
        .with_name("MNIST_Logistic_20x5".to_string()),
    ]
}

pub fn fashion_mnist_problems(samples: usize) -> Vec<ProblemSpec> {
    let mut rng = StdRng::seed_from_u64(42);
    vec![
        ProblemSpec::new(
            Arc::new({
                let mut network = FashionMnistNeuralNetwork::create(
                    Some(samples),
                    &[20],
                    Some(samples),
                    &mut rng,
                    Some(FashionActivationType::ReLU),
                )
                .expect("Failed to create Fashion-MNIST neural network");
                network.set_optimal_value(Option::from(0.08));
                network
            }),
            "FashionMNIST".to_string(),
            None,
            42,
        )
        .with_name("FashionMNIST_ReLU_20".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = FashionMnistNeuralNetwork::create(
                    Some(samples),
                    &[20],
                    Some(samples),
                    &mut rng,
                    Some(FashionActivationType::Logistic),
                )
                .expect("Failed to create Fashion-MNIST neural network");
                network.set_optimal_value(Option::from(0.08));
                network
            }),
            "FashionMNIST".to_string(),
            None,
            42,
        )
        .with_name("FashionMNIST_Logistic_20".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = FashionMnistNeuralNetwork::create(
                    Some(samples),
                    &[30],
                    Some(samples),
                    &mut rng,
                    Some(FashionActivationType::ReLU),
                )
                .expect("Failed to create Fashion-MNIST neural network");
                network.set_optimal_value(Option::from(0.07));
                network
            }),
            "FashionMNIST".to_string(),
            None,
            42,
        )
        .with_name("FashionMNIST_ReLU_30".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = FashionMnistNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(FashionActivationType::ReLU),
                )
                .expect("Failed to create Fashion-MNIST neural network");
                network.set_optimal_value(Option::from(0.06));
                network
            }),
            "FashionMNIST".to_string(),
            None,
            42,
        )
        .with_name("FashionMNIST_ReLU_20x3".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = FashionMnistNeuralNetwork::create(
                    Some(samples),
                    &[20, 20, 20],
                    Some(samples),
                    &mut rng,
                    Some(FashionActivationType::Logistic),
                )
                .expect("Failed to create Fashion-MNIST neural network");
                network.set_optimal_value(Option::from(0.06));
                network
            }),
            "FashionMNIST".to_string(),
            None,
            42,
        )
        .with_name("FashionMNIST_Logistic_20x3".to_string()),
        ProblemSpec::new(
            Arc::new({
                let mut network = FashionMnistNeuralNetwork::create(
                    Some(samples),
                    &[15, 25, 15],
                    Some(samples),
                    &mut rng,
                    Some(FashionActivationType::Sinewave),
                )
                .expect("Failed to create Fashion-MNIST neural network");
                network.set_optimal_value(Option::from(0.09));
                network
            }),
            "FashionMNIST".to_string(),
            None,
            42,
        )
        .with_name("FashionMNIST_Sinewave_15x25x15".to_string()),
    ]
}
