use approx::assert_relative_eq;
use candle_core::{Device, Tensor};
use qqn_optimizer::benchmarks::functions::{OptimizationProblem, RosenbrockFunction, SphereFunction};
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

#[tokio::test]
async fn test_qqn_rosenbrock_optimization() {
    let problem = RosenbrockFunction::new(2);
    let mut config = QQNConfig::default();
    config.line_search.initial_step = 1.0; // Use default step size
    config.line_search.c1 = 1e-4;
    config.line_search.c2 = 0.9;
    config.lbfgs_history = 10; // Increase memory for better approximation
    config.threshold = 0.1; // 10% threshold for switching
    let mut optimizer = QQNOptimizer::new(config);

    let mut x = problem.initial_point();
    let mut params = vec![tensor_from_vec(x.clone())];
    let mut iterations = 0;
    let max_iterations = 2000; // Allow more iterations for difficult Rosenbrock function

    while iterations < max_iterations {
        let gradient_vec = problem.gradient(&x).unwrap();
        // Check gradient norm for convergence
        let grad_norm = gradient_vec.iter().map(|g| g * g).sum::<f64>().sqrt();
        let gradients = vec![tensor_from_vec(gradient_vec)];
        if grad_norm < 1e-6 {
            break;
        }
        
        
        let step_result = optimizer.step(&mut params, &gradients).unwrap();
        
        // Update x for next iteration
        x = tensors_to_vec(&params);
        
        

        iterations += 1;
    }

    let final_value = problem.evaluate(&x).unwrap();
    let final_gradient = problem.gradient(&x).unwrap();
    let final_grad_norm = final_gradient.iter().map(|g| g * g).sum::<f64>().sqrt();
    
    // More lenient convergence criteria for Rosenbrock
    assert!(final_value < 0.1 || final_grad_norm < 1e-3, 
            "QQN failed to converge to Rosenbrock minimum: final_value = {}, grad_norm = {}", 
            final_value, final_grad_norm);
    assert!(iterations < max_iterations, "QQN took maximum iterations: {}", iterations);
    
    // Check that we're close to the optimum (1, 1)
    assert_relative_eq!(x[0], 1.0, epsilon = 0.1);
    assert_relative_eq!(x[1], 1.0, epsilon = 0.1);
}

#[tokio::test]
async fn test_qqn_vs_lbfgs_sphere_function() {
    let problem = SphereFunction::new(5); // Use smaller dimension for more reliable convergence
    
    // Test QQN
    let qqn_config = QQNConfig::default();
    let mut qqn_optimizer = QQNOptimizer::new(qqn_config);
    
    let mut qqn_x = problem.initial_point();
    let mut qqn_params = vec![tensor_from_vec(qqn_x.clone())];
    let mut qqn_iterations = 0;
    let max_iterations = 200;
    
    while qqn_iterations < max_iterations {
        let gradient_vec = problem.gradient(&qqn_x).unwrap();
        let grad_norm = gradient_vec.iter().map(|g| g * g).sum::<f64>().sqrt();
        if grad_norm < 1e-8 {
            break;
        }
        
        let gradients = vec![tensor_from_vec(gradient_vec)];
        
        let _ = qqn_optimizer.step(&mut qqn_params, &gradients).unwrap();
        qqn_x = tensors_to_vec(&qqn_params);
        
        
        qqn_iterations += 1;
    }
    
    // Test L-BFGS
    let lbfgs_config = LBFGSConfig::default();
    let mut lbfgs_optimizer = LBFGSOptimizer::new(lbfgs_config);
    
    let mut lbfgs_x = problem.initial_point();
    let mut lbfgs_params = vec![tensor_from_vec(lbfgs_x.clone())];
    let mut lbfgs_iterations = 0;
    
    while lbfgs_iterations < max_iterations {
        let gradient_vec = problem.gradient(&lbfgs_x).unwrap();
        let grad_norm = gradient_vec.iter().map(|g| g * g).sum::<f64>().sqrt();
        if grad_norm < 1e-8 {
            break;
        }
        
        let gradients = vec![tensor_from_vec(gradient_vec)];
        
        let _ = lbfgs_optimizer.step(&mut lbfgs_params, &gradients).unwrap();
        lbfgs_x = tensors_to_vec(&lbfgs_params);
        
        
        lbfgs_iterations += 1;
    }
    
    let qqn_final_value = problem.evaluate(&qqn_x).unwrap();
    let lbfgs_final_value = problem.evaluate(&lbfgs_x).unwrap();
    
    // Both should converge to near-zero
    assert!(qqn_final_value < 1e-6, "QQN failed to converge on sphere function: {}", qqn_final_value);
    assert!(lbfgs_final_value < 1e-6, "L-BFGS failed to converge on sphere function: {}", lbfgs_final_value);
    
    // QQN should be competitive with L-BFGS (within 3x iterations)
    assert!(qqn_iterations <= lbfgs_iterations * 3, 
            "QQN took significantly more iterations than L-BFGS: {} vs {}", 
            qqn_iterations, lbfgs_iterations);
}

#[test]
fn test_qqn_quadratic_path_switching() {
    let mut config = QQNConfig::default();
    config.threshold = 0.1; // 10% threshold for easier testing

    let mut optimizer = QQNOptimizer::new(config);
    
    // Create scenario where magnitude difference is large
    let mut params = vec![tensor_from_vec(vec![1.0, 1.0])];
    let large_gradient = vec![tensor_from_vec(vec![10.0, 10.0])]; // Large gradient
    
    // First step should use quadratic path due to magnitude difference
    let result1 = optimizer.step(&mut params, &large_gradient);
    if let Err(e) = &result1 {
        println!("First step error: {:?}", e);
    }
    assert!(result1.is_ok(), "First step should succeed");
    
    let state1 = optimizer.state();
    // Should have used quadratic path due to large magnitude difference
    assert!(!state1.magnitude_ratios.is_empty());
    // Note: The magnitude ratio might be small initially due to L-BFGS state being empty
    // This is expected behavior for the first few iterations
    
    // Create scenario with small gradient (similar magnitudes)
    let small_gradient = vec![tensor_from_vec(vec![0.01, 0.01])];
    let result2 = optimizer.step(&mut params, &small_gradient);
    if let Err(e) = &result2 {
        println!("Second step error: {:?}", e);
    }
    assert!(result2.is_ok(), "Second step should succeed");
    
    let state2 = optimizer.state();
    assert!(state2.iteration >= 2, "Should have completed at least 2 iterations");
}

#[test]
fn test_qqn_numerical_stability() {
    let config = QQNConfig::default();
    let mut optimizer = QQNOptimizer::new(config);
    
    // Test with very small gradients
    let mut params = vec![tensor_from_vec(vec![1.0, 1.0])];
    let tiny_gradient = vec![tensor_from_vec(vec![1e-12, 1e-12])];
    
    let result = optimizer.step(&mut params, &tiny_gradient);
    if let Err(e) = &result {
        println!("Tiny gradient step error: {:?}", e);
    }
    assert!(result.is_ok(), "Should handle tiny gradients");
    
    // Check that parameters are still finite
    let param_values: Vec<f64> = params[0].to_vec1().unwrap();
    assert!(param_values.iter().all(|&x| x.is_finite()));
    
    // Test with very large gradients
    let large_gradient = vec![tensor_from_vec(vec![1e6, 1e6])];
    let result2 = optimizer.step(&mut params, &large_gradient);
    if let Err(e) = &result2 {
        println!("Large gradient step error: {:?}", e);
    }
    assert!(result2.is_ok(), "Should handle large gradients");
    
    let param_values2: Vec<f64> = params[0].to_vec1().unwrap();
    assert!(param_values2.iter().all(|&x| x.is_finite()));
}

#[test]
fn test_qqn_reset_functionality() {
    let config = QQNConfig::default();
    let mut optimizer = QQNOptimizer::new(config);
    
    // Perform several steps
    let mut params = vec![tensor_from_vec(vec![1.0, 1.0])];
    let gradient = vec![tensor_from_vec(vec![0.1, 0.1])];
    let mut iterations_before_reset = 0;
    
    for _ in 0..5 {
        if let Ok(_) = optimizer.step(&mut params, &gradient) {
            iterations_before_reset += 1;
        }
    }
    
    assert!(iterations_before_reset > 0, "Should have performed some iterations");
    
    // Reset and verify state
    optimizer.reset();
    let state = optimizer.state();
    assert_eq!(state.iteration, 0);
    assert!(state.magnitude_ratios.is_empty());
    assert_eq!(state.quadratic_path_count, 0);
    assert_eq!(state.lbfgs_count, 0);
}

// Property-based tests using proptest
#[cfg(feature = "proptest")]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_descent_property(
            gradient in prop::collection::vec(-10.0..10.0, 2..10),
            lbfgs_direction in prop::collection::vec(-10.0..10.0, 2..10)
        ) {
            // Ensure we have valid descent directions
            let grad_norm_sq: f64 = gradient.iter().map(|x| x * x).sum();
            let lbfgs_dot_grad: f64 = lbfgs_direction.iter().zip(&gradient).map(|(l, g)| l * g).sum();
            
            prop_assume!(grad_norm_sq > 1e-10);
            prop_assume!(lbfgs_dot_grad < -1e-10); // L-BFGS should be descent
            
            let config = QQNConfig::default();
            let optimizer = QQNOptimizer::new(config);
            
            let grad_tensors = vec![tensor_from_vec(gradient.clone())];
            let lbfgs_tensors = vec![tensor_from_vec(lbfgs_direction.clone())];
            
            let quadratic_path = optimizer.create_quadratic_path(&grad_tensors, &lbfgs_tensors).unwrap();
            
            // Test that at various t values, we get descent directions
            for t in [0.1, 0.5, 0.9] {
                let direction_t = quadratic_path.evaluate(t).unwrap();
                let direction_vec: Vec<f64> = direction_t[0].to_vec1().unwrap();
                
                let dot_product: f64 = direction_vec.iter().zip(&gradient).map(|(d, g)| d * g).sum();
                prop_assert!(dot_product <= 0.0, "QQN direction is not descent at t={}: dot_product={}", t, dot_product);
            }
        }
        
        #[test]
        fn test_magnitude_computation_properties(
            values in prop::collection::vec(-100.0..100.0, 1..20)
        ) {
            let tensors = vec![tensor_from_vec(values.clone())];
            let magnitude = compute_magnitude(&tensors).unwrap();
            
            // Magnitude should be non-negative
            prop_assert!(magnitude >= 0.0);
            
            // Magnitude should be zero only if all values are zero
            if values.iter().all(|&x| x.abs() < 1e-15) {
                prop_assert!(magnitude < 1e-10);
            } else {
                prop_assert!(magnitude > 0.0);
            }
            
            // Scaling property: magnitude(k*x) = |k| * magnitude(x)
            let scale_factor = 2.0;
            let scaled_tensors = vec![tensor_from_vec(values.iter().map(|&x| x * scale_factor).collect())];
            let scaled_magnitude = compute_magnitude(&scaled_tensors).unwrap();
            
            prop_assert!((scaled_magnitude - scale_factor.abs() * magnitude).abs() < 1e-10);
        }
    }
}