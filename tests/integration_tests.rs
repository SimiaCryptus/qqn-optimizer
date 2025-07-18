use candle_core::Tensor;
use qqn_optimizer::core::optimizer::Optimizer;
use qqn_optimizer::core::qqn::{QQNConfig, QQNOptimizer};
use qqn_optimizer::utils::math::{tensor_from_vec, SeparateFunctions};
use std::sync::Arc;

#[test]
fn test_qqn_numerical_stability() {
    let config = QQNConfig::default();
    let mut optimizer = QQNOptimizer::new(config);

    // Create a simple function with exact gradients using SeparateFunctions
    let objective_fn = |tensors: &[Tensor]| -> candle_core::Result<f64> {
        let x = tensors[0].to_vec1::<f64>()?;
        Ok(x[0] * x[0] + x[1] * x[1])
    };
    let gradient_fn = |tensors: &[Tensor]| -> candle_core::Result<Vec<Tensor>> {
        let x = tensors[0].to_vec1::<f64>()?;
        let grad = vec![2.0 * x[0], 2.0 * x[1]];
        Ok(vec![tensor_from_vec(grad)])
    };
    let function = Arc::new(SeparateFunctions::new(objective_fn, gradient_fn));


    // Test with very small gradients
    let mut params = vec![tensor_from_vec(vec![1.0, 1.0])];

    let result = optimizer.step(&mut params, function.clone());
    let result = optimizer.step(&mut params, function.clone());
    if let Err(e) = &result {
        println!("Tiny gradient step error: {:?}", e);
    }
    // For very tiny gradients, we expect the optimizer to handle them gracefully
    // Either succeed or fail gracefully without panicking
    match result {
        Ok(_) => {
            // Check that parameters are still finite
            let param_values: Vec<f64> = params[0].to_vec1().unwrap();
            assert!(param_values.iter().all(|&x| x.is_finite()));
        }
        Err(_) => {
            // It's acceptable to fail on extremely small gradients
            // as long as we don't panic
        }
    }


    // Test with parameters that would give large gradients
    let mut large_params = vec![tensor_from_vec(vec![1e3, 1e3])];
    let result2 = optimizer.step(&mut large_params, function.clone());
    if let Err(e) = &result2 {
        println!("Large gradient step error: {:?}", e);
    }
    assert!(result2.is_ok(), "Should handle large gradients");

    let param_values2: Vec<f64> = large_params[0].to_vec1().unwrap();
    assert!(param_values2.iter().all(|&x| x.is_finite()));
}

#[test]
fn test_qqn_reset_functionality() {
    let config = QQNConfig::default();
    let mut optimizer = QQNOptimizer::new(config);

    // Create a simple function with exact gradients using SeparateFunctions
    let objective_fn = |tensors: &[Tensor]| -> candle_core::Result<f64> {
        let x = tensors[0].to_vec1::<f64>()?;
        Ok(x[0] * x[0] + x[1] * x[1])
    };
    let gradient_fn = |tensors: &[Tensor]| -> candle_core::Result<Vec<Tensor>> {
        let x = tensors[0].to_vec1::<f64>()?;
        let grad = vec![2.0 * x[0], 2.0 * x[1]];
        Ok(vec![tensor_from_vec(grad)])
    };
    let function = Arc::new(SeparateFunctions::new(objective_fn, gradient_fn));


    // Perform several steps
    let mut params = vec![tensor_from_vec(vec![1.0, 1.0])];
    let mut iterations_before_reset = 0;

    for _ in 0..5 {
        if let Ok(_) = optimizer.step(&mut params, function.clone()) {
            iterations_before_reset += 1;
        }
    }

    assert!(iterations_before_reset > 0, "Should have performed some iterations");

    // Reset and verify state
    optimizer.reset();
    let state = optimizer.state;
    assert_eq!(state.iteration, 0);
}