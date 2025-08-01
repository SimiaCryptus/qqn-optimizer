//! Report generation tools.

use crate::analysis::AnalysisReport;
use crate::optimizers::OptResult;
use std::fs;
use std::io::Write;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ReportConfig {
    pub template: String,
    pub output_format: String,
    pub include_plots: bool,
    pub include_raw_data: bool,
}

impl Default for ReportConfig {
    fn default() -> Self {
        Self {
            template: "academic".to_string(),
            output_format: "latex".to_string(),
            include_plots: true,
            include_raw_data: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AcademicReport {
    pub title: String,
    pub abstract_text: String,
    pub sections: Vec<ReportSection>,
}

#[derive(Debug, Clone)]
pub struct ReportSection {
    pub title: String,
    pub content: String,
    pub subsections: Vec<ReportSection>,
}

/// LaTeX exporter
pub struct LaTeXExporter;

impl Default for LaTeXExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl LaTeXExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export_report(&self, report: &AnalysisReport) -> OptResult<String> {
        let mut latex = String::new();
        
        // Document preamble
        latex.push_str("\\documentclass[11pt,a4paper]{article}\n");
        latex.push_str("\\usepackage{amsmath}\n");
        latex.push_str("\\usepackage{graphicx}\n");
        latex.push_str("\\usepackage{booktabs}\n");
        latex.push_str("\\usepackage{hyperref}\n");
        latex.push_str("\n");
        
        // Document metadata
        latex.push_str("\\title{QQN Optimization Analysis Report}\n");
        latex.push_str("\\author{QQN Optimizer}\n");
        latex.push_str("\\date{\\today}\n");
        latex.push_str("\n");
        
        // Begin document
        latex.push_str("\\begin{document}\n");
        latex.push_str("\\maketitle\n");
        latex.push_str("\n");
        
        // Abstract
        latex.push_str("\\begin{abstract}\n");
        latex.push_str(&report.summary());
        latex.push_str("\n\\end{abstract}\n");
        latex.push_str("\n");
        
        // Main content sections
        latex.push_str("\\section{Introduction}\n");
        latex.push_str("This report presents the results of QQN optimization analysis.\n");
        latex.push_str("\n");
        
        latex.push_str("\\section{Methodology}\n");
        latex.push_str("The analysis was performed using Quasi-Quasi-Newton optimization methods.\n");
        latex.push_str("\n");
        
        latex.push_str("\\section{Results}\n");
        latex.push_str(&self.format_results(report)?);
        latex.push_str("\n");
        
        latex.push_str("\\section{Conclusion}\n");
        latex.push_str(&self.format_conclusion(report)?);
        latex.push_str("\n");
        
        // End document
        latex.push_str("\\end{document}\n");
        
        Ok(latex)
    }
    fn format_results(&self, report: &AnalysisReport) -> OptResult<String> {
        let mut results = String::new();
        // Format convergence analysis
        if let Some(convergence) = report.convergence_analysis() {
            results.push_str("\\subsection{Convergence Analysis}\n");
            results.push_str(&format!(
                "The optimization converged in {} iterations with a final objective value of {:.6e}.\n",
                convergence.iterations, convergence.final_value
            ));
            results.push_str("\n");
        }
        // Format performance metrics
        if let Some(performance) = report.performance_metrics() {
            results.push_str("\\subsection{Performance Metrics}\n");
            results.push_str("\\begin{itemize}\n");
            results.push_str(&format!("\\item Total time: {:.3} seconds\n", performance.total_time));
            results.push_str(&format!("\\item Function evaluations: {}\n", performance.function_evals));
            results.push_str(&format!("\\item Gradient evaluations: {}\n", performance.gradient_evals));
            results.push_str("\\end{itemize}\n");
            results.push_str("\n");
        }
        Ok(results)
    }
    fn format_conclusion(&self, report: &AnalysisReport) -> OptResult<String> {
        Ok(format!(
            "The optimization analysis completed successfully. {}",
            report.summary()
        ))
    }
    pub fn export_to_file(&self, report: &AnalysisReport, path: &Path) -> OptResult<()> {
        let latex_content = self.export_report(report)?;
        let mut file = fs::File::create(path)?;
        file.write_all(latex_content.as_bytes())?;
        Ok(())
    }
}

/// CSV exporter
pub struct CSVExporter;

impl Default for CSVExporter {
    fn default() -> Self {
        Self::new()
    }
}

impl CSVExporter {
    pub fn new() -> Self {
        Self
    }

    pub fn export_report(
        &self,
        report: &AnalysisReport,
        output_dir: &Path,
    ) -> OptResult<()> {
        // Create output directory if it doesn't exist
        fs::create_dir_all(output_dir)?;
        
        // Export convergence data
        if let Some(convergence) = report.convergence_analysis() {
            let convergence_path = output_dir.join("convergence.csv");
            let mut file = fs::File::create(convergence_path)?;
            writeln!(file, "iteration,objective_value,gradient_norm")?;
            
            for (i, (obj, grad)) in convergence.history.iter().enumerate() {
                writeln!(file, "{},{},{}", i, obj, grad)?;
            }
        }
        
        // Export performance metrics
        if let Some(performance) = report.performance_metrics() {
            let metrics_path = output_dir.join("performance_metrics.csv");
            let mut file = fs::File::create(metrics_path)?;
            writeln!(file, "metric,value")?;
            writeln!(file, "total_time,{}", performance.total_time)?;
            writeln!(file, "function_evaluations,{}", performance.function_evals)?;
            writeln!(file, "gradient_evaluations,{}", performance.gradient_evals)?;
        }
        
        Ok(())
    }
}
/// Report generator trait for extensibility
pub trait ReportGenerator {
    fn generate(&self, report: &AnalysisReport, config: &ReportConfig) -> OptResult<String>;
    fn save_to_file(&self, report: &AnalysisReport, config: &ReportConfig, path: &Path) -> OptResult<()>;
}
/// HTML exporter for web-based reports
pub struct HTMLExporter;
impl HTMLExporter {
    pub fn new() -> Self {
        Self
    }
    pub fn export_report(&self, report: &AnalysisReport) -> OptResult<String> {
        let mut html = String::new();
        html.push_str("<!DOCTYPE html>\n");
        html.push_str("<html lang=\"en\">\n");
        html.push_str("<head>\n");
        html.push_str("    <meta charset=\"UTF-8\">\n");
        html.push_str("    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("    <title>QQN Optimization Report</title>\n");
        html.push_str("    <style>\n");
        html.push_str("        body { font-family: Arial, sans-serif; margin: 40px; }\n");
        html.push_str("        h1, h2, h3 { color: #333; }\n");
        html.push_str("        .metric { margin: 10px 0; }\n");
        html.push_str("        .value { font-weight: bold; color: #0066cc; }\n");
        html.push_str("    </style>\n");
        html.push_str("</head>\n");
        html.push_str("<body>\n");
        html.push_str("    <h1>QQN Optimization Analysis Report</h1>\n");
        html.push_str(&format!("    <p>{}</p>\n", report.summary()));
        html.push_str("</body>\n");
        html.push_str("</html>\n");
        Ok(html)
    }
}