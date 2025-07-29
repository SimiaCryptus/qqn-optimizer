use std::path::Path;
use std::fs;
use anyhow::Context;
use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner::report_generator;

/// Generate convergence speed table content (without document wrapper)
pub fn generate_convergence_speed_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    // Similar logic as generate_convergence_speed_latex_table but return just the table content
    let mut optimizer_speed_data = std::collections::HashMap::new();
    for (_, results) in all_results {
        for result in &results.results {
            if result.convergence_achieved && !result.trace.iterations.is_empty() {
                let speed_data = optimizer_speed_data
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                let initial_value = result
                    .trace
                    .iterations
                    .first()
                    .map(|iter| iter.function_value)
                    .unwrap_or(result.final_value);
                let final_value = result.final_value;
                let improvement_50 = initial_value - 0.5 * (initial_value - final_value);
                let improvement_90 = initial_value - 0.9 * (initial_value - final_value);
                let mut iter_to_50 = None;
                let mut iter_to_90 = None;
                for iter_data in &result.trace.iterations {
                    if iter_to_50.is_none() && iter_data.function_value <= improvement_50 {
                        iter_to_50 = Some(iter_data.iteration);
                    }
                    if iter_to_90.is_none() && iter_data.function_value <= improvement_90 {
                        iter_to_90 = Some(iter_data.iteration);
                    }
                }
                speed_data.push((
                    iter_to_50.unwrap_or(result.iterations),
                    iter_to_90.unwrap_or(result.iterations),
                    result.iterations,
                ));
            }
        }
    }
    if optimizer_speed_data.is_empty() {
        return Ok(String::new());
    }
    let mut content = String::from(
        r#"\begin{table}[H]
\centering
\caption{Convergence Speed Analysis: Mean Iterations to Reach Improvement Milestones}
\label{tab:convergence_speed}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{lrrr}
\toprule
\textbf{Optimizer} & \textbf{Mean Iterations} & \textbf{Mean Iterations} & \textbf{Final Convergence} \\
 & \textbf{to 50\% Improvement} & \textbf{to 90\% Improvement} & \textbf{Iteration} \\
\midrule
"#,
    );
    // Same calculation and sorting logic as the standalone table
    let mut optimizer_averages = Vec::new();
    for (optimizer, speed_data) in optimizer_speed_data {
        if !speed_data.is_empty() {
            let avg_50 = speed_data
                .iter()
                .map(|(iter_50, _, _)| *iter_50 as f64)
                .sum::<f64>()
                / speed_data.len() as f64;
            let avg_90 = speed_data
                .iter()
                .map(|(_, iter_90, _)| *iter_90 as f64)
                .sum::<f64>()
                / speed_data.len() as f64;
            let avg_final = speed_data
                .iter()
                .map(|(_, _, final_iter)| *final_iter as f64)
                .sum::<f64>()
                / speed_data.len() as f64;
            optimizer_averages.push((optimizer, avg_50, avg_90, avg_final));
        }
    }
    optimizer_averages.sort_by(|a, b| {
        let score_a = 0.3 * a.1 + 0.4 * a.2 + 0.3 * a.3;
        let score_b = 0.3 * b.1 + 0.4 * b.2 + 0.3 * b.3;
        score_a
            .partial_cmp(&score_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for (i, (optimizer, avg_50, avg_90, avg_final)) in optimizer_averages.iter().enumerate() {
        let optimizer_style = if i == 0 {
            format!("\\textbf{{{}}}", report_generator::escape_latex(optimizer))
        } else if optimizer.contains("QQN") {
            format!(
                "\\textcolor{{green!70!black}}{{{}}}",
                report_generator::escape_latex(optimizer)
            )
        } else {
            report_generator::escape_latex(optimizer)
        };
        content.push_str(&format!(
            "{} & {:.1} & {:.1} & {:.1} \\\\\n",
            optimizer_style, avg_50, avg_90, avg_final
        ));
    }
    content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Compares convergence rates for different optimizers. Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.
"#,
    );
    Ok(content)
}

pub fn generate_convergence_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let successful_runs: Vec<_> = runs.iter().filter(|r| r.convergence_achieved).collect();
    let failed_runs: Vec<_> = runs.iter().filter(|r| !r.convergence_achieved).collect();
    let mut content = String::from("## Convergence Analysis\n\n");
    if !successful_runs.is_empty() {
        let iterations: Vec<usize> = successful_runs.iter().map(|r| r.iterations).collect();
        let function_evals: Vec<usize> = successful_runs
            .iter()
            .map(|r| r.function_evaluations)
            .collect();
        let times: Vec<f64> = successful_runs
            .iter()
            .map(|r| r.execution_time.as_secs_f64())
            .collect();
        let mean_iterations = iterations.iter().sum::<usize>() as f64 / iterations.len() as f64;
        let mean_func_evals =
            function_evals.iter().sum::<usize>() as f64 / function_evals.len() as f64;
        let mean_time = times.iter().sum::<f64>() / times.len() as f64;
        // Find min and max times with their corresponding iterations
        let (min_iter, min_time) = successful_runs
            .iter()
            .map(|r| (r.iterations, r.execution_time.as_secs_f64()))
            .min_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0, 0.0));
        let (max_iter, max_time) = successful_runs
            .iter()
            .map(|r| (r.iterations, r.execution_time.as_secs_f64()))
            .max_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or((0, 0.0));

        content.push_str(&format!(
            r#"### Successful Runs ({} out of {})

* **Average Iterations to Convergence:** {:.1}
* **Average Function Evaluations:** {:.1}
* **Average Time to Convergence:** {:.3}s
* **Fastest Convergence:** {} iterations ({:.3}s)
* **Slowest Convergence:** {} iterations ({:.3}s)

"#,
            successful_runs.len(),
            runs.len(),
            mean_iterations,
            mean_func_evals,
            mean_time,
            min_iter,
            min_time,
            max_iter,
            max_time
        ));
    }
    if !failed_runs.is_empty() {
        let mut failure_reasons = std::collections::HashMap::new();
        for run in &failed_runs {
            *failure_reasons
                .entry(format!("{:?}", run.convergence_reason))
                .or_insert(0) += 1;
        }
        content.push_str(&format!(
            "### Failed Runs ({} out of {})\n\n**Failure Reasons:**\n",
            failed_runs.len(),
            runs.len()
        ));
        for (reason, count) in failure_reasons {
            content.push_str(&format!("- {}: {} runs\n", reason, count));
        }
        content.push_str("\n");
    }
    Ok(content)
}

