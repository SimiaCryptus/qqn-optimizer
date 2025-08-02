use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner::{
    report_generator, Report, ReportConfig, ReportFormat, ReportMetadata,
};
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
/// Convergence Analysis Report
pub struct ConvergenceAnalysisReport;
impl ConvergenceAnalysisReport {
    pub fn new() -> Self {
        Self
    }
    fn generate_html(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let mut content = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Convergence Analysis Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; }
        .best { font-weight: bold; }
        .qqn { color: #2e7d32; }
        .stats { background-color: #f9f9f9; padding: 15px; margin: 10px 0; }
    </style>
</head>
<body>
    <h1>Convergence Analysis Report</h1>
"#,
        );
        if config.include_detailed_stats {
            content.push_str(&self.generate_convergence_speed_html_table(data)?);
        }
        content.push_str("</body></html>");
        Ok(content)
    }
    fn generate_latex(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let mut content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage[table]{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage[margin=1in]{geometry}
\usepackage{float}
\begin{document}
\title{Convergence Analysis Report}
\maketitle
"#,
        );
        if config.include_detailed_stats {
            content.push_str(&self.generate_convergence_speed_latex_table(data)?);
        }
        content.push_str(r#"\end{document}"#);
        Ok(content)
    }
    fn generate_markdown(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let mut content = String::from("# Convergence Analysis Report\n\n");
        if config.include_detailed_stats {
            content.push_str(&self.generate_convergence_speed_markdown_table(data)?);
        }
        Ok(content)
    }
    fn generate_csv(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let mut content = String::from(
            "Optimizer,Mean_FuncEvals_50%,Mean_FuncEvals_90%,Final_Convergence_FuncEvals\n",
        );
        let optimizer_averages = self.calculate_convergence_averages(data)?;
        for (optimizer, avg_50, avg_90, avg_final) in optimizer_averages {
            content.push_str(&format!(
                "{},{:.1},{:.1},{:.1}\n",
                optimizer, avg_50, avg_90, avg_final
            ));
        }
        Ok(content)
    }
    fn calculate_convergence_averages(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<Vec<(String, f64, f64, f64)>> {
        let mut optimizer_speed_data = HashMap::new();
        for (_, results) in data {
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
        Ok(optimizer_averages)
    }
    fn generate_convergence_speed_html_table(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        let optimizer_averages = self.calculate_convergence_averages(data)?;
        if optimizer_averages.is_empty() {
            return Ok(String::new());
        }
        let mut content = String::from(
            r#"
    <h2>Convergence Speed Analysis</h2>
    <table>
        <thead>
            <tr>
                <th>Optimizer</th>
                <th>Mean Function Evaluations to 50% Improvement</th>
                <th>Mean Function Evaluations to 90% Improvement</th>
                <th>Final Convergence Function Evaluations</th>
            </tr>
        </thead>
        <tbody>
"#,
        );
        for (i, (optimizer, avg_50, avg_90, avg_final)) in optimizer_averages.iter().enumerate() {
            let class = if i == 0 {
                "best"
            } else if optimizer.contains("QQN") {
                "qqn"
            } else {
                ""
            };
            content.push_str(&format!(
                r#"            <tr>
                <td class="{}">{}</td>
                <td>{:.1}</td>
                <td>{:.1}</td>
                <td>{:.1}</td>
            </tr>
"#,
                class, optimizer, avg_50, avg_90, avg_final
            ));
        }
        content.push_str(r#"        </tbody>
    </table>
    <p><strong>Purpose:</strong> Compares convergence rates for different optimizers based on total function evaluations (function + gradient evaluations). Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.</p>
"#);
        Ok(content)
    }
    fn generate_convergence_speed_markdown_table(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        let optimizer_averages = self.calculate_convergence_averages(data)?;
        if optimizer_averages.is_empty() {
            return Ok(String::new());
        }
        let mut content = String::from(
            r#"## Convergence Speed Analysis
| Optimizer | Mean Function Evaluations to 50% Improvement | Mean Function Evaluations to 90% Improvement | Final Convergence Function Evaluations |
|-----------|-----------------------------------------------|-----------------------------------------------|----------------------------------------|
"#,
        );
        for (optimizer, avg_50, avg_90, avg_final) in optimizer_averages {
            content.push_str(&format!(
                "| {} | {:.1} | {:.1} | {:.1} |\n",
                optimizer, avg_50, avg_90, avg_final
            ));
        }
        content.push_str("\n**Purpose:** Compares convergence rates for different optimizers based on total function evaluations (function + gradient evaluations). Sorted by fastest overall convergence (weighted average).\n\n");
        Ok(content)
    }

    fn generate_convergence_speed_latex_table(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        // Convert HTML table to LaTeX format
        let html_content = self.generate_convergence_speed_html_table(data)?;
        // Simple conversion - you might want to make this more sophisticated
        let latex_content = html_content
            .replace("<table>", "\\begin{tabular}{|l|c|c|c|}\n\\hline")
            .replace("</table>", "\\end{tabular}")
            .replace("<tr>", "")
            .replace("</tr>", " \\\\\n\\hline")
            .replace("<th>", "")
            .replace("</th>", " & ")
            .replace("<td>", "")
            .replace("</td>", " & ");
        Ok(latex_content)
    }
}
impl Report for ConvergenceAnalysisReport {
    fn name(&self) -> &'static str {
        "convergence_analysis"
    }
    fn description(&self) -> &'static str {
        "Analyzes convergence speed and patterns across optimizers, showing mean iterations to reach improvement milestones"
    }
    fn generate_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> anyhow::Result<String> {
        match config.format {
            ReportFormat::Html => self.generate_html(data, config),
            ReportFormat::Latex => self.generate_latex(data, config),
            ReportFormat::Markdown => self.generate_markdown(data, config),
            ReportFormat::Csv => self.generate_csv(data, config),
        }
    }
    fn export_to_file(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
        output_path: &Path,
    ) -> anyhow::Result<()> {
        let content = self.generate_content(data, config)?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!(
                    "Failed to create parent directories for: {}",
                    output_path.display()
                )
            })?;
        }
        fs::write(output_path, content).with_context(|| {
            format!(
                "Failed to write convergence analysis report to: {}",
                output_path.display()
            )
        })?;
        Ok(())
    }
    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> anyhow::Result<()> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("No benchmark data provided"));
        }
        let has_convergence_data = data.iter().any(|(_, results)| {
            results
                .results
                .iter()
                .any(|r| r.convergence_achieved && !r.trace.iterations.is_empty())
        });
        if !has_convergence_data {
            return Err(anyhow::anyhow!(
                "No convergence data available for analysis"
            ));
        }
        Ok(())
    }
    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let total_problems = data.len();
        let total_runs: usize = data.iter().map(|(_, results)| results.results.len()).sum();
        let convergent_runs: usize = data
            .iter()
            .map(|(_, results)| {
                results
                    .results
                    .iter()
                    .filter(|r| r.convergence_achieved)
                    .count()
            })
            .sum();
        let mut metadata = HashMap::new();
        metadata.insert("total_problems".to_string(), total_problems.to_string());
        metadata.insert("total_runs".to_string(), total_runs.to_string());
        metadata.insert("convergent_runs".to_string(), convergent_runs.to_string());
        metadata.insert(
            "convergence_rate".to_string(),
            format!(
                "{:.1}%",
                (convergent_runs as f64 / total_runs as f64) * 100.0
            ),
        );
        let optimizer_count = data
            .iter()
            .flat_map(|(_, results)| &results.results)
            .map(|r| &r.optimizer_name)
            .collect::<std::collections::HashSet<_>>()
            .len();
        let data_points = data.iter().map(|(_, results)| results.results.len()).sum();

        ReportMetadata {
            report_type: self.name().to_string(),
            generated_at: chrono::Utc::now(),
            problem_count: total_problems,
            optimizer_count,
            data_points,
        }
    }
    fn supported_formats(&self) -> Vec<ReportFormat> {
        vec![
            ReportFormat::Html,
            ReportFormat::Latex,
            ReportFormat::Markdown,
            ReportFormat::Csv,
        ]
    }
}
impl Default for ConvergenceAnalysisReport {
    fn default() -> Self {
        Self::new()
    }
}
// Legacy function for backward compatibility - now uses the unified report

