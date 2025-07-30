use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::{report_generator, Report, ReportConfig, ReportFormat, ReportMetadata};
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
/// Performance table report showing detailed metrics for each optimizer-problem combination
pub struct PerformanceTableReport;
impl PerformanceTableReport {
    pub fn new() -> Self {
        Self
    }
    fn calculate_performance_data(&self, all_results: &[(&ProblemSpec, BenchmarkResults)]) -> Vec<(String, Vec<(String, f64, f64, f64, f64, f64, f64, f64)>)> {
        let mut problem_data = Vec::new();
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
                let function_evals: Vec<f64> = runs.iter().map(|r| r.function_evaluations as f64).collect();
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let execution_times: Vec<f64> = runs.iter().map(|r| r.execution_time.as_secs_f64()).collect();
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
                let worst_final = final_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
                let mean_function_evals = function_evals.iter().sum::<f64>() / function_evals.len() as f64;
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
            problem_data.push((problem_name, perf_data));
        }
        problem_data
    }
    fn generate_html(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> anyhow::Result<String> {
        let performance_data = self.calculate_performance_data(data);
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Performance Table Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; font-weight: bold; }
        .problem-header { background-color: #e6f3ff; font-weight: bold; }
        .best-optimizer { background-color: #d4edda; }
        .numeric { text-align: right; }
    </style>
</head>
<body>
    <h1>Performance Table Report</h1>
    <p>Detailed performance metrics for each optimizer-problem combination.</p>
    <table>
        <thead>
            <tr>
                <th>Problem</th>
                <th>Optimizer</th>
                <th>Mean Final Value</th>
                <th>Std Dev</th>
                <th>Best Value</th>
                <th>Worst Value</th>
                <th>Mean Func Evals</th>
                <th>Success Rate (%)</th>
                <th>Mean Time (s)</th>
            </tr>
        </thead>
        <tbody>
"#,
        );
        for (problem_name, perf_data) in performance_data {
            for (i, (optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time)) in perf_data.iter().enumerate() {
                let problem_cell = if i == 0 {
                    format!("<td rowspan=\"{}\" class=\"problem-header\">{}</td>", perf_data.len(), problem_name)
                } else {
                    String::new()
                };
                let row_class = if i == 0 { "best-optimizer" } else { "" };
                html.push_str(&format!(
                    "<tr class=\"{}\">{}
                        <td>{}</td>
                        <td class=\"numeric\">{:.2e}</td>
                        <td class=\"numeric\">{:.2e}</td>
                        <td class=\"numeric\">{:.2e}</td>
                        <td class=\"numeric\">{:.2e}</td>
                        <td class=\"numeric\">{:.1}</td>
                        <td class=\"numeric\">{:.1}</td>
                        <td class=\"numeric\">{:.3}</td>
                    </tr>\n",
                    row_class, problem_cell, optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time
                ));
            }
        }
        html.push_str("</tbody></table></body></html>");
        Ok(html)
    }
    fn generate_latex(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> anyhow::Result<String> {
        let performance_data = self.calculate_performance_data(data);
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
        for (problem_name, perf_data) in performance_data {
            for (i, (optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time)) in perf_data.iter().enumerate() {
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
                    "{problem_cell} & {optimizer_style} & {mean_final:.2e} & {std_final:.2e} & {best_final:.2e} & {worst_final:.2e} & {mean_func_evals:.1} & {success_rate:.1} & {mean_time:.3} \\\\\n"
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
        Ok(latex_content)
    }
    fn generate_markdown(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> anyhow::Result<String> {
        let performance_data = self.calculate_performance_data(data);
        let mut markdown = String::from("# Performance Table Report\n\n");
        markdown.push_str("Detailed performance metrics for each optimizer-problem combination.\n\n");
        for (problem_name, perf_data) in performance_data {
            markdown.push_str(&format!("## {}\n\n", problem_name));
            markdown.push_str("| Optimizer | Mean Final Value | Std Dev | Best Value | Worst Value | Mean Func Evals | Success Rate (%) | Mean Time (s) |\n");
            markdown.push_str("|-----------|------------------|---------|------------|-------------|-----------------|------------------|---------------|\n");
            for (optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time) in perf_data {
                markdown.push_str(&format!(
                    "| {} | {:.2e} | {:.2e} | {:.2e} | {:.2e} | {:.1} | {:.1} | {:.3} |\n",
                    optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time
                ));
            }
            markdown.push_str("\n");
        }
        Ok(markdown)
    }
    fn generate_csv(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> anyhow::Result<String> {
        let performance_data = self.calculate_performance_data(data);
        let mut csv = String::from("Problem,Optimizer,Mean Final Value,Std Dev,Best Value,Worst Value,Mean Func Evals,Success Rate (%),Mean Time (s)\n");
        for (problem_name, perf_data) in performance_data {
            for (optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time) in perf_data {
                csv.push_str(&format!(
                    "{},{},{:.2e},{:.2e},{:.2e},{:.2e},{:.1},{:.1},{:.3}\n",
                    problem_name, optimizer, mean_final, std_final, best_final, worst_final, mean_func_evals, success_rate, mean_time
                ));
            }
        }
        Ok(csv)
    }
}
impl Report for PerformanceTableReport {
    fn name(&self) -> &'static str {
        "performance_table"
    }
    fn description(&self) -> &'static str {
        "Shows detailed performance metrics for each optimizer-problem combination"
    }
    fn generate_content(&self, data: &[(&ProblemSpec, BenchmarkResults)], config: &ReportConfig) -> anyhow::Result<String> {
        match config.format {
            ReportFormat::Html => self.generate_html(data, config),
            ReportFormat::Latex => self.generate_latex(data, config),
            ReportFormat::Markdown => self.generate_markdown(data, config),
            ReportFormat::Csv => self.generate_csv(data, config),
        }
    }
    fn export_to_file(&self, data: &[(&ProblemSpec, BenchmarkResults)], config: &ReportConfig, output_path: &Path) -> anyhow::Result<()> {
        let content = self.generate_content(data, config)?;
        fs::write(output_path, content)
            .with_context(|| format!("Failed to write performance table report to: {}", output_path.display()))?;
        Ok(())
    }
    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> anyhow::Result<()> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("No benchmark data provided"));
        }
        for (problem, results) in data {
            if results.results.is_empty() {
                return Err(anyhow::anyhow!("No results for problem: {}", problem.get_name()));
            }
        }
        Ok(())
    }
    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let total_problems = data.len();
        let total_optimizers: std::collections::HashSet<String> = data
            .iter()
            .flat_map(|(_, results)| results.results.iter().map(|r| r.optimizer_name.clone()))
            .collect();
        let total_runs: usize = data.iter().map(|(_, results)| results.results.len()).sum();
        ReportMetadata {

            // report_name: self.name().to_string(),
            // generation_time: std::time::SystemTime::now(),
            // total_problems,
            // total_optimizers: total_optimizers.len(),
            // total_runs,
            // additional_info: std::collections::HashMap::new(),
            report_type: "".to_string(),
            generated_at: Default::default(),
            problem_count: 0,
            optimizer_count: 0,
            data_points: 0,
        }
    }
    fn supported_formats(&self) -> Vec<ReportFormat> {
        vec![ReportFormat::Html, ReportFormat::Latex, ReportFormat::Markdown, ReportFormat::Csv]
    }
}
// Legacy functions for backward compatibility

