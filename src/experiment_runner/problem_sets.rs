use rand::prelude::StdRng;
use rand::SeedableRng;
use std::sync::Arc;
use crate::benchmarks::evaluation::ProblemSpec;
use crate::benchmarks::ml_problems::{generate_linear_regression_data, generate_svm_data};
use crate::benchmarks::mnist::ActivationType;
use crate::{AckleyFunction, BealeFunction, LinearRegression, LogisticRegression, MnistNeuralNetwork, NeuralNetworkTraining, RastriginFunction, RosenbrockFunction, SphereFunction, SupportVectorMachine};
use crate::benchmarks::analytic_functions::{BarrierFunction, GoldsteinPriceFunction, IllConditionedRosenbrock, LeviFunction, MatyasFunction, NoisySphere, PenaltyFunctionI, SparseQuadratic, SparseRosenbrock, StyblinskiTangFunction, TrigonometricFunction};
use crate::benchmarks::{BoothFunction, GriewankFunction, HimmelblauFunction, LevyFunction, MichalewiczFunction, SchwefelFunction, ZakharovFunction};

pub fn analytic_problems() -> Vec<ProblemSpec> {
    vec![
        ProblemSpec::new(Arc::new(SphereFunction::new(2)), "Sphere".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new(SphereFunction::new(10)), "Sphere".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = RosenbrockFunction::new(2);
            // func.set_optimal_value(Some(8.45e-3));
            func
        }), "Rosenbrock".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = RosenbrockFunction::new(5);
            // func.set_optimal_value(Some(3.98e-1));
            func
        }), "Rosenbrock".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = RosenbrockFunction::new(10);
            // func.set_optimal_value(Some(9.70e0));
            func
        }), "Rosenbrock".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = MichalewiczFunction::new(2);
            // func.set_optimal_value(Some(-9.96e-1));
            func
        }), "Michalewicz".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = MichalewiczFunction::new(5);
            // func.set_optimal_value(Some(-2.69e0));
            func
        }), "Michalewicz".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = MichalewiczFunction::new(10);
            // func.set_optimal_value(Some(-6.26e0));
            func
        }), "Michalewicz".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = RastriginFunction::new(2);
            // func.set_optimal_value(Some(7.96e0));
            func
        }), "Rastrigin".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = RastriginFunction::new(5);
            // func.set_optimal_value(Some(2.04e1));
            func
        }), "Rastrigin".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = RastriginFunction::new(10);
            // func.set_optimal_value(Some(4.18e1));
            func
        }), "Rastrigin".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = AckleyFunction::new(2);
            // func.set_optimal_value(Some(3.57e0));
            func
        }), "Ackley".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = AckleyFunction::new(5);
            // func.set_optimal_value(Some(3.57e0));
            func
        }), "Ackley".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = AckleyFunction::new(10);
            // func.set_optimal_value(Some(3.57e0));
            func
        }), "Ackley".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = StyblinskiTangFunction::new(2);
            // func.set_optimal_value(Some(-7.83e1));
            func
        }), "StyblinskiTang".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = StyblinskiTangFunction::new(5);
            // func.set_optimal_value(Some(-1.95e2));
            func
        }), "StyblinskiTang".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = StyblinskiTangFunction::new(10);
            // func.set_optimal_value(Some(-3.78e2));
            func
        }), "StyblinskiTang".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new(BealeFunction::new()), "Beale".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = LeviFunction::new();
            // func.set_optimal_value(Some(2.84e-1));
            func
        }), "Levi".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = GoldsteinPriceFunction::new();
            // func.set_optimal_value(Some(8.40e1));
            func
        }), "GoldsteinPrice".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new(MatyasFunction::new()), "Matyas".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new(HimmelblauFunction::new()), "Himmelblau".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new(BoothFunction::new()), "Booth".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = GriewankFunction::new(2);
            // func.set_optimal_value(Some(4.54e0));
            func
        }), "Griewank".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = GriewankFunction::new(5);
            // func.set_optimal_value(Some(1.22e1));
            func
        }), "Griewank".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = GriewankFunction::new(10);
            // func.set_optimal_value(Some(2.36e0));
            func
        }), "Griewank".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = SchwefelFunction::new(2);
            // func.set_optimal_value(Some(2.86e2));
            func
        }), "Schwefel".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = SchwefelFunction::new(5);
            // func.set_optimal_value(Some(1.98e1));
            func
        }), "Schwefel".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = SchwefelFunction::new(10);
            // func.set_optimal_value(Some(2.96e3));
            func
        }), "Schwefel".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new(LevyFunction::new(2)), "LevyN".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new(LevyFunction::new(5)), "LevyN".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new(LevyFunction::new(10)), "LevyN".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new(ZakharovFunction::new(2)), "Zakharov".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new(ZakharovFunction::new(5)), "Zakharov".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new(ZakharovFunction::new(10)), "Zakharov".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = IllConditionedRosenbrock::new(2, 100.0);
            // func.set_optimal_value(Some(4.35e-13));
            func
        }), "IllConditionedRosenbrock".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = IllConditionedRosenbrock::new(5, 100.0);
            // func.set_optimal_value(Some(6.80e-1));
            func
        }), "IllConditionedRosenbrock".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = IllConditionedRosenbrock::new(10, 100.0);
            // func.set_optimal_value(Some(1.19e0));
            func
        }), "IllConditionedRosenbrock".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new(TrigonometricFunction::new(2)), "Trigonometric".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = TrigonometricFunction::new(5);
            // func.set_optimal_value(Some(2.17e-15));
            func
        }), "Trigonometric".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = TrigonometricFunction::new(10);
            // func.set_optimal_value(Some(2.50e-14));
            func
        }), "Trigonometric".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = PenaltyFunctionI::new(2);
            // func.set_optimal_value(Some(1.13e0));
            func
        }), "PenaltyI".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = PenaltyFunctionI::new(5);
            // func.set_optimal_value(Some(2.81e0));
            func
        }), "PenaltyI".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = PenaltyFunctionI::new(10);
            // func.set_optimal_value(Some(5.63e0));
            func
        }), "PenaltyI".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = BarrierFunction::new(2);
            // func.set_optimal_value(Some(4.00e-1));
            func
        }), "Barrier".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = BarrierFunction::new(5);
            // func.set_optimal_value(Some(9.99e-1));
            func
        }), "Barrier".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = BarrierFunction::new(10);
            // func.set_optimal_value(Some(2.00e0));
            func
        }), "Barrier".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = NoisySphere::new(2, 0.01);
            // func.set_optimal_value(Some(1.66e0));
            func
        }), "NoisySphere".to_string(), Some(2), 42),
        ProblemSpec::new(Arc::new({
            let mut func = NoisySphere::new(5, 0.01);
            // func.set_optimal_value(Some(4.58e0));
            func
        }), "NoisySphere".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut func = NoisySphere::new(10, 0.01);
            // func.set_optimal_value(Some(9.71e0));
            func
        }), "NoisySphere".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut func = SparseRosenbrock::new(4);
            // func.set_optimal_value(Some(3.24e-2));
            func
        }), "SparseRosenbrock".to_string(), Some(4), 42),
        ProblemSpec::new(Arc::new({
            let mut func = SparseRosenbrock::new(10);
            // func.set_optimal_value(Some(8.99e-2));
            func
        }), "SparseRosenbrock".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new(SparseQuadratic::new(5)), "SparseQuadratic".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new(SparseQuadratic::new(10)), "SparseQuadratic".to_string(), Some(10), 42),
    ]
}

