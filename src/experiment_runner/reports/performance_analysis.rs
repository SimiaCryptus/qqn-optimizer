use crate::benchmarks::evaluation::{is_no_threshold_mode, BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner;
use crate::experiment_runner::{Report, ReportConfig, ReportFormat, ReportMetadata};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub struct PerformanceAnalysisReport;

impl PerformanceAnalysisReport {
    pub fn new() -> Self {
        Self
    }

    fn generate_analysis_content(&self, runs: &[&SingleResult]) -> String {
        let mut content = String::from("## Performance Analysis\n\n");
        let total_func_evals: usize = runs.iter().map(|r| r.function_evaluations).sum();
        let total_grad_evals: usize = runs.iter().map(|r| r.gradient_evaluations).sum();
        let total_time: f64 = runs.iter().map(|r| r.execution_time.as_secs_f64()).sum();
        let total_iterations: usize = runs.iter().map(|r| r.iterations).sum();
        let avg_func_evals = total_func_evals as f64 / runs.len() as f64;
        let avg_grad_evals = total_grad_evals as f64 / runs.len() as f64;
        let avg_time = total_time / runs.len() as f64;
        let avg_iterations = total_iterations as f64 / runs.len() as f64;

        content.push_str(&format!(
            r#"### Computational Efficiency
- **Average Function Evaluations per Run:** {:.1}
- **Average Gradient Evaluations per Run:** {:.1}
- **Average Iterations per Run:** {:.1}
- **Average Time per Run:** {:.3}s
- **Function Evaluations per Second:** {:.1}
- **Iterations per Second:** {:.1}
### Resource Utilization
- **Total Function Evaluations:** {}
- **Total Gradient Evaluations:** {}
- **Total Computation Time:** {:.1}s
- **Function/Gradient Ratio:** {:.2}
"#,
            avg_func_evals,
            avg_grad_evals,
            avg_iterations,
            avg_time,
            if avg_time > 0.0 {
                avg_func_evals / avg_time
            } else {
                0.0
            },
            if avg_time > 0.0 {
                avg_iterations / avg_time
            } else {
                0.0
            },
            total_func_evals,
            total_grad_evals,
            total_time,
            if total_grad_evals > 0 {
                total_func_evals as f64 / total_grad_evals as f64
            } else {
                0.0
            }
        ));
        content
    }

    fn generate_html(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> Result<String> {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Performance Analysis Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        h2 { color: #2c3e50; border-bottom: 2px solid #3498db; }
        h3 { color: #34495e; margin-top: 30px; }
        ul { line-height: 1.6; }
        li { margin: 5px 0; }
        .problem-section { margin: 30px 0; padding: 20px; border: 1px solid #ddd; border-radius: 5px; }
    </style>
</head>
<body>
"#,
        );

        for (problem_spec, results) in data {
            html.push_str(&format!(
                r#"<div class="problem-section">
<h2>Problem: {}</h2>
"#,
                problem_spec.name.as_deref().unwrap_or("Unknown")
            ));

            // Group results by optimizer
            let mut optimizer_results: HashMap<String, Vec<&SingleResult>> =
                HashMap::new();
            for result in &results.results {
                optimizer_results
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }

            for (optimizer_name, runs) in optimizer_results {
                html.push_str(&format!("<h3>Optimizer: {}</h3>\n", optimizer_name));
                let analysis = self.generate_analysis_content(&runs);
                // Convert markdown to basic HTML
                let html_analysis = analysis
                    .replace("## ", "<h2>")
                    .replace("### ", "<h3>")
                    .replace("- **", "<li><strong>")
                    .replace(":**", ":</strong>")
                    .replace("\n\n", "</ul>\n\n<ul>\n")
                    .replace("\n", "</li>\n");
                html.push_str(&format!("<ul>\n{}</li>\n</ul>\n", html_analysis));
            }
            html.push_str("</div>\n");
        }

        html.push_str("</body>\n</html>");
        Ok(html)
    }

    fn generate_latex(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> Result<String> {
        let mut latex = String::from(
            r#"\documentclass{article}
\usepackage[utf8]{inputenc}
\usepackage{amsmath}
\usepackage{booktabs}
\usepackage{geometry}
\geometry{margin=1in}
\title{Performance Analysis Report}
\date{\today}
\begin{document}
\maketitle

"#,
        );

        for (problem_spec, results) in data {
            latex.push_str(&format!(
                "\\section{{Problem: {}}}\n\n",
                problem_spec.name.as_deref().unwrap_or("Unknown")
            ));

            // Group results by optimizer
            let mut optimizer_results: HashMap<String, Vec<&SingleResult>> =
                HashMap::new();
            for result in &results.results {
                optimizer_results
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }

            for (optimizer_name, runs) in optimizer_results {
                latex.push_str(&format!(
                    "\\subsection{{Optimizer: {}}}\n\n",
                    optimizer_name
                ));
                let analysis = self.generate_analysis_content(&runs);
                // Convert markdown to LaTeX
                let latex_analysis = analysis
                    .replace("## ", "\\section{")
                    .replace("### ", "\\subsection{")
                    .replace("- **", "\\item \\textbf{")
                    .replace(":**", ":}")
                    .replace("\n\n", "}\n\n\\begin{itemize}\n")
                    .replace("\n", "}\n");
                latex.push_str(&format!(
                    "\\begin{{itemize}}\n{}\\end{{itemize}}\n\n",
                    latex_analysis
                ));
            }
        }

        latex.push_str("\\end{document}");
        Ok(latex)
    }

    fn generate_markdown(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> Result<String> {
        let mut markdown = String::from("# Performance Analysis Report\n\n");

        for (problem_spec, results) in data {
            markdown.push_str(&format!(
                "## Problem: {}\n\n",
                problem_spec.name.as_deref().unwrap_or("Unknown")
            ));

            // Group results by optimizer
            let mut optimizer_results: HashMap<String, Vec<&SingleResult>> =
                HashMap::new();
            for result in &results.results {
                optimizer_results
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }

            for (optimizer_name, runs) in optimizer_results {
                markdown.push_str(&format!("### Optimizer: {}\n\n", optimizer_name));
                let analysis = self.generate_analysis_content(&runs);
                markdown.push_str(&analysis);
                markdown.push_str("\n\n");
            }
        }

        Ok(markdown)
    }

    fn generate_csv(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> Result<String> {
        let mut csv = String::from("Problem,Optimizer,Avg_Function_Evals,Avg_Gradient_Evals,Avg_Iterations,Avg_Time_Sec,Total_Function_Evals,Total_Gradient_Evals,Total_Time_Sec,Function_Gradient_Ratio\n");

        for (problem_spec, results) in data {
            // Group results by optimizer
            let mut optimizer_results: HashMap<String, Vec<&SingleResult>> =
                HashMap::new();
            for result in &results.results {
                optimizer_results
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }

            for (optimizer_name, runs) in optimizer_results {
                let total_func_evals: usize = runs.iter().map(|r| r.function_evaluations).sum();
                let total_grad_evals: usize = runs.iter().map(|r| r.gradient_evaluations).sum();
                let total_time: f64 = runs.iter().map(|r| r.execution_time.as_secs_f64()).sum();
                let total_iterations: usize = runs.iter().map(|r| r.iterations).sum();
                let avg_func_evals = total_func_evals as f64 / runs.len() as f64;
                let avg_grad_evals = total_grad_evals as f64 / runs.len() as f64;
                let avg_time = total_time / runs.len() as f64;
                let avg_iterations = total_iterations as f64 / runs.len() as f64;
                let func_grad_ratio = if total_grad_evals > 0 {
                    total_func_evals as f64 / total_grad_evals as f64
                } else {
                    0.0
                };

                csv.push_str(&format!(
                    "{},{},{:.1},{:.1},{:.1},{:.3},{},{},{:.1},{:.2}\n",
                    problem_spec.name.as_deref().unwrap_or("Unknown"),
                    &optimizer_name,
                    avg_func_evals,
                    avg_grad_evals,
                    avg_iterations,
                    avg_time,
                    total_func_evals,
                    total_grad_evals,
                    total_time,
                    func_grad_ratio
                ));
            }
        }

        Ok(csv)
    }
}

impl Report for PerformanceAnalysisReport {
    fn name(&self) -> &'static str {
        "performance_analysis"
    }

    fn description(&self) -> &'static str {
        "Detailed performance analysis including computational efficiency and resource utilization metrics"
    }

    fn generate_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> Result<String> {
        match config.format {
            ReportFormat::Html => self.generate_html(data),
            ReportFormat::Latex => self.generate_latex(data),
            ReportFormat::Markdown => self.generate_markdown(data),
            ReportFormat::Csv => self.generate_csv(data),
        }
    }

    fn export_to_file(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
        output_path: &Path,
    ) -> Result<()> {
        let content = self.generate_content(data, config)?;
        if let Some(parent) = output_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(output_path, content)?;
        Ok(())
    }

    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("No benchmark data provided"));
        }

        for (problem_spec, results) in data {
            if problem_spec.name.is_none() {
                return Err(anyhow::anyhow!("Problem spec has empty name"));
            }
            if results.results.is_empty() {
                return Err(anyhow::anyhow!(
                    "No results for problem: {}",
                    problem_spec.name.as_deref().unwrap_or("Unknown")
                ));
            }
        }
        Ok(())
    }

    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let total_problems = data.len();
        let total_optimizers: usize = data
            .iter()
            .flat_map(|(_, results)| &results.results)
            .map(|r| &r.optimizer_name)
            .collect::<std::collections::HashSet<_>>()
            .len();
        let total_runs: usize = data.iter().map(|(_, results)| results.results.len()).sum();

        let mut metadata = HashMap::new();
        metadata.insert("total_problems".to_string(), total_problems.to_string());
        metadata.insert("total_optimizers".to_string(), total_optimizers.to_string());
        metadata.insert("total_runs".to_string(), total_runs.to_string());
        metadata.insert(
            "report_type".to_string(),
            "performance_analysis".to_string(),
        );

        ReportMetadata {
            report_type: "performance_analysis".to_string(),
            generated_at: chrono::Utc::now(),
            problem_count: total_problems,
            optimizer_count: total_optimizers,
            data_points: total_runs,
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

// Legacy function for backward compatibility
pub fn generate_performance_analysis(runs: &[&SingleResult]) -> Result<String> {
    let report = PerformanceAnalysisReport::new();
    Ok(report.generate_analysis_content(runs))
}

/// Performance table report showing detailed metrics for each optimizer-problem combination
pub struct PerformanceTableReport;

impl PerformanceTableReport {
    pub fn new() -> Self {
        Self
    }
    fn calculate_performance_data(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> Vec<(String, Vec<(String, f64, f64, f64, f64, f64, f64, f64)>)> {
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

            if is_no_threshold_mode() {
                // In no-threshold mode, sort by best value
                perf_data.sort_by(|a, b| {
                    a.3.total_cmp(&b.3)
                });
            } else {
                // Sort by success rate first, then by total evaluations
                perf_data.sort_by(|a, b| {
                    use std::cmp::Ordering;
                    match b.7.total_cmp(&a.7) {
                        Ordering::Equal => (a.5 + a.6).total_cmp(&(b.5 + b.6)),
                        other => other,
                    }
                });
            }

            problem_data.push((problem_name, perf_data));
        }
        problem_data
    }
    fn generate_html(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
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
                        "<td rowspan=\"{}\" class=\"problem-header\">{}</td>",
                        perf_data.len(),
                        problem_name
                    )
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
                    row_class,
                    problem_cell,
                    optimizer,
                    mean_final,
                    std_final,
                    best_final,
                    worst_final,
                    mean_func_evals,
                    success_rate,
                    mean_time
                ));
            }
        }
        html.push_str("</tbody></table></body></html>");
        Ok(html)
    }
    fn generate_latex(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> Result<String> {
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
\usepackage{graphicx}
\begin{document}
\footnotesize
\begin{longtable}{|l|l|c|c|c|c|c|c|c|}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
\hline
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\hline
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\hline
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\hline
\midrule
\endhead
\hline
\multicolumn{9}{r}{{Continued on next page}} \\
\hline
\endfoot
\hline
\bottomrule
\endlastfoot
"#,
        );
        for (problem_name, perf_data) in performance_data {
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
                    experiment_runner::escape_latex(&problem_name)
                } else {
                    String::new()
                };
                let optimizer_style = if i == 0 {
                    format!("\\textbf{{{}}}", experiment_runner::escape_latex(optimizer))
                } else {
                    experiment_runner::escape_latex(optimizer)
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
                if i < perf_data.len() - 1 {
                    latex_content.push_str("\\hline\n");
                }
            }
        }
        latex_content.push_str(
            r#"\end{longtable}
\end{document}
"#,
        );
        Ok(latex_content)
    }
    fn generate_markdown(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let performance_data = self.calculate_performance_data(data);
        let mut markdown = String::from("# Performance Table Report\n\n");
        markdown
            .push_str("Detailed performance metrics for each optimizer-problem combination.\n\n");
        for (problem_name, perf_data) in performance_data {
            markdown.push_str(&format!("## {}\n\n", problem_name));
            markdown.push_str("| Optimizer | Mean Final Value | Std Dev | Best Value | Worst Value | Mean Func Evals | Success Rate (%) | Mean Time (s) |\n");
            markdown.push_str("|-----------|------------------|---------|------------|-------------|-----------------|------------------|---------------|\n");
            for (
                optimizer,
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_func_evals,
                success_rate,
                mean_time,
            ) in perf_data
            {
                markdown.push_str(&format!(
                    "| {} | {:.2e} | {:.2e} | {:.2e} | {:.2e} | {:.1} | {:.1} | {:.3} |\n",
                    optimizer,
                    mean_final,
                    std_final,
                    best_final,
                    worst_final,
                    mean_func_evals,
                    success_rate,
                    mean_time
                ));
            }
            markdown.push_str("\n");
        }
        Ok(markdown)
    }
    fn generate_csv(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let performance_data = self.calculate_performance_data(data);
        let mut csv = String::from("Problem,Optimizer,Mean Final Value,Std Dev,Best Value,Worst Value,Mean Func Evals,Success Rate (%),Mean Time (s)\n");
        for (problem_name, perf_data) in performance_data {
            for (
                optimizer,
                mean_final,
                std_final,
                best_final,
                worst_final,
                mean_func_evals,
                success_rate,
                mean_time,
            ) in perf_data
            {
                csv.push_str(&format!(
                    "{},{},{:.2e},{:.2e},{:.2e},{:.2e},{:.1},{:.1},{:.3}\n",
                    problem_name,
                    optimizer,
                    mean_final,
                    std_final,
                    best_final,
                    worst_final,
                    mean_func_evals,
                    success_rate,
                    mean_time
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
    fn generate_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> Result<String> {
        match config.format {
            ReportFormat::Html => self.generate_html(data, config),
            ReportFormat::Latex => self.generate_latex(data),
            ReportFormat::Markdown => self.generate_markdown(data, config),
            ReportFormat::Csv => self.generate_csv(data, config),
        }
    }
    fn export_to_file(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
        output_path: &Path,
    ) -> Result<()> {
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
                "Failed to write performance table report to: {}",
                output_path.display()
            )
        })?;
        Ok(())
    }
    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> Result<()> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("No benchmark data provided"));
        }
        for (problem, results) in data {
            if results.results.is_empty() {
                return Err(anyhow::anyhow!(
                    "No results for problem: {}",
                    problem.get_name()
                ));
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
            report_type: "performance_table".to_string(),
            generated_at: Default::default(),
            problem_count: total_problems,
            optimizer_count: total_optimizers.len(),
            data_points: total_runs,
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

/// Generate main performance LaTeX table
pub fn generate_main_performance_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> Result<()> {
    let report = PerformanceTableReport::new();
    let config = ReportConfig {
        format: ReportFormat::Latex,
        ..Default::default()
    };
    report.generate_content(all_results, &config)?;

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
\usepackage{graphicx}
\begin{document}
\footnotesize
\begin{longtable}{|l|l|c|c|c|c|c|c|c|}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
...
...
\hline
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\hline
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\hline
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\hline
\midrule
\endhead
\hline
\multicolumn{9}{r}{{Continued on next page}} \\
\hline
\endfoot
\hline
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

        if is_no_threshold_mode() {
            // In no-threshold mode, sort by best value
            perf_data.sort_by(|a, b| {
                a.3.total_cmp(&b.3)
            });
        } else {
            // Sort by success rate first, then by total evaluations
            perf_data.sort_by(|a, b| {
                use std::cmp::Ordering;
                match b.7.total_cmp(&a.7) {
                    Ordering::Equal => (a.5 + a.6).total_cmp(&(b.5 + b.6)),
                    other => other,
                }
            });
        }

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
                experiment_runner::escape_latex(&problem_name)
            } else {
                String::new()
            };
            let optimizer_style = if i == 0 {
                format!("\\textbf{{{}}}", experiment_runner::escape_latex(optimizer))
            } else {
                experiment_runner::escape_latex(optimizer)
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
            if i < perf_data.len() - 1 {
                latex_content.push_str("\\hline\n");
            }
        }
    }
    latex_content.push_str(
        r#"\end{longtable}
\end{document}
"#,
    );
    let latex_path = latex_dir.join("main_performance_table.tex");
    fs::create_dir_all(latex_dir)
        .with_context(|| format!("Failed to create LaTeX directory: {}", latex_dir.display()))?;
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
) -> Result<String> {
    let mut content = String::from(
        r#"\footnotesize
\begin{longtable}{|l|l|c|c|c|c|c|c|c|}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
\hline
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\hline
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\hline
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\hline
\midrule
\endhead
\hline
\multicolumn{9}{r}{{Continued on next page}} \\
\hline
\endfoot
\hline
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

        if is_no_threshold_mode() {
            // In no-threshold mode, sort by best value
            perf_data.sort_by(|a, b| {
                a.3.total_cmp(&b.3)
            });
        } else {
            // Sort by success rate first, then by total evaluations
            perf_data.sort_by(|a, b| {
                use std::cmp::Ordering;
                match b.7.total_cmp(&a.7) {
                    Ordering::Equal => (a.5 + a.6).total_cmp(&(b.5 + b.6)),
                    other => other,
                }
            });
        }

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
                experiment_runner::escape_latex(&problem_name)
            } else {
                String::new()
            };
            let optimizer_style = if i == 0 {
                format!("\\textbf{{{}}}", experiment_runner::escape_latex(optimizer))
            } else {
                experiment_runner::escape_latex(optimizer)
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
            if i < perf_data.len() - 1 {
                content.push_str("\\hline\n");
            }
        }
    }
    content.push_str("\\end{longtable}\n");
    Ok(content)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::UnifiedReportTestSuite;

    #[test]
    fn test_performance_analysis_report() {
        let report = PerformanceAnalysisReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }

    #[test]
    fn test_basic_functionality() {
        let report = PerformanceAnalysisReport::new();
        UnifiedReportTestSuite::test_basic_functionality(&report).unwrap();
    }

    #[test]
    fn test_content_generation() {
        let report = PerformanceAnalysisReport::new();
        UnifiedReportTestSuite::test_content_generation(&report).unwrap();
    }

    #[test]
    fn test_data_validation() {
        let report = PerformanceAnalysisReport::new();
        UnifiedReportTestSuite::test_data_validation(&report).unwrap();
    }

    #[test]
    fn test_metadata_generation() {
        let report = PerformanceAnalysisReport::new();
        UnifiedReportTestSuite::test_metadata_generation(&report).unwrap();
    }

    #[test]
    fn test_file_export() {
        let report = PerformanceAnalysisReport::new();
        UnifiedReportTestSuite::test_file_export(&report).unwrap();
    }

    #[test]
    fn test_performance_table_report() {
        let report = PerformanceTableReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }
    #[test]
    fn test_performance_table_basic_functionality() {
        let report = PerformanceTableReport::new();
        assert_eq!(report.name(), "performance_table");
        assert_eq!(
            report.description(),
            "Shows detailed performance metrics for each optimizer-problem combination"
        );
        assert_eq!(
            report.supported_formats(),
            vec![
                ReportFormat::Html,
                ReportFormat::Latex,
                ReportFormat::Markdown,
                ReportFormat::Csv
            ]
        );
    }
}
