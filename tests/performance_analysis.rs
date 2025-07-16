use qqn_optimizer::analysis::statistics::StatisticalAnalysis;
use qqn_optimizer::benchmarks::evaluation::{
    BenchmarkConfig, OptimizationTrace, PerformanceMetrics,
};
use qqn_optimizer::benchmarks::evaluation::{BenchmarkResults, ConvergenceReason, SingleResult};
use std::fs;
use std::time::Duration;

/// Generate LaTeX tables for academic papers
pub struct LaTeXTableGenerator;

impl LaTeXTableGenerator {
    pub fn generate_performance_table(results: &BenchmarkResults) -> String {
        let mut latex = String::from(
            r#"\begin{table}[htbp]
\centering
\caption{Performance comparison of optimization algorithms}
\label{tab:performance}
\begin{tabular}{lcccc}
\toprule
Algorithm & Mean Final Value & Std Dev & Mean Iterations & Success Rate \\
\midrule
"#,
        );

        // Group results by optimizer
        let mut optimizer_stats = std::collections::HashMap::new();
        for result in &results.results {
            let stats = optimizer_stats
                .entry(result.optimizer_name.clone())
                .or_insert(Vec::new());
            stats.push(result);
        }

        for (optimizer, runs) in optimizer_stats {
            let final_values: Vec<f64> = runs.iter().map(|r| r.final_value).collect();
            let iterations: Vec<f64> = runs.iter().map(|r| r.iterations as f64).collect();
            let success_count = runs.iter().filter(|r| r.convergence_achieved).count();

            let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
            let std_final = {
                let variance = final_values
                    .iter()
                    .map(|x| (x - mean_final).powi(2))
                    .sum::<f64>()
                    / final_values.len() as f64;
                variance.sqrt()
            };
            let mean_iterations = iterations.iter().sum::<f64>() / iterations.len() as f64;
            let success_rate = success_count as f64 / runs.len() as f64;

            latex.push_str(&format!(
                "{} & {:.2e} & {:.2e} & {:.1} & {:.1}\\% \\\\\n",
                optimizer,
                mean_final,
                std_final,
                mean_iterations,
                success_rate * 100.0
            ));
        }

        latex.push_str(
            r#"\bottomrule
\end{tabular}
\end{table}
"#,
        );

        latex
    }

    pub fn generate_significance_table(analysis: &StatisticalAnalysis) -> String {
        let mut latex = String::from(
            r#"\begin{table}[htbp]
\centering
\caption{Statistical significance tests comparing optimization algorithms}
\label{tab:significance}
\begin{tabular}{lccc}
\toprule
Comparison & Test Statistic & p-value & Significant \\
\midrule
"#,
        );

        for test in analysis.significance_tests() {
            let significant = if test.is_significant() { "Yes" } else { "No" };
            latex.push_str(&format!(
                "{} vs {} & {:.4} & {:.4} & {} \\\\\n",
                test.optimizer_a, test.optimizer_b, test.statistic, test.p_value, significant
            ));
        }

        latex.push_str(
            r#"\bottomrule
\end{tabular}
\end{table}
"#,
        );

        latex
    }
}

