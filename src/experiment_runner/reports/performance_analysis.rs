use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner::{Report, ReportConfig, ReportFormat, ReportMetadata};
use anyhow::Result;
use std::collections::HashMap;
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
        config: &ReportConfig,
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
            let mut optimizer_results: std::collections::HashMap<String, Vec<&SingleResult>> = std::collections::HashMap::new();
            for result in &results.results {
                optimizer_results.entry(result.optimizer_name.clone()).or_insert_with(Vec::new).push(result);
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
        config: &ReportConfig,
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
            latex.push_str(&format!("\\section{{Problem: {}}}\n\n", 
                problem_spec.name.as_deref().unwrap_or("Unknown")));

            // Group results by optimizer
            let mut optimizer_results: std::collections::HashMap<String, Vec<&SingleResult>> = std::collections::HashMap::new();
            for result in &results.results {
                optimizer_results.entry(result.optimizer_name.clone()).or_insert_with(Vec::new).push(result);
            }

            for (optimizer_name, runs) in optimizer_results {
                latex.push_str(&format!("\\subsection{{Optimizer: {}}}\n\n", optimizer_name));
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
        config: &ReportConfig,
    ) -> Result<String> {
        let mut markdown = String::from("# Performance Analysis Report\n\n");

        for (problem_spec, results) in data {
            markdown.push_str(&format!("## Problem: {}\n\n", 
                problem_spec.name.as_deref().unwrap_or("Unknown")));

            // Group results by optimizer
            let mut optimizer_results: std::collections::HashMap<String, Vec<&SingleResult>> = std::collections::HashMap::new();
            for result in &results.results {
                optimizer_results.entry(result.optimizer_name.clone()).or_insert_with(Vec::new).push(result);
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
        config: &ReportConfig,
    ) -> Result<String> {
        let mut csv = String::from("Problem,Optimizer,Avg_Function_Evals,Avg_Gradient_Evals,Avg_Iterations,Avg_Time_Sec,Total_Function_Evals,Total_Gradient_Evals,Total_Time_Sec,Function_Gradient_Ratio\n");

        for (problem_spec, results) in data {
            // Group results by optimizer
            let mut optimizer_results: std::collections::HashMap<String, Vec<&SingleResult>> = std::collections::HashMap::new();
            for result in &results.results {
                optimizer_results.entry(result.optimizer_name.clone()).or_insert_with(Vec::new).push(result);
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
    ) -> Result<()> {
        let content = self.generate_content(data, config)?;
        std::fs::write(output_path, content)?;
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
pub fn generate_performance_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let report = PerformanceAnalysisReport::new();
    Ok(report.generate_analysis_content(runs))
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
}