pub fn ml_problems() -> Vec<ProblemSpec> {
    vec![
        ProblemSpec::new(Arc::new({
            let mut regression =
                LogisticRegression::synthetic(100, 5, &mut StdRng::seed_from_u64(42))
                    .expect("Failed to create synthetic logistic regression");
            regression.set_optimal_value(Option::from(3.15e-1));
            regression
        }), "LogisticRegression".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut regression =
                LogisticRegression::synthetic(200, 10, &mut StdRng::seed_from_u64(42))
                    .expect("Failed to create synthetic logistic regression");
            regression.set_optimal_value(Option::from(3.23e-1));
            regression
        }), "LogisticRegression".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut regression = LinearRegression::new(
                generate_linear_regression_data(100, 5, &mut StdRng::seed_from_u64(42)).0,
                generate_linear_regression_data(100, 5, &mut StdRng::seed_from_u64(42)).1,
                0.01,
            )
            .expect("Failed to create linear regression");
            regression.set_optimal_value(Option::from(7.15e-2));
            regression
        }), "LinearRegression".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new({
            let mut regression = LinearRegression::new(
                generate_linear_regression_data(200, 10, &mut StdRng::seed_from_u64(42)).0,
                generate_linear_regression_data(200, 10, &mut StdRng::seed_from_u64(42)).1,
                0.01,
            )
            .expect("Failed to create linear regression");
            regression.set_optimal_value(Option::from(4.82e-1));
            regression
        }), "LinearRegression".to_string(), Some(10), 42),
        ProblemSpec::new(Arc::new({
            let mut training = NeuralNetworkTraining::mlp_classification(
                vec![5, 10, 3],
                &mut StdRng::seed_from_u64(42),
            )
            .expect("Failed to create MLP");
            training.set_optimal_value(Option::from(1.40e-1));
            training
        }), "NeuralNetwork".to_string(), None, 42),
        ProblemSpec::new(Arc::new({
            let mut training = NeuralNetworkTraining::mlp_classification(
                vec![10, 20, 5],
                &mut StdRng::seed_from_u64(42),
            )
            .expect("Failed to create MLP");
            training.set_optimal_value(Option::from(3.82e-2));
            training
        }), "NeuralNetwork".to_string(), None, 42),
        ProblemSpec::new(Arc::new(
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
        ), "SVM".to_string(), Some(5), 42),
        ProblemSpec::new(Arc::new(
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
        ), "SVM".to_string(), Some(10), 42),
    ]
}

pub fn mnist_problems(samples: usize) -> Vec<ProblemSpec> {
    let mut rng = StdRng::seed_from_u64(42);
    vec![
        ProblemSpec::new(Arc::new({
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
        }), "MNIST".to_string(), None, 42)
            .with_name("MNIST_ReLU_20".to_string()),
        ProblemSpec::new(Arc::new({
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
        }), "MNIST".to_string(), None, 42)
            .with_name("MNIST_Logistic_20".to_string()),
        ProblemSpec::new(Arc::new({
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
        }), "MNIST".to_string(), None, 42)
            .with_name("MNIST_ReLU_20x3".to_string()),
        ProblemSpec::new(Arc::new({
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
        }), "MNIST".to_string(), None, 42)
            .with_name("MNIST_Logistic_20x3".to_string()),
        ProblemSpec::new(Arc::new({
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
        }), "MNIST".to_string(), None, 42)
            .with_name("MNIST_Logistic_20x5".to_string()),
    ]
}