/// Generate convergence speed analysis LaTeX table
pub fn generate_convergence_speed_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    // Collect convergence speed data for all optimizers
    let mut optimizer_speed_data = std::collections::HashMap::new();
    for (_, results) in all_results {
        for result in &results.results {
            if result.convergence_achieved && !result.trace.iterations.is_empty() {
                let speed_data = optimizer_speed_data
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                // Calculate convergence milestones
                let initial_value = result
                    .trace
                    .iterations
                    .first()
                    .map(|iter| iter.function_value)
                    .unwrap_or(result.final_value);
                let final_value = result.final_value;
                let improvement_50 = initial_value - 0.5 * (initial_value - final_value);
                let improvement_90 = initial_value - 0.9 * (initial_value - final_value);
                let mut iter_to_50 = None;
                let mut iter_to_90 = None;
                for iter_data in &result.trace.iterations {
                    if iter_to_50.is_none() && iter_data.function_value <= improvement_50 {
                        iter_to_50 = Some(iter_data.iteration);
                    }
                    if iter_to_90.is_none() && iter_data.function_value <= improvement_90 {
                        iter_to_90 = Some(iter_data.iteration);
                    }
                }
                speed_data.push((
                    iter_to_50.unwrap_or(result.iterations),
                    iter_to_90.unwrap_or(result.iterations),
                    result.iterations,
                ));
            }
        }
    }
    if optimizer_speed_data.is_empty() {
        return Ok(());
    }
    let mut latex_content = String::from(
        r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage[table]{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage[margin=1in]{geometry}
\begin{document}
\begin{table}[htbp]
\centering
\caption{Convergence Speed Analysis: Mean Iterations to Reach Improvement Milestones}
\label{tab:convergence_speed}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{p{3cm}p{2cm}p{2cm}p{2cm}}
\toprule
\textbf{Optimizer} & \textbf{Mean Iterations} & \textbf{Mean Iterations} & \textbf{Final Convergence} \\
 & \textbf{to 50\% Improvement} & \textbf{to 90\% Improvement} & \textbf{Iteration} \\
\midrule
"#,
    );
    // Calculate averages and sort by overall convergence speed
    let mut optimizer_averages = Vec::new();
    for (optimizer, speed_data) in optimizer_speed_data {
        if !speed_data.is_empty() {
            let avg_50 = speed_data
                .iter()
                .map(|(iter_50, _, _)| *iter_50 as f64)
                .sum::<f64>()
                / speed_data.len() as f64;
            let avg_90 = speed_data
                .iter()
                .map(|(_, iter_90, _)| *iter_90 as f64)
                .sum::<f64>()
                / speed_data.len() as f64;
            let avg_final = speed_data
                .iter()
                .map(|(_, _, final_iter)| *final_iter as f64)
                .sum::<f64>()
                / speed_data.len() as f64;
            optimizer_averages.push((optimizer, avg_50, avg_90, avg_final));
        }
    }
    // Sort by fastest overall convergence (weighted average of milestones)
    optimizer_averages.sort_by(|a, b| {
        let score_a = 0.3 * a.1 + 0.4 * a.2 + 0.3 * a.3;
        let score_b = 0.3 * b.1 + 0.4 * b.2 + 0.3 * b.3;
        score_a
            .partial_cmp(&score_b)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for (i, (optimizer, avg_50, avg_90, avg_final)) in optimizer_averages.iter().enumerate() {
        let optimizer_style = if i == 0 {
            format!("\\textbf{{{}}}", report_generator::escape_latex(optimizer))
        } else if optimizer.contains("QQN") {
            format!(
                "\\textcolor{{green!70!black}}{{{}}}",
                report_generator::escape_latex(optimizer)
            )
        } else {
            report_generator::escape_latex(optimizer)
        };
        latex_content.push_str(&format!(
            "{} & {:.1} & {:.1} & {:.1} \\\\\n",
            optimizer_style, avg_50, avg_90, avg_final
        ));
    }
    latex_content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Compares convergence rates for different optimizers. Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.
\end{document}
"#,
    );
    let latex_path = latex_dir.join("convergence_speed_table.tex");
    fs::write(&latex_path, latex_content).with_context(|| {
        format!(
            "Failed to write convergence speed LaTeX table to: {}",
            latex_path.display()
        )
    })?;
    println!(
        "Generated convergence speed analysis LaTeX table: {}",
        latex_path.display()
    );
    Ok(())
}