//! Performance table report implementation using the unified report trait.

use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::unified_report::{Report, ReportConfig, ReportFormat};
use anyhow::Result;
use std::collections::HashMap;

/// Performance table report that provides detailed performance metrics
/// for each optimizer-problem combination.
pub struct PerformanceTableReport;

impl PerformanceTableReport {
    /// Create a new performance table report
    pub fn new() -> Self {
        Self
    }
}

impl Default for PerformanceTableReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for PerformanceTableReport {
    fn name(&self) -> &'static str {
        "performance_table"
    }

    fn description(&self) -> &'static str {
        "Detailed performance table showing metrics for each optimizer-problem combination"
    }

    fn generate_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        config: &ReportConfig,
    ) -> Result<String> {
        self.validate_data(data)?;

        match config.format {
            ReportFormat::Html => self.generate_html_content(data, config),
            ReportFormat::Latex => self.generate_latex_content(data, config),
            ReportFormat::Markdown => self.generate_markdown_content(data, config),
            ReportFormat::Csv => self.generate_csv_content(data, config),
        }
    }
}

impl PerformanceTableReport {
    fn generate_html_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
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
        .metric { text-align: right; }
        .best { background-color: #d4edda; font-weight: bold; }
        .problem-header { background-color: #e9ecef; font-size: 1.1em; }
    </style>
</head>
<body>
    <h1>Performance Table Report</h1>
    <p>Generated on: "#,
        );

        html.push_str(
            &chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        );
        html.push_str("</p>");

        for (problem, results) in data {
            html.push_str(&format!(
                r#"<h2>Problem: {}</h2>
<table>
<tr>
    <th>Optimizer</th>
    <th>Success Rate (%)</th>
    <th>Mean Final Value</th>
    <th>Best Value</th>
    <th>Mean Function Evals</th>
    <th>Mean Time (s)</th>
</tr>
"#,
                problem.get_name()
            ));

            let mut optimizer_stats = HashMap::new();
            for result in &results.results {
                let stats = optimizer_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                stats.push(result);
            }

            let mut perf_data = Vec::new();
            for (optimizer, runs) in &optimizer_stats {
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let success_rate = success_count as f64 / runs.len() as f64 * 100.0;

                let final_values: Vec<f64> = runs
                    .iter()
                    .map(|r| r.final_value)
                    .filter(|&v| v.is_finite())
                    .collect();

                let mean_final = if !final_values.is_empty() {
                    final_values.iter().sum::<f64>() / final_values.len() as f64
                } else {
                    f64::INFINITY
                };

                let best_final = final_values.iter().cloned().fold(f64::INFINITY, f64::min);

                let mean_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;

                let mean_time = runs
                    .iter()
                    .map(|r| r.execution_time.as_secs_f64())
                    .sum::<f64>()
                    / runs.len() as f64;

                perf_data.push((
                    optimizer.clone(),
                    success_rate,
                    mean_final,
                    best_final,
                    mean_func_evals,
                    mean_time,
                ));
            }

            // Sort by success rate, then by best value
            perf_data.sort_by(|a, b| {
                match b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal) {
                    std::cmp::Ordering::Equal => {
                        a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal)
                    }
                    other => other,
                }
            });

            for (
                i,
                (optimizer, success_rate, mean_final, best_final, mean_func_evals, mean_time),
            ) in perf_data.iter().enumerate()
            {
                let class = if i == 0 { " class=\"best\"" } else { "" };
                html.push_str(&format!(
                    "<tr{}><td>{}</td><td class=\"metric\">{:.1}</td><td class=\"metric\">{:.2e}</td><td class=\"metric\">{:.2e}</td><td class=\"metric\">{:.1}</td><td class=\"metric\">{:.3}</td></tr>\n",
                    class, optimizer, success_rate, mean_final, best_final, mean_func_evals, mean_time
                ));
            }

