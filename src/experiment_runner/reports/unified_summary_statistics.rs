//! Summary statistics report implementation using the unified report trait.

use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::reports::summary_statistics::generate_summary_statistics_table_content;
use crate::experiment_runner::unified_report::{Report, ReportConfig, ReportFormat};
use anyhow::Result;

/// Summary statistics report that provides aggregate performance metrics
/// grouped by problem family and optimizer.
pub struct SummaryStatisticsReport;

impl SummaryStatisticsReport {
    /// Create a new summary statistics report
    pub fn new() -> Self {
        Self
    }
}

impl Default for SummaryStatisticsReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for SummaryStatisticsReport {
    fn name(&self) -> &'static str {
        "summary_statistics"
    }

    fn description(&self) -> &'static str {
        "Summary statistics showing average performance metrics grouped by problem family and optimizer"
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

impl SummaryStatisticsReport {
    fn generate_html_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Summary Statistics Report</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: left; }
        th { background-color: #f2f2f2; font-weight: bold; }
        .metric { text-align: right; }
        .best { background-color: #d4edda; font-weight: bold; }
    </style>
</head>
<body>
    <h1>Summary Statistics Report</h1>
    <p>Generated on: "#,
        );

        html.push_str(
            &chrono::Utc::now()
                .format("%Y-%m-%d %H:%M:%S UTC")
                .to_string(),
        );
        html.push_str("</p>");

        // Convert LaTeX table to HTML format
        let latex_content = generate_summary_statistics_table_content(data)?;
        let html_table = self.convert_latex_to_html(&latex_content)?;
        html.push_str(&html_table);

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
\usepackage{multirow}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{adjustbox}
\title{Summary Statistics Report}
\author{QQN Optimizer Benchmark}
\date{\today}
\begin{document}
\maketitle

"#,
        );

        latex.push_str(&generate_summary_statistics_table_content(data)?);
        latex.push_str("\n\\end{document}");

        Ok(latex)
    }

    fn generate_markdown_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        let mut md = String::from("# Summary Statistics Report\n\n");
        md.push_str(&format!(
            "Generated on: {}\n\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));

        // Convert LaTeX table to Markdown format
        let latex_content = generate_summary_statistics_table_content(data)?;
        let markdown_table = self.convert_latex_to_markdown(&latex_content)?;
        md.push_str(&markdown_table);

        Ok(md)
    }

    fn generate_csv_content(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> Result<String> {
        use crate::experiment_runner::report_generator::get_family;
        use std::collections::HashMap;

        let mut csv = String::from("Problem_Family,Optimizer,Avg_Success_Rate,Avg_Final_Value,Avg_Func_Evals,Avg_Grad_Evals,Avg_Time\n");

        // Group by problem family (similar to the original logic)
        let mut family_results: HashMap<
            String,
            HashMap<String, Vec<&crate::benchmarks::evaluation::SingleResult>>,
        > = HashMap::new();
        for (problem, results) in data {
            let family = get_family(&problem.get_name());
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
                for (optimizer, runs) in optimizers {
                    let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                    let success_rate = success_count as f32 / runs.len() as f32 * 100.0;

                    let final_values: Vec<f32> = runs
                        .iter()
                        .map(|r| r.final_value)
                        .filter(|&v| v.is_finite())
                        .collect();
                    let avg_final = if !final_values.is_empty() {
                        final_values.iter().sum::<f32>() / final_values.len() as f32
                    } else {
                        f32::INFINITY
                    };

                    let avg_func_evals = runs
                        .iter()
                        .map(|r| r.function_evaluations as f32)
                        .sum::<f32>()
                        / runs.len() as f32;
                    let avg_grad_evals = runs
                        .iter()
                        .map(|r| r.gradient_evaluations as f32)
                        .sum::<f32>()
                        / runs.len() as f32;
                    let avg_time = runs
                        .iter()
                        .map(|r| r.execution_time.as_secs_f32())
                        .sum::<f32>()
                        / runs.len() as f32;

                    csv.push_str(&format!(
                        "{},{},{:.1},{:.2e},{:.1},{:.1},{:.3}\n",
                        family,
                        optimizer,
                        success_rate,
                        avg_final,
                        avg_func_evals,
                        avg_grad_evals,
                        avg_time
                    ));
                }
            }
        }

        Ok(csv)
    }

    fn convert_latex_to_html(&self, latex_content: &str) -> Result<String> {
        // Simple LaTeX to HTML conversion for tables
        // This is a basic implementation - a full converter would be more complex
        let mut html = String::new();

        // Extract table content between \begin{tabular} and \end{tabular}
        if let Some(start) = latex_content.find("\\begin{tabular}") {
            if let Some(end) = latex_content.find("\\end{tabular}") {
                html.push_str("<table>\n");

                let table_content = &latex_content[start..end];
                let lines: Vec<&str> = table_content.lines().collect();

                let mut in_header = true;
                for line in lines {
                    let trimmed = line.trim();
                    if trimmed.contains("\\toprule")
                        || trimmed.contains("\\midrule")
                        || trimmed.contains("\\bottomrule")
                    {
                        continue;
                    }
                    if trimmed.contains("&") && trimmed.ends_with("\\\\") {
                        let row_data = trimmed.trim_end_matches("\\\\");
                        let cells: Vec<&str> = row_data.split(" & ").collect();

                        if in_header {
                            html.push_str("  <tr>\n");
                            for cell in cells {
                                let clean_cell = cell.replace("\\textbf{", "").replace("}", "");
                                html.push_str(&format!("    <th>{}</th>\n", clean_cell));
                            }
                            html.push_str("  </tr>\n");
                            in_header = false;
                        } else {
                            html.push_str("  <tr>\n");
                            for cell in cells {
                                let clean_cell = cell.replace("\\textbf{", "").replace("}", "");
                                html.push_str(&format!("    <td>{}</td>\n", clean_cell));
                            }
                            html.push_str("  </tr>\n");
                        }
                    }
                }

                html.push_str("</table>\n");
            }
        }

        if html.is_empty() {
            html = "<p>Table content could not be converted</p>".to_string();
        }

        Ok(html)
    }

    fn convert_latex_to_markdown(&self, latex_content: &str) -> Result<String> {
        // Simple LaTeX to Markdown conversion for tables
        let mut md = String::new();

        if let Some(start) = latex_content.find("\\begin{tabular}") {
            if let Some(end) = latex_content.find("\\end{tabular}") {
                let table_content = &latex_content[start..end];
                let lines: Vec<&str> = table_content.lines().collect();

                let mut header_written = false;
                for line in lines {
                    let trimmed = line.trim();
                    if trimmed.contains("\\toprule")
                        || trimmed.contains("\\midrule")
                        || trimmed.contains("\\bottomrule")
                    {
                        continue;
                    }
                    if trimmed.contains("&") && trimmed.ends_with("\\\\") {
                        let row_data = trimmed.trim_end_matches("\\\\");
                        let cells: Vec<&str> = row_data.split(" & ").collect();

                        if !header_written {
                            // Write header
                            md.push('|');
                            for cell in &cells {
                                let clean_cell = cell.replace("\\textbf{", "").replace("}", "");
                                md.push_str(&format!(" {} |", clean_cell));
                            }
                            md.push('\n');

                            // Write separator
                            md.push('|');
                            for _ in &cells {
                                md.push_str(" --- |");
                            }
                            md.push('\n');
                            header_written = true;
                        } else {
                            // Write data row
                            md.push('|');
                            for cell in cells {
                                let clean_cell = cell.replace("\\textbf{", "").replace("}", "");
                                md.push_str(&format!(" {} |", clean_cell));
                            }
                            md.push('\n');
                        }
                    }
                }
            }
        }

        if md.is_empty() {
            md = "Table content could not be converted\n".to_string();
        }

        Ok(md)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::evaluation::{
        BenchmarkConfig, ConvergenceReason, OptimizationTrace, PerformanceMetrics, SingleResult,
    };
    use crate::SphereFunction;
    use std::sync::Arc;
    use std::time::Duration;

    fn create_test_data() -> Vec<(ProblemSpec, BenchmarkResults)> {
        let problem_spec = ProblemSpec {
            name: None,
            problem: Arc::new(SphereFunction::new(2)),
            dimensions: Some(2),
            family: "Convex Unimodal".to_string(),
            seed: 0,
        };

        let result = SingleResult {
            problem_name: "".to_string(),
            optimizer_name: "TestOptimizer".to_string(),
            run_id: 0,
            final_value: 1e-6,
            final_gradient_norm: 1e-8,
            iterations: 100,
            function_evaluations: 150,
            gradient_evaluations: 100,
            execution_time: Duration::from_millis(100),
            convergence_achieved: true,
            convergence_reason: ConvergenceReason::FunctionTolerance,
            memory_usage: None,
            best_value: 1e-6,
            trace: OptimizationTrace::default(),
            error_message: None,
            performance_metrics: PerformanceMetrics {
                iterations_per_second: 0.0,
                function_evaluations_per_second: 0.0,
                gradient_evaluations_per_second: 0.0,
                convergence_rate: 0.0,
            },
        };

        let results = BenchmarkResults {
            config: BenchmarkConfig::default(),
            timestamp: Default::default(),
            convergence_achieved: false,
            final_value: None,
            function_evaluations: 0,
            results: vec![result],
            gradient_evaluations: 0,
        };

        vec![(problem_spec, results)]
    }

    #[test]
    fn test_summary_statistics_report_basic() {
        let report = SummaryStatisticsReport::new();
        assert_eq!(report.name(), "summary_statistics");
        assert!(report.description().contains("Summary statistics"));
    }

    #[test]
    fn test_summary_statistics_report_html() {
        let report = SummaryStatisticsReport::new();
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();

        let config = ReportConfig {
            format: ReportFormat::Html,
            ..Default::default()
        };

        let content = report.generate_content(&data_refs, &config).unwrap();
        assert!(content.contains("<!DOCTYPE html>"));
        assert!(content.contains("Summary Statistics Report"));
    }

    #[test]
    fn test_summary_statistics_report_latex() {
        let report = SummaryStatisticsReport::new();
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();

        let config = ReportConfig {
            format: ReportFormat::Latex,
            ..Default::default()
        };

        let content = report.generate_content(&data_refs, &config).unwrap();
        assert!(content.contains("\\documentclass"));
        assert!(content.contains("Summary Statistics Report"));
    }

    #[test]
    fn test_summary_statistics_report_markdown() {
        let report = SummaryStatisticsReport::new();
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();

        let config = ReportConfig {
            format: ReportFormat::Markdown,
            ..Default::default()
        };

        let content = report.generate_content(&data_refs, &config).unwrap();
        assert!(content.contains("# Summary Statistics Report"));
    }

    #[test]
    fn test_summary_statistics_report_csv() {
        let report = SummaryStatisticsReport::new();
        let data = create_test_data();
        let data_refs: Vec<_> = data.iter().map(|(p, r)| (p, r.clone())).collect();

        let config = ReportConfig {
            format: ReportFormat::Csv,
            ..Default::default()
        };

        let content = report.generate_content(&data_refs, &config).unwrap();
        assert!(content.contains("Problem_Family,Optimizer"));
        assert!(content.contains("Sphere,TestOptimizer"));
    }
}