#[test]
fn test_latex_table_generation() {
    println!("Starting LaTeX table generation test...");

    // Create mock results
    let config = BenchmarkConfig::default();
    let mut results = BenchmarkResults::new(config);

    // Add some mock results
    for i in 0..10 {
        println!("Adding mock result {} for QQN", i);
        results.add_result(SingleResult {
            problem_name: "test_problem".to_string(),
            optimizer_name: "QQN".to_string(),
            run_id: i,
            final_value: 1e-6 * (i as f64 + 1.0),
            final_gradient_norm: 1e-8,
            iterations: 100 + i * 10,
            function_evaluations: 200 + i * 20,
            gradient_evaluations: 100 + i * 10,
            convergence_achieved: true,
            execution_time: Duration::from_millis(1000 + i as u64 * 100),
            trace: OptimizationTrace::new(),
            convergence_reason: ConvergenceReason::GradientTolerance,
            memory_usage: None,
            performance_metrics: PerformanceMetrics {
                iterations_per_second: 100.0,
                function_evaluations_per_second: 200.0,
                gradient_evaluations_per_second: 100.0,
                convergence_rate: -0.1,
            },
        });
        println!("Adding mock result {} for L-BFGS", i);

        results.add_result(SingleResult {
            problem_name: "test_problem".to_string(),
            optimizer_name: "L-BFGS".to_string(),
            run_id: i,
            final_value: 2e-6 * (i as f64 + 1.0),
            final_gradient_norm: 2e-8,
            iterations: 120 + i * 12,
            function_evaluations: 240 + i * 24,
            gradient_evaluations: 120 + i * 12,
            convergence_achieved: i < 8, // Some failures
            execution_time: Duration::from_millis(1200 + i as u64 * 120),
            trace: OptimizationTrace::new(),
            convergence_reason: if i < 8 {
                ConvergenceReason::GradientTolerance
            } else {
                ConvergenceReason::MaxIterations
            },
            memory_usage: None,
            performance_metrics: PerformanceMetrics {
                iterations_per_second: 0.0,
                function_evaluations_per_second: 0.0,
                gradient_evaluations_per_second: 0.0,
                convergence_rate: 0.0,
            },
        });
    }
    println!("Generating LaTeX performance table...");

    let latex_table = LaTeXTableGenerator::generate_performance_table(&results);

    // Verify LaTeX structure
    assert!(latex_table.contains("\\begin{table}"));
    assert!(latex_table.contains("\\end{table}"));
    assert!(latex_table.contains("QQN"));
    assert!(latex_table.contains("L-BFGS"));
    assert!(latex_table.contains("Mean Final Value"));

    // Generate statistical analysis
    println!("Generating statistical analysis...");
    let analysis = StatisticalAnalysis::new(&results);
    let significance_table = LaTeXTableGenerator::generate_significance_table(&analysis);

    assert!(significance_table.contains("Statistical significance"));
    assert!(significance_table.contains("p-value"));

    println!("Generated LaTeX table:\n{}", latex_table);
    println!("Generated significance table:\n{}", significance_table);
    // Save to files for verification
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir_name = format!("results/latex/output_{}", timestamp);
    let output_dir = std::path::Path::new(&output_dir_name);
    std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
    std::fs::write(output_dir.join("performance_table.tex"), &latex_table)
        .expect("Failed to write performance table");
    std::fs::write(
        output_dir.join("significance_table.tex"),
        &significance_table,
    )
        .expect("Failed to write significance table");
    println!("LaTeX tables saved to: {}", output_dir.display());
}

#[test]
fn test_export_academic_formats() -> std::io::Result<()> {
    // Use a timestamped directory to avoid conflicts
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let output_dir = format!("results/academic/exports_{}", timestamp);
    let output_path = std::path::Path::new(&output_dir);
    std::fs::create_dir_all(output_path)?;

    println!("Exporting academic formats to: {}", output_path.display());

    // Create mock results (same as above)
    let config = BenchmarkConfig::default();
    let mut results = BenchmarkResults::new(config);

    for i in 0..5 {
        results.add_result(SingleResult {
            problem_name: "rosenbrock_2d".to_string(),
            optimizer_name: "QQN".to_string(),
            run_id: i,
            final_value: 1e-8 * (i as f64 + 1.0),
            final_gradient_norm: 1e-10,
            iterations: 50 + i * 5,
            function_evaluations: 100 + i * 10,
            gradient_evaluations: 50 + i * 5,
            convergence_achieved: true,
            execution_time: Duration::from_millis(500 + i as u64 * 50),
            trace: OptimizationTrace::new(),
            convergence_reason: ConvergenceReason::GradientTolerance,
            memory_usage: None,
            performance_metrics: PerformanceMetrics {
                iterations_per_second: 0.0,
                function_evaluations_per_second: 0.0,
                gradient_evaluations_per_second: 0.0,
                convergence_rate: 0.0,
            },
        });
    }

    // Export LaTeX tables
    let performance_table = LaTeXTableGenerator::generate_performance_table(&results);
    println!(
        "Writing performance table to: {}",
        output_path.join("performance_table.tex").display()
    );
    fs::write(output_path.join("performance_table.tex"), performance_table)?;

    let analysis = StatisticalAnalysis::new(&results);
    let significance_table = LaTeXTableGenerator::generate_significance_table(&analysis);
    println!(
        "Writing significance table to: {}",
        output_path.join("significance_table.tex").display()
    );
    fs::write(
        output_path.join("significance_table.tex"),
        significance_table,
    )?;

    // Export raw data for external analysis
    let csv_data = results
        .results
        .iter()
        .map(|r| {
            format!(
                "{},{},{},{:.6e},{},{:.3}",
                r.problem_name,
                r.optimizer_name,
                r.run_id,
                r.final_value,
                r.iterations,
                r.execution_time.as_secs_f64()
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let csv_header = "Problem,Optimizer,Run,FinalValue,Iterations,Time\n";
    fs::write(
        output_path.join("raw_results.csv"),
        format!("{}{}", csv_header, csv_data),
    )?;

    // Verify files were created
    assert!(output_path.join("performance_table.tex").exists());
    assert!(output_path.join("significance_table.tex").exists());
    assert!(output_path.join("raw_results.csv").exists());

    println!(
        "Academic format exports saved to: {}",
        output_path.display()
    );
    Ok(())
}
