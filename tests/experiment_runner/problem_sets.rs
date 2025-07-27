use qqn_optimizer::benchmarks::analytic_functions::{
    GoldsteinPriceFunction, LeviFunction, MatyasFunction, StyblinskiTangFunction,
};
use qqn_optimizer::benchmarks::ml_problems::{generate_linear_regression_data, generate_svm_data};
use qqn_optimizer::benchmarks::mnist::ActivationType;
use qqn_optimizer::benchmarks::MichalewiczFunction;
use qqn_optimizer::{
    AckleyFunction, BealeFunction, LinearRegression, LogisticRegression, MnistNeuralNetwork,
    NeuralNetworkTraining, OptimizationProblem, RastriginFunction, RosenbrockFunction,
    SphereFunction, SupportVectorMachine,
};
use rand::prelude::StdRng;
use rand::SeedableRng;
use std::sync::Arc;

pub fn analytic_problems() -> Vec<Arc<dyn OptimizationProblem>> {
    vec![
        Arc::new(SphereFunction::new(2)),
        Arc::new(SphereFunction::new(10)),
        Arc::new(RosenbrockFunction::new(2)),
        Arc::new(RosenbrockFunction::new(5)),
        Arc::new(RosenbrockFunction::new(10)),
        Arc::new(MichalewiczFunction::new(2)),
        Arc::new(MichalewiczFunction::new(5)),
        Arc::new(MichalewiczFunction::new(10)),
        Arc::new(RastriginFunction::new(2)),
        Arc::new(RastriginFunction::new(5)),
        Arc::new(RastriginFunction::new(10)),
        Arc::new(AckleyFunction::new(2)),
        Arc::new(AckleyFunction::new(5)),
        Arc::new(AckleyFunction::new(10)),
        Arc::new(StyblinskiTangFunction::new(2)),
        Arc::new(StyblinskiTangFunction::new(5)),
        Arc::new(StyblinskiTangFunction::new(10)),
        Arc::new(BealeFunction::new()),
        Arc::new(LeviFunction::new()),
        Arc::new(GoldsteinPriceFunction::new()),
        Arc::new(MatyasFunction::new()),
    ]
}

pub fn ml_problems() -> Vec<Arc<dyn OptimizationProblem>> {
    vec![
        Arc::new({
            let mut regression =
                LogisticRegression::synthetic(100, 5, &mut StdRng::seed_from_u64(42))
                    .expect("Failed to create synthetic logistic regression");
            regression.set_optimal_value(Option::from(3.15e-1));
            regression
        }),
        Arc::new({
            let mut regression =
                LogisticRegression::synthetic(200, 10, &mut StdRng::seed_from_u64(42))
                    .expect("Failed to create synthetic logistic regression");
            regression.set_optimal_value(Option::from(3.23e-1));
            regression
        }),
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
        Arc::new({
            let mut training = NeuralNetworkTraining::mlp_classification(
                vec![5, 10, 3],
                &mut StdRng::seed_from_u64(42),
            )
            .expect("Failed to create MLP");
            training.set_optimal_value(Option::from(1.36e-1));
            training
        }),
        Arc::new({
            let mut training = NeuralNetworkTraining::mlp_classification(
                vec![10, 20, 5],
                &mut StdRng::seed_from_u64(42),
            )
            .expect("Failed to create MLP");
            training.set_optimal_value(Option::from(3.54e-2));
            training
        }),
        Arc::new(
            {
                let mut svm = SupportVectorMachine::new(
                generate_svm_data(100, 5, &mut StdRng::seed_from_u64(42)).0,
                generate_svm_data(100, 5, &mut StdRng::seed_from_u64(42)).1,
                1.0,
                )
                .expect("Failed to create SVM");
                svm.set_optimal_value(Option::from(6.43e-1));
                svm
            }
        ),
        Arc::new(
            {
                let mut svm = SupportVectorMachine::new(
                generate_svm_data(200, 10, &mut StdRng::seed_from_u64(42)).0,
                generate_svm_data(200, 10, &mut StdRng::seed_from_u64(42)).1,
                1.0,
                )
                .expect("Failed to create SVM");
                svm.set_optimal_value(Option::from(6.86e-1));
                svm
            }
        ),
    ]
}

pub fn mnist_problems(samples: usize) -> Vec<Arc<dyn OptimizationProblem>> {
    let mut rng = StdRng::seed_from_u64(42);
    vec![
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
        Arc::new({
            let mut network = MnistNeuralNetwork::create(
                Some(samples),
                &[
                    // 20
                    20,
                ],
                Some(samples),
                &mut rng,
                Some(ActivationType::Logistic),
            )
            .expect("Failed to create MNIST neural network");
            network.set_optimal_value(Option::from(0.05));
            network
        }),
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
        Arc::new({
            let mut network = MnistNeuralNetwork::create(
                Some(samples),
                &[
                    // 20
                    20, 20, 20,
                ],
                Some(samples),
                &mut rng,
                Some(ActivationType::Logistic),
            )
            .expect("Failed to create MNIST neural network");
            network.set_optimal_value(Option::from(0.05));
            network
        }),
        Arc::new({
            let mut network = MnistNeuralNetwork::create(
                Some(samples),
                &[
                    // 20
                    20, 20, 20, 20, 20,
                ],
                Some(samples),
                &mut rng,
                Some(ActivationType::Logistic),
            )
            .expect("Failed to create MNIST neural network");
            network.set_optimal_value(Option::from(0.05));
            network
        }),
    ]
}