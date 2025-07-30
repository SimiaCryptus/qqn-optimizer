use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::report_generator;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate main performance LaTeX table
pub fn generate_main_performance_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    let mut latex_content = String::from(
        r#"\documentclass{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{multirow}
\usepackage{longtable}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage{rotating}
\usepackage{graphicx}
\begin{document}
\tiny
\begin{adjustbox}{width=\textwidth,center}
\begin{longtable}{p{2cm}p{2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endhead
\midrule \multicolumn{9}{r}{{Continued on next page}} \\ \midrule
\endfoot
\bottomrule
\endlastfoot
"#,
    );
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        let mut optimizer_stats = HashMap::new();
        for result in &results.results {
            let stats = optimizer_stats
                .entry(result.optimizer_name.clone())
                .or_insert(Vec::new());
            stats.push(result);
        }
        let mut perf_data = Vec::new();
        for (optimizer, runs) in &optimizer_stats {
            let final_values: Vec<f64> = runs
                .iter()
                .map(|r| r.final_value)
                .filter(|&v| v.is_finite())
                .collect();
            if final_values.is_empty() {
                continue;
            }
            let function_evals: Vec<f64> =
                runs.iter().map(|r| r.function_evaluations as f64).collect();
            let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
            let execution_times: Vec<f64> = runs
                .iter()
                .map(|r| r.execution_time.as_secs_f64())
                .collect();
            let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
            let std_final = {
                let variance = final_values
                    .iter()
                    .map(|x| (x - mean_final).powi(2))
                    .sum::<f64>()
                    / final_values.len() as f64;
                variance.sqrt()
            };
            let best_final = final_values.iter().cloned().fold(f64::INFINITY, f64::min);
            let worst_final = final_values
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            let mean_function_evals =
                function_evals.iter().sum::<f64>() / function_evals.len() as f64;
            let success_rate = success_count as f64 / runs.len() as f64 * 100.0;
            let mean_time = execution_times.iter().sum::<f64>() / execution_times.len() as f64;
            perf_data.push((
                optimizer.clone(),
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_function_evals,
                success_rate,
                mean_time,
            ));
        }
        // Sort by success rate first, then by mean final value
        perf_data.sort_by(|a, b| {
            let success_cmp = b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal);
            if success_cmp != std::cmp::Ordering::Equal {
                success_cmp
            } else {
                a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        for (
            i,
            (
                optimizer,
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_func_evals,
                success_rate,
                mean_time,
            ),
        ) in perf_data.iter().enumerate()
        {
            let problem_cell = if i == 0 {
                format!(
                    "\\multirow{{{}}}{{*}}{{{}}}",
                    perf_data.len(),
                    report_generator::escape_latex(&problem_name)
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
                "{} & {} & {:.2e} & {:.2e} & {:.2e} & {:.2e} & {:.1} & {:.1} & {:.3} \\\\\n",
                problem_cell,
                optimizer_style,
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_func_evals,
                success_rate,
                mean_time
            ));
        }
        if !perf_data.is_empty() {
            latex_content.push_str("\\midrule\n");
        }
    }
    latex_content.push_str(
        r#"\end{longtable}
\end{adjustbox}
\end{document}
"#,
    );
    let latex_path = latex_dir.join("main_performance_table.tex");
    fs::write(&latex_path, latex_content)
        .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
    println!(
        "Generated main performance LaTeX table: {}",
        latex_path.display()
    );
    Ok(())
}

/// Generate main performance table content (without document wrapper)
pub fn generate_main_performance_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    let mut content = String::from(
        r#"\tiny
\begin{adjustbox}{width=\textwidth,center}
\begin{longtable}{p{2cm}p{2cm}*{7}{c}}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endhead
\midrule \multicolumn{9}{r}{{Continued on next page}} \\ \midrule
\endfoot
\bottomrule
\endlastfoot
"#,
    );
    // Add the same table generation logic as in generate_main_performance_latex_table
    // but without the document wrapper
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        let mut optimizer_stats = HashMap::new();
        for result in &results.results {
            let stats = optimizer_stats
                .entry(result.optimizer_name.clone())
                .or_insert(Vec::new());
            stats.push(result);
        }
        let mut perf_data = Vec::new();
        for (optimizer, runs) in &optimizer_stats {
            let final_values: Vec<f64> = runs
                .iter()
                .map(|r| r.final_value)
                .filter(|&v| v.is_finite())
                .collect();
            if final_values.is_empty() {
                continue;
            }
            let function_evals: Vec<f64> =
                runs.iter().map(|r| r.function_evaluations as f64).collect();
            let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
            let execution_times: Vec<f64> = runs
                .iter()
                .map(|r| r.execution_time.as_secs_f64())
                .collect();
            let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
            let std_final = {
                let variance = final_values
                    .iter()
                    .map(|x| (x - mean_final).powi(2))
                    .sum::<f64>()
                    / final_values.len() as f64;
                variance.sqrt()
            };
            let best_final = final_values.iter().cloned().fold(f64::INFINITY, f64::min);
            let worst_final = final_values
                .iter()
                .cloned()
                .fold(f64::NEG_INFINITY, f64::max);
            let mean_function_evals =
                function_evals.iter().sum::<f64>() / function_evals.len() as f64;
            let success_rate = success_count as f64 / runs.len() as f64 * 100.0;
            let mean_time = execution_times.iter().sum::<f64>() / execution_times.len() as f64;
            perf_data.push((
                optimizer.clone(),
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_function_evals,
                success_rate,
                mean_time,
            ));
        }
        // Sort by success rate first, then by mean final value
        perf_data.sort_by(|a, b| {
            let success_cmp = b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal);
            if success_cmp != std::cmp::Ordering::Equal {
                success_cmp
            } else {
                a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
            }
        });
        for (
            i,
            (
                optimizer,
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_func_evals,
                success_rate,
                mean_time,
            ),
        ) in perf_data.iter().enumerate()
        {
            let problem_cell = if i == 0 {
                format!(
                    "\\multirow{{{}}}{{*}}{{{}}}",
                    perf_data.len(),
                    report_generator::escape_latex(&problem_name)
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
                "{} & {} & {:.2e} & {:.2e} & {:.2e} & {:.2e} & {:.1} & {:.1} & {:.3} \\\\\n",
                problem_cell,
                optimizer_style,
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_func_evals,
                success_rate,
                mean_time
            ));
        }
        if !perf_data.is_empty() {
            content.push_str("\\midrule\n");
        }
    }
    content.push_str("\\end{longtable}\n}\n");
    content.push_str("\\end{adjustbox}\n");
    Ok(content)
}