            html.push_str("</table>\n");
        }

        html.push_str("</body></html>");
        Ok(html)
    }

    fn generate_latex_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let mut latex = String::from(
            r#"\documentclass{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{siunitx}
\usepackage{longtable}
\title{Performance Table Report}
\author{QQN Optimizer Benchmark}
\date{\today}
\begin{document}
\maketitle

"#,
        );

        for (problem, results) in data {
            latex.push_str(&format!(
                r#"\section{{Problem: {}}}
\begin{{longtable}}{{p{{3cm}}*{{5}}{{c}}}}
\toprule
\textbf{{Optimizer}} & \textbf{{Success Rate (\%)}} & \textbf{{Mean Final Value}} & \textbf{{Best Value}} & \textbf{{Mean Func Evals}} & \textbf{{Mean Time (s)}} \\
\midrule
"#,
                self.escape_latex(&problem.get_name())
            ));

            let mut optimizer_stats = HashMap::new();
            for result in &results.results {
                let stats = optimizer_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                stats.push(result);
            }

            for (optimizer, runs) in &optimizer_stats {
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let success_rate = success_count as f64 / runs.len() as f64 * 100.0;

                let final_values: Vec<f64> = runs
                    .iter()
                    .map(|r| r.final_value)
                    .filter(|&v| v.is_finite())
                    .collect();

                let mean_final = if !final_values.is_empty() {
                    final_values.iter().sum::<f64>() / final_values.len() as f64
                } else {
                    f64::INFINITY
                };

                let best_final = final_values.iter().cloned().fold(f64::INFINITY, f64::min);

                let mean_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;

                let mean_time = runs
                    .iter()
                    .map(|r| r.execution_time.as_secs_f64())
                    .sum::<f64>()
                    / runs.len() as f64;

                latex.push_str(&format!(
                    "{} & {:.1} & {:.2e} & {:.2e} & {:.1} & {:.3} \\\\\n",
                    self.escape_latex(optimizer),
                    success_rate,
                    mean_final,
                    best_final,
                    mean_func_evals,
                    mean_time
                ));
            }

            latex.push_str(
                r#"\bottomrule
\end{longtable}

"#,
            );
        }

        latex.push_str("\\end{document}");
        Ok(latex)
    }

    fn generate_markdown_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let mut md = String::from("# Performance Table Report\n\n");
        md.push_str(&format!(
            "Generated on: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        for (problem, results) in data {
            md.push_str(&format!("## Problem: {}\n\n", problem.get_name()));
            md.push_str("| Optimizer | Success Rate (%) | Mean Final Value | Best Value | Mean Func Evals | Mean Time (s) |\n");
            md.push_str("|-----------|------------------|------------------|------------|-----------------|---------------|\n");

            let mut optimizer_stats = HashMap::new();
            for result in &results.results {
                let stats = optimizer_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                stats.push(result);
            }

            for (optimizer, runs) in &optimizer_stats {
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let success_rate = success_count as f64 / runs.len() as f64 * 100.0;

                let final_values: Vec<f64> = runs
                    .iter()
                    .map(|r| r.final_value)
                    .filter(|&v| v.is_finite())
                    .collect();

                let mean_final = if !final_values.is_empty() {
                    final_values.iter().sum::<f64>() / final_values.len() as f64
                } else {
                    f64::INFINITY
                };

                let best_final = final_values.iter().cloned().fold(f64::INFINITY, f64::min);

                let mean_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;

                let mean_time = runs
                    .iter()
                    .map(|r| r.execution_time.as_secs_f64())
                    .sum::<f64>()
                    / runs.len() as f64;

                md.push_str(&format!(
                    "| {} | {:.1} | {:.2e} | {:.2e} | {:.1} | {:.3} |\n",
                    optimizer, success_rate, mean_final, best_final, mean_func_evals, mean_time
                ));
            }

            md.push_str("\n");
        }

        Ok(md)
    }

    fn generate_csv_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let mut csv = String::from("Problem,Optimizer,Success_Rate,Mean_Final_Value,Best_Value,Mean_Func_Evals,Mean_Time\n");

        for (problem, results) in data {
            let problem_name = problem.get_name();

            let mut optimizer_stats = HashMap::new();
            for result in &results.results {
                let stats = optimizer_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                stats.push(result);
            }

            for (optimizer, runs) in &optimizer_stats {
                let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                let success_rate = success_count as f64 / runs.len() as f64 * 100.0;

                let final_values: Vec<f64> = runs
                    .iter()
                    .map(|r| r.final_value)
                    .filter(|&v| v.is_finite())
                    .collect();

                let mean_final = if !final_values.is_empty() {
                    final_values.iter().sum::<f64>() / final_values.len() as f64
                } else {
                    f64::INFINITY
                };

                let best_final = final_values.iter().cloned().fold(f64::INFINITY, f64::min);

                let mean_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;

                let mean_time = runs
                    .iter()
                    .map(|r| r.execution_time.as_secs_f64())
                    .sum::<f64>()
                    / runs.len() as f64;

                csv.push_str(&format!(
                    "{},{},{:.1},{:.2e},{:.2e},{:.1},{:.3}\n",
                    problem_name,
                    optimizer,
                    success_rate,
                    mean_final,
                    best_final,
                    mean_func_evals,
                    mean_time
                ));
            }
        }

        Ok(csv)
    }

    fn escape_latex(&self, text: &str) -> String {
        text.replace("_", "\\_")
            .replace("&", "\\&")
            .replace("%", "\\%")
            .replace("$", "\\$")
            .replace("#", "\\#")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::unified_report_tests::UnifiedReportTestSuite;

    #[test]
    fn test_performance_table_report_basic() {
        let report = PerformanceTableReport::new();
        assert_eq!(report.name(), "performance_table");
        assert!(report.description().contains("performance table"));
    }

    #[test]
    fn test_performance_table_report_with_unified_suite() {
        let report = PerformanceTableReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }

    #[test]
    fn test_performance_table_all_formats() {
        let report = PerformanceTableReport::new();
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        // Test HTML
        let html_config = ReportConfig {
            format: ReportFormat::Html,
            ..Default::default()
        };
        let html_content = report.generate_content(&data_refs, &html_config).unwrap();
        assert!(html_content.contains("<!DOCTYPE html>"));
        assert!(html_content.contains("Performance Table Report"));

        // Test Markdown
        let md_config = ReportConfig {
            format: ReportFormat::Markdown,
            ..Default::default()
        };
        let md_content = report.generate_content(&data_refs, &md_config).unwrap();
        assert!(md_content.contains("# Performance Table Report"));
        assert!(md_content.contains("| Optimizer |"));

        // Test CSV
        let csv_config = ReportConfig {
            format: ReportFormat::Csv,
            ..Default::default()
        };
        let csv_content = report.generate_content(&data_refs, &csv_config).unwrap();
        assert!(csv_content.contains("Problem,Optimizer"));

        // Test LaTeX
        let latex_config = ReportConfig {
            format: ReportFormat::Latex,
            ..Default::default()
        };
        let latex_content = report.generate_content(&data_refs, &latex_config).unwrap();
        assert!(latex_content.contains("\\documentclass"));
    }
}