/// Generate main performance LaTeX table
pub fn generate_main_performance_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    let report = PerformanceTableReport::new();
    let config = ReportConfig { format: ReportFormat::Latex, ..Default::default() };
    let content = report.generate_content(all_results, &config)?;
    
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
                "{problem_cell} & {optimizer_style} & {mean_final:.2e} & {std_final:.2e} & {best_final:.2e} & {worst_final:.2e} & {mean_func_evals:.1} & {success_rate:.1} & {mean_time:.3} \\\\\n"
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
    let report = PerformanceTableReport::new();
    let config = ReportConfig { format: ReportFormat::Latex, ..Default::default() };
    let latex_content = report.generate_latex(all_results, &config)?;
    
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
                "{problem_cell} & {optimizer_style} & {mean_final:.2e} & {std_final:.2e} & {best_final:.2e} & {worst_final:.2e} & {mean_func_evals:.1} & {success_rate:.1} & {mean_time:.3} \\\\\n"
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::UnifiedReportTestSuite;
    #[test]
    fn test_performance_table_report() {
        let report = PerformanceTableReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }
    #[test]
    fn test_performance_table_basic_functionality() {
        let report = PerformanceTableReport::new();
        assert_eq!(report.name(), "performance_table");
        assert_eq!(report.description(), "Shows detailed performance metrics for each optimizer-problem combination");
        assert_eq!(report.supported_formats(), vec![ReportFormat::Html, ReportFormat::Latex, ReportFormat::Markdown, ReportFormat::Csv]);
    }
}