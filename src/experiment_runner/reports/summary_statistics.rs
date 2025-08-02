use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner::report_generator;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate summary statistics LaTeX table
pub fn generate_summary_statistics_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    let mut latex_content = String::from(
        r#"\documentclass{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{siunitx}
\usepackage{multirow}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{adjustbox}
\usepackage{graphicx}
\begin{document}
\small
\begin{table}[htbp]
\centering
\caption{Summary Statistics by Problem Family}
\label{tab:summary_statistics}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{p{2.5cm}p{2.5cm}*{5}{c}}
\toprule
\textbf{Problem Family} & \textbf{Optimizer} & \textbf{Avg Success} & \textbf{Avg Final} & \textbf{Avg Func} & \textbf{Avg Grad} & \textbf{Avg Time} \\
 & & \textbf{Rate (\%)} & \textbf{Value} & \textbf{Evals} & \textbf{Evals} & \textbf{(s)} \\
\midrule
"#,
    );
    // Group by problem family
    let mut family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> = HashMap::new();
    for (problem, results) in all_results {
        let family = report_generator::get_family(&problem.get_name());
        for result in &results.results {
            family_results
                .entry(family.clone())
                .or_default()
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(result);
        }
    }
    let mut families: Vec<_> = family_results.keys().cloned().collect();
    families.sort();
    for family in families {
        if let Some(optimizers) = family_results.get(&family) {
            let mut optimizer_data = Vec::new();
            for (optimizer, runs) in optimizers {
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let success_rate = success_count as f64 / runs.len() as f64 * 100.0;
                let final_values: Vec<f64> = runs
                    .iter()
                    .map(|r| r.final_value)
                    .filter(|&v| v.is_finite())
                    .collect();
                let avg_final = if !final_values.is_empty() {
                    final_values.iter().sum::<f64>() / final_values.len() as f64
                } else {
                    f64::INFINITY
                };
                let avg_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;
                let avg_grad_evals = runs
                    .iter()
                    .map(|r| r.gradient_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;
                let avg_time = runs
                    .iter()
                    .map(|r| r.execution_time.as_secs_f64())
                    .sum::<f64>()
                    / runs.len() as f64;
                optimizer_data.push((
                    optimizer.clone(),
                    success_rate,
                    avg_final,
                    avg_func_evals,
                    avg_grad_evals,
                    avg_time,
                ));
            }
            // Sort by success rate
            optimizer_data.sort_by(|a, b| b.1.total_cmp(&a.1));
            
            for (
                i,
                (optimizer, success_rate, avg_final, avg_func_evals, avg_grad_evals, avg_time),
            ) in optimizer_data.iter().enumerate()
            {
                let family_cell = if i == 0 {
                    format!(
                        "\\multirow{{{}}}{{*}}{{{}}}",
                        optimizer_data.len(),
                        report_generator::escape_latex(&family)
                    )
                } else {
                    String::new()
                };
                let optimizer_style = if i == 0 {
                    format!("\\textbf{{{}}}", report_generator::escape_latex(optimizer))
                } else {
                    report_generator::escape_latex(optimizer)
                };
                latex_content.push_str(&format!(
                    "{family_cell} & {optimizer_style} & {success_rate:.1} & {avg_final:.2e} & {avg_func_evals:.1} & {avg_grad_evals:.1} & {avg_time:.3} \\\\\n"
                ));
            }
            if !optimizer_data.is_empty() {
                latex_content.push_str("\\midrule\n");
            }
        }
    }
    latex_content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\end{document}
"#,
    );
    let latex_path = latex_dir.join("summary_statistics.tex");
    // Ensure parent directory exists
    if let Some(parent) = latex_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    fs::write(&latex_path, latex_content)
        .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
    println!(
        "Generated summary statistics LaTeX table: {}",
        latex_path.display()
    );
    Ok(())
}

/// Generate summary statistics table content (without document wrapper)
pub fn generate_summary_statistics_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    // Similar implementation as generate_summary_statistics_latex_table but return just the table content
    let mut content = String::from(
        r#"\begin{table}[H]
\centering
\caption{Summary Statistics by Problem Family}
\label{tab:summary_statistics}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{p{{2.5cm}}p{{2.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}}
\toprule
\textbf{Problem Family} & \textbf{Optimizer} & \textbf{Avg Success} & \textbf{Avg Final} & \textbf{Avg Func} & \textbf{Avg Grad} & \textbf{Avg Time} \\
 & & \textbf{Rate (\%)} & \textbf{Value} & \textbf{Evals} & \textbf{Evals} & \textbf{(s)} \\
\midrule
"#,
    );
    // Group by problem family (same logic as before)
    let mut family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> = HashMap::new();
    for (problem, results) in all_results {
        let family = report_generator::get_family(&problem.get_name());
        for result in &results.results {
            family_results
                .entry(family.clone())
                .or_default()
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(result);
        }
    }
    let mut families: Vec<_> = family_results.keys().cloned().collect();
    families.sort();
    for family in families {
        if let Some(optimizers) = family_results.get(&family) {
            let mut optimizer_data = Vec::new();
            for (optimizer, runs) in optimizers {
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let success_rate = success_count as f64 / runs.len() as f64 * 100.0;
                let final_values: Vec<f64> = runs
                    .iter()
                    .map(|r| r.final_value)
                    .filter(|&v| v.is_finite())
                    .collect();
                let avg_final = if !final_values.is_empty() {
                    final_values.iter().sum::<f64>() / final_values.len() as f64
                } else {
                    f64::INFINITY
                };
                let avg_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;
                let avg_grad_evals = runs
                    .iter()
                    .map(|r| r.gradient_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;
                let avg_time = runs
                    .iter()
                    .map(|r| r.execution_time.as_secs_f64())
                    .sum::<f64>()
                    / runs.len() as f64;
                optimizer_data.push((
                    optimizer.clone(),
                    success_rate,
                    avg_final,
                    avg_func_evals,
                    avg_grad_evals,
                    avg_time,
                ));
            }
            
            optimizer_data
                .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            
            for (
                i,
                (optimizer, success_rate, avg_final, avg_func_evals, avg_grad_evals, avg_time),
            ) in optimizer_data.iter().enumerate()
            {
                let family_cell = if i == 0 {
                    format!(
                        "\\multirow{{{}}}{{*}}{{{}}}",
                        optimizer_data.len(),
                        report_generator::escape_latex(&family)
                    )
                } else {
                    String::new()
                };
                let optimizer_style = if i == 0 {
                    format!("\\textbf{{{}}}", report_generator::escape_latex(optimizer))
                } else {
                    report_generator::escape_latex(optimizer)
                };
                content.push_str(&format!(
                    "{family_cell} & {optimizer_style} & {success_rate:.1} & {avg_final:.2e} & {avg_func_evals:.1} & {avg_grad_evals:.1} & {avg_time:.3} \\\\\n"
                ));
            }
            if !optimizer_data.is_empty() {
                content.push_str("\\midrule\n");
            }
        }
    }
    content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
"#,
    );
    Ok(content)
}