/// Generate convergence speed table content (without document wrapper)
#[deprecated(note = "Use ConvergenceAnalysisReport::new().generate_content() instead")]
pub fn generate_convergence_speed_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    let report = ConvergenceAnalysisReport::new();
    let config = ReportConfig {
        format: ReportFormat::Latex,
        include_detailed_stats: true,
        ..Default::default()
    };
    report.generate_content(all_results, &config)
}
// Keep the original implementation for backward compatibility
fn generate_convergence_speed_table_content_legacy(
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
\caption{Convergence Speed Analysis: Mean Function Evaluations to Reach Improvement Milestones}
\label{tab:convergence_speed}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{lrrr}
\toprule
\textbf{Optimizer} & \textbf{Mean Function Evals} & \textbf{Mean Function Evals} & \textbf{Final Convergence} \\
 & \textbf{to 50\% Improvement} & \textbf{to 90\% Improvement} & \textbf{Function Evals} \\
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
            "{optimizer_style} & {avg_50:.1} & {avg_90:.1} & {avg_final:.1} \\\\\n"
        ));
    }
    content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Compares convergence rates for different optimizers based on total function evaluations (function + gradient evaluations). Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.
"#,
    );
    Ok(content)
}
#[deprecated(note = "Use ConvergenceAnalysisReport for unified reporting")]

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
            content.push_str(&format!("- {reason}: {count} runs\n"));
        }
        content.push('\n');
    }
    Ok(content)
}

/// Generate convergence speed analysis LaTeX table
#[deprecated(note = "Use ConvergenceAnalysisReport::export_to_file() instead")]
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
\textbf{Optimizer} & \textbf{Mean Function Evals} & \textbf{Mean Function Evals} & \textbf{Final Convergence} \\
 & \textbf{to 50\% Improvement} & \textbf{to 90\% Improvement} & \textbf{Function Evals} \\
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
            "{optimizer_style} & {avg_50:.1} & {avg_90:.1} & {avg_final:.1} \\\\\n"
        ));
    }
    latex_content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Compares convergence rates for different optimizers based on total function evaluations (function + gradient evaluations). Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.
\end{document}
"#,
    );
    let latex_path = latex_dir.join("convergence_speed_table.tex");
    fs::create_dir_all(latex_dir)
        .with_context(|| format!("Failed to create LaTeX directory: {}", latex_dir.display()))?;
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
#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::UnifiedReportTestSuite;
    #[test]
    fn test_convergence_analysis_report() {
        let report = ConvergenceAnalysisReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }
    #[test]
    fn test_convergence_analysis_basic_functionality() {
        let report = ConvergenceAnalysisReport::new();
        assert_eq!(report.name(), "convergence_analysis");
        assert!(!report.description().is_empty());
        assert_eq!(report.supported_formats().len(), 4);
        assert!(report.supported_formats().contains(&ReportFormat::Html));
        assert!(report.supported_formats().contains(&ReportFormat::Latex));
    }
}
