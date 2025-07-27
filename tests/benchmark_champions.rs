use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

mod experiment_runner;
use crate::experiment_runner::experiment_runner::run_benchmark;
use crate::experiment_runner::optimizer_sets::standard_optimizers;
use crate::experiment_runner::problem_sets::{analytic_problems, ml_problems, mnist_problems};
use qqn_optimizer::{init_logging, OptimizationProblem};
use qqn_optimizer::benchmarks::evaluation::enable_no_threshold_mode;

/// Championship configuration mapping problem names to their champion optimizers
fn get_championship_config() -> HashMap<String, Vec<String>> {
    let mut champions = HashMap::new();
    
    let enable_mnist = std::env::var("ENABLE_MNIST").map_or(false, |v| v == "1");
    
    // Analytic function champions (based on typical performance characteristics)
    champions.insert("Sphere_2D".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS".to_string(), 
        "Trust Region-Conservative".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Sphere_10D".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS-Aggressive".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Rosenbrock_2D".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS".to_string(),
        "Trust Region-Aggressive".to_string(),
        "GD".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("Rosenbrock_5D".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Aggressive".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Rosenbrock_10D".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Aggressive".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Michalewicz_2D_m10".to_string(), vec![
        "QQN-StrongWolfe".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("Michalewicz_5D_m10".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Michalewicz_10D_m10".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-Nesterov".to_string(),
        "Adam-AMSGrad".to_string(),
    ]);
    
    champions.insert("Rastrigin_2D".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-WeightDecay".to_string(),
    ]);
    
    champions.insert("Rastrigin_5D".to_string(), vec![
        "QQN-GoldenSection".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region".to_string(),
        "GD".to_string(),
        "Adam-WeightDecay".to_string(),
    ]);
    
    champions.insert("Rastrigin_10D".to_string(), vec![
        "QQN-GoldenSection".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("Ackley_2D_a20_b0.2_c6.28e0".to_string(), vec![
        "QQN-GoldenSection".to_string(),
        "L-BFGS".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Ackley_5D_a20_b0.2_c6.28e0".to_string(), vec![
        "QQN-StrongWolfe".to_string(),
        "L-BFGS".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-Nesterov".to_string(),
        "Adam-WeightDecay".to_string(),
    ]);
    
    champions.insert("Ackley_10D_a20_b0.2_c6.28e0".to_string(), vec![
        "QQN-GoldenSection".to_string(),
        "L-BFGS".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("StyblinskiTang_2D".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS".to_string(),
        "Trust Region".to_string(),
        "GD".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("StyblinskiTang_5D".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS".to_string(),
        "Trust Region".to_string(),
        "GD".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("StyblinskiTang_10D".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS-Aggressive".to_string(),
        "Trust Region".to_string(),
        "GD".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("Beale_2D".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS".to_string(),
        "Trust Region".to_string(),
        "GD-Nesterov".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Levi_2D".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-AMSGrad".to_string(),
    ]);
    
    champions.insert("GoldsteinPrice_2D".to_string(), vec![
        "QQN-Bisection-1".to_string(),
        "L-BFGS".to_string(),
        "Trust Region-Aggressive".to_string(),
        "GD-Nesterov".to_string(),
        "Adam".to_string(),
    ]);
    
    champions.insert("Matyas_2D".to_string(), vec![
        "QQN-StrongWolfe".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-Nesterov".to_string(),
        "Adam".to_string(),
    ]);
    
    // ML problem champions
    champions.insert("LogisticRegression_100samples_5features_reg0.01".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-Momentum".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("LogisticRegression_200samples_10features_reg0.01".to_string(), vec![
        "QQN-StrongWolfe".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region-Conservative".to_string(),
        "GD-Momentum".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("LinearRegression_100samples_5features_reg0.01".to_string(), vec![
        "QQN-MoreThuente".to_string(),
        "L-BFGS-Aggressive".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("LinearRegression_200samples_10features_reg0.01".to_string(), vec![
        "QQN-GoldenSection".to_string(),
        "L-BFGS-Aggressive".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("NeuralNetwork_100samples_layers_5_10_3".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS".to_string(),
        "Trust Region".to_string(),
        "GD-Momentum".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("NeuralNetwork_100samples_layers_10_20_5".to_string(), vec![
        "QQN-Bisection-2".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region".to_string(),
        "GD-Momentum".to_string(),
        "Adam-Fast".to_string(),
    ]);
    
    champions.insert("SVM_100samples_5features_C1".to_string(), vec![
        "QQN-Backtracking".to_string(),
        "L-BFGS-Conservative".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam-Fast-Fast".to_string(),
    ]);
    
    champions.insert("SVM_200samples_10features_C1".to_string(), vec![
        "QQN-GoldenSection".to_string(),
        "L-BFGS-Aggressive".to_string(),
        "Trust Region".to_string(),
        "GD-WeightDecay".to_string(),
        "Adam".to_string(),
    ]);
    
    if enable_mnist {
        // MNIST problem champions
        champions.insert("MNIST_10000samples".to_string(), vec![
            "QQN-Backtracking".to_string(),
            "L-BFGS-Conservative".to_string(),
            "Trust Region".to_string(),
            "GD-Nesterov".to_string(),
            "Adam-Fast".to_string(),
        ]);
        
        champions.insert("MNIST_20000samples".to_string(), vec![
            "QQN-GoldenSection".to_string(),
            "L-BFGS-Aggressive".to_string(),
            "Trust Region-Conservative".to_string(),
            "GD-Momentum".to_string(),
            "Adam-AMSGrad".to_string(),
        ]);
    }
    
    champions
}


/// Create champion optimizers list from the championship configuration
fn create_champion_optimizers() -> Vec<(String, Arc<dyn qqn_optimizer::Optimizer>)> {
    // Get all available optimizers
    let mut all_optimizers = Vec::new();
    all_optimizers.extend(crate::experiment_runner::optimizer_sets::qqn_variants());
    all_optimizers.extend(crate::experiment_runner::optimizer_sets::qqn_line_search_optimizers());
    all_optimizers.extend(crate::experiment_runner::optimizer_sets::lbfgs_variants());
    all_optimizers.extend(crate::experiment_runner::optimizer_sets::gd_variants());
    all_optimizers.extend(crate::experiment_runner::optimizer_sets::adam_variants());
    all_optimizers.extend(crate::experiment_runner::optimizer_sets::trust_region_variants());
    
    // Create a map for quick lookup
    let optimizer_map: HashMap<String, Arc<dyn qqn_optimizer::Optimizer>> = all_optimizers
        .into_iter()
        .collect();
    
    // Get unique champion names from all problems
    let championship_config = get_championship_config();
    let mut unique_champions = std::collections::HashSet::new();
    
    for champions in championship_config.values() {
        for champion in champions {
            unique_champions.insert(champion.clone());
        }
    }
    
    // Build the champion optimizers list
    let mut champion_optimizers = Vec::new();
    for champion_name in unique_champions {
        if let Some(optimizer) = optimizer_map.get(&champion_name) {
            champion_optimizers.push((champion_name, optimizer.clone()));
        }
    }
    
    champion_optimizers
}

#[tokio::test]
async fn test_championship_benchmarks() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logging(false)?;
    
    // Enable no threshold mode for this test
    enable_no_threshold_mode();

    let mut problems = analytic_problems();
    problems.extend(ml_problems());
    problems.extend(mnist_problems(10000));
    
    let champion_optimizers = create_champion_optimizers();
    
    run_championship_benchmark(
        "results/championship_",
        1000,
        10,
        Duration::from_secs(120),
        problems,
        champion_optimizers,
    ).await?;

    tokio::task::yield_now().await;
    Ok(())
}

async fn run_championship_benchmark(
    prefix: &str,
    max_evals: usize,
    num_runs: usize,
    time_limit: Duration,
    problems: Vec<Arc<dyn OptimizationProblem>>,
    optimizers: Vec<(String, Arc<dyn qqn_optimizer::Optimizer>)>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    
    // Get championship configuration
    let championship_config = get_championship_config();
    
    // Run the championship benchmark with problem-specific optimizers
    crate::experiment_runner::experiment_runner::run_championship_benchmark(
        prefix,
        max_evals,
        num_runs,
        time_limit,
        problems,
        championship_config,
        optimizers,
    ).await?;
    
    // Then generate the championship report
    generate_championship_report(prefix).await?;
    
    Ok(())
}

async fn generate_championship_report(prefix: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    use std::fs;
    use std::io::Write;
    
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir = format!("{}{}", prefix, timestamp);
    
    let championship_config = get_championship_config();
    
    let mut report = String::new();
    report.push_str("# Championship Optimization Benchmark Report\n\n");
    report.push_str("This report presents the results of a championship competition between optimizer families.\n");
    report.push_str("Each optimizer family's champion variant was selected for each problem based on prior performance analysis.\n\n");
    
    report.push_str("## Championship Configuration\n\n");
    report.push_str("The following table shows which optimizer variant was selected as the champion for each problem:\n\n");
    report.push_str("| Problem | QQN Champion | L-BFGS Champion | Trust Region Champion | GD Champion | Adam Champion |\n");
    report.push_str("|---------|--------------|-----------------|----------------------|-------------|---------------|\n");
    
    for (problem_name, champions) in &championship_config {
        report.push_str(&format!("| {} |", problem_name));
        for champion in champions {
            report.push_str(&format!(" {} |", champion));
        }
        report.push('\n');
    }
    
    report.push_str("\n## Family Performance Matrix\n\n");
    report.push_str("This section will contain statistical comparisons between optimizer families,\n");
    report.push_str("using the champion variants for each problem to represent family performance.\n\n");
    
    report.push_str("### Win-Loss Matrix\n\n");
    report.push_str("| Family | QQN | L-BFGS | Trust Region | GD | Adam |\n");
    report.push_str("|--------|-----|--------|--------------|----|----- |\n");
    report.push_str("| QQN | - | TBD | TBD | TBD | TBD |\n");
    report.push_str("| L-BFGS | TBD | - | TBD | TBD | TBD |\n");
    report.push_str("| Trust Region | TBD | TBD | - | TBD | TBD |\n");
    report.push_str("| GD | TBD | TBD | TBD | - | TBD |\n");
    report.push_str("| Adam | TBD | TBD | TBD | TBD | - |\n\n");
    
    report.push_str("### Statistical Significance Tests\n\n");
    report.push_str("Statistical significance tests (Wilcoxon signed-rank test) will be performed\n");
    report.push_str("to determine if performance differences between families are statistically significant.\n\n");
    
    report.push_str("## Per-Problem Championship Results\n\n");
    for problem_name in championship_config.keys() {
        report.push_str(&format!("### {}\n\n", problem_name));
        report.push_str("Championship results for this problem will be populated after benchmark execution.\n\n");
    }
    
    report.push_str("## Methodology\n\n");
    report.push_str("1. **Champion Selection**: For each optimizer family, the best-performing variant was selected for each problem based on prior benchmarking results.\n\n");
    report.push_str("2. **Fair Competition**: Each family is represented by its strongest variant for each specific problem, ensuring a fair comparison of family capabilities.\n\n");
    report.push_str("3. **Statistical Analysis**: Family-level performance is aggregated across all problems, with statistical tests to determine significance of differences.\n\n");
    report.push_str("4. **Problem-Specific Analysis**: Individual problem results show which family's champion performed best on each specific optimization challenge.\n\n");
    
    // Write the championship report
    let report_path = format!("{}/championship_report.md", output_dir);
    let mut file = fs::File::create(&report_path)?;
    file.write_all(report.as_bytes())?;
    
    println!("Championship report generated at: {}", report_path);
    
    Ok(())
}