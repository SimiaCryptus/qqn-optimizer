//! Family vs Family comparison report implementation for the unified reporting system.
//!
//! This report provides a comprehensive comparison matrix showing how different
//! optimizer families perform across different problem families.

use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::unified_report::{Report, ReportConfig, ReportFormat, ReportMetadata};
use crate::experiment_runner::reports::family_vs_family::{
    generate_family_vs_family_comparison_table,
    generate_family_vs_family_table_content,
    calculate_family_performance_data,
};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::report_generator;
use anyhow::Result;
use std::collections::HashSet;
use std::path::Path;

/// Family vs Family comparison report
pub struct FamilyVsFamilyReport;

impl FamilyVsFamilyReport {
    /// Create a new family vs family report
    pub fn new() -> Self {
        Self
    }

    /// Generate HTML content for the report
    fn generate_html(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> Result<String> {
        let mut content = String::from(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Family vs Family Performance Matrix</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; line-height: 1.6; }
        h1, h2 { color: #333; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: center; }
        th { background-color: #f2f2f2; font-weight: bold; }
        .best { background-color: #90EE90; }
        .worst { background-color: #FFB6C1; }
        .legend { margin: 20px 0; padding: 15px; background-color: #f8f9fa; border-radius: 5px; }
        .legend ul { margin: 10px 0; }
    </style>
</head>
<body>
    <h1>Family vs Family Performance Matrix</h1>
"#,
        );

        let table_content = generate_family_vs_family_comparison_table(data)?;
        content.push_str(&table_content);
        content.push_str("</body>\n</html>");
        
        Ok(content)
    }

    /// Generate LaTeX content for the report
    fn generate_latex(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> Result<String> {
        let mut content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage{multirow}
\usepackage{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage{rotating}
\usepackage[margin=0.5in]{geometry}
\usepackage{longtable}
\definecolor{bestgreen}{RGB}{0,150,0}
\definecolor{worstred}{RGB}{200,0,0}
\begin{document}

\title{Family vs Family Performance Matrix}
\author{QQN Optimizer Benchmark Suite}
\date{\today}
\maketitle

"#,
        );

        let table_content = generate_family_vs_family_table_content(data)?;
        content.push_str(&table_content);
        content.push_str("\n\\end{document}");
        
        Ok(content)
    }

    /// Generate Markdown content for the report
    fn generate_markdown(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> Result<String> {
        let mut content = String::from(
            r#"# Family vs Family Performance Matrix

This report shows how different optimizer families perform across different problem families.

"#,
        );

        // Generate a simplified markdown table since HTML tables in markdown are complex
        let mut all_optimizer_families = HashSet::new();
        let mut all_problem_families = HashSet::new();
        
        for (problem, results) in data {
            let problem_family = report_generator::get_family(&problem.get_name());
            all_problem_families.insert(problem_family);
            for result in &results.results {
                let optimizer_family = get_optimizer_family(&result.optimizer_name);
                all_optimizer_families.insert(optimizer_family);
            }
        }

        let mut optimizer_families: Vec<_> = all_optimizer_families.into_iter().collect();
        let mut problem_families: Vec<_> = all_problem_families.into_iter().collect();
        optimizer_families.sort();
        problem_families.sort();

        if optimizer_families.is_empty() || problem_families.is_empty() {
            content.push_str("*No data available for family comparison.*\n\n");
            return Ok(content);
        }

        // Create markdown table header
        content.push_str("| Problem Family |");
        for optimizer_family in &optimizer_families {
            content.push_str(&format!(" {} |", optimizer_family));
        }
        content.push_str("\n|");
        content.push_str(&"---|".repeat(optimizer_families.len() + 1));
        content.push_str("\n");

        // Generate table rows
        for problem_family in &problem_families {
            content.push_str(&format!("| **{}** |", problem_family));

            let problems_in_family: Vec<_> = data
                .iter()
                .filter(|(problem, _)| {
                    report_generator::get_family(&problem.get_name()) == *problem_family
                })
                .collect();

            for optimizer_family in &optimizer_families {
                let cell_data = calculate_family_performance_data(&problems_in_family, optimizer_family)?;
                
                let cell_content = if cell_data.average_ranking.is_finite() {
                    format!(" {:.1} / {:.1}<br/>Best: {}<br/>Worst: {} |", 
                        cell_data.average_ranking,
                        cell_data.best_rank_average,
                        cell_data.best_variant,
                        cell_data.worst_variant)
                } else {
                    " N/A |".to_string()
                };
                content.push_str(&cell_content);
            }
            content.push_str("\n");
        }

        content.push_str(r#"
## Legend

- **First value**: Average ranking of all variants in the optimizer family (lower is better)
- **Second value**: Average of the best rank achieved by any variant in the optimizer family
- **Best**: The specific optimizer variant that achieved the best average rank
- **Worst**: The specific optimizer variant that achieved the worst average rank

"#);

        Ok(content)
    }

    /// Generate CSV content for the report
    fn generate_csv(&self, data: &[(&ProblemSpec, BenchmarkResults)], _config: &ReportConfig) -> Result<String> {
        let mut content = String::from("Problem Family,Optimizer Family,Average Ranking,Best Rank Average,Best Variant,Worst Variant\n");

        let mut all_optimizer_families = HashSet::new();
        let mut all_problem_families = HashSet::new();
        
        for (problem, results) in data {
            let problem_family = report_generator::get_family(&problem.get_name());
            all_problem_families.insert(problem_family);
            for result in &results.results {
                let optimizer_family = get_optimizer_family(&result.optimizer_name);
                all_optimizer_families.insert(optimizer_family);
            }
        }

        let mut optimizer_families: Vec<_> = all_optimizer_families.into_iter().collect();
        let mut problem_families: Vec<_> = all_problem_families.into_iter().collect();
        optimizer_families.sort();
        problem_families.sort();

        for problem_family in &problem_families {
            let problems_in_family: Vec<_> = data
                .iter()
                .filter(|(problem, _)| {
                    report_generator::get_family(&problem.get_name()) == *problem_family
                })
                .collect();

            for optimizer_family in &optimizer_families {
                let cell_data = calculate_family_performance_data(&problems_in_family, optimizer_family)?;
                
                content.push_str(&format!(
                    "{},{},{:.3},{:.3},{},{}\n",
                    problem_family,
                    optimizer_family,
                    if cell_data.average_ranking.is_finite() { cell_data.average_ranking } else { f64::NAN },
                    if cell_data.best_rank_average.is_finite() { cell_data.best_rank_average } else { f64::NAN },
                    cell_data.best_variant,
                    cell_data.worst_variant
                ));
            }
        }

        Ok(content)
    }
}

impl Default for FamilyVsFamilyReport {
    fn default() -> Self {
        Self::new()
    }
}

impl Report for FamilyVsFamilyReport {
    fn name(&self) -> &'static str {
        "family_vs_family"
    }

    fn description(&self) -> &'static str {
        "Comparison matrix showing how different optimizer families perform across different problem families"
    }

    fn generate_content(&self, data: &[(&ProblemSpec, BenchmarkResults)], config: &ReportConfig) -> Result<String> {
        match config.format {
            ReportFormat::Html => self.generate_html(data, config),
            ReportFormat::Latex => self.generate_latex(data, config),
            ReportFormat::Markdown => self.generate_markdown(data, config),
            ReportFormat::Csv => self.generate_csv(data, config),
        }
    }

    fn export_to_file(&self, data: &[(&ProblemSpec, BenchmarkResults)], config: &ReportConfig, output_path: &Path) -> Result<()> {
        let content = self.generate_content(data, config)?;
        std::fs::write(output_path, content)?;
        Ok(())
    }

    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> Result<()> {
        if data.is_empty() {
            anyhow::bail!("No benchmark data provided");
        }

        // Check that we have results with optimizer names
        let has_results = data.iter().any(|(_, results)| !results.results.is_empty());
        if !has_results {
            anyhow::bail!("No benchmark results found in data");
        }

        // Check that we have at least one optimizer family and problem family
        let mut optimizer_families = HashSet::new();
        let mut problem_families = HashSet::new();
        
        for (problem, results) in data {
            let problem_family = report_generator::get_family(&problem.get_name());
            problem_families.insert(problem_family);
            for result in &results.results {
                let optimizer_family = get_optimizer_family(&result.optimizer_name);
                optimizer_families.insert(optimizer_family);
            }
        }

        if optimizer_families.is_empty() {
            anyhow::bail!("No optimizer families found in data");
        }

        if problem_families.is_empty() {
            anyhow::bail!("No problem families found in data");
        }

        Ok(())
    }

    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let mut optimizer_families = HashSet::new();
        let mut problem_families = HashSet::new();
        let mut total_data_points = 0;

        for (problem, results) in data {
            let problem_family = report_generator::get_family(&problem.get_name());
            problem_families.insert(problem_family);
            total_data_points += results.results.len();
            
            for result in &results.results {
                let optimizer_family = get_optimizer_family(&result.optimizer_name);
                optimizer_families.insert(optimizer_family);
            }
        }

        ReportMetadata {
            report_type: self.name().to_string(),
            generated_at: Default::default(),
            problem_count: problem_families.len(),
            optimizer_count: optimizer_families.len(),
            data_points: total_data_points,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::unified_report_tests::UnifiedReportTestSuite;

    #[test]
    fn test_family_vs_family_report_with_unified_suite() {
        let report = FamilyVsFamilyReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }

    #[test]
    fn test_family_vs_family_report_basic_functionality() {
        let report = FamilyVsFamilyReport::new();
        assert_eq!(report.name(), "family_vs_family");
        assert!(!report.description().is_empty());
        assert_eq!(report.supported_formats().len(), 4);
    }

    #[test]
    fn test_family_vs_family_report_content_generation() {
        let report = FamilyVsFamilyReport::new();
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        for format in &report.supported_formats() {
            let config = ReportConfig {
                format: format.clone(),
                ..Default::default()
            };
            
            let content = report.generate_content(&data_refs, &config).unwrap();
            assert!(!content.is_empty(), "Content should not be empty for format {:?}", format);

            // Format-specific validations
            match format {
                ReportFormat::Html => {
                    assert!(content.contains("<html>") && content.contains("</html>"));
                    assert!(content.contains("Family vs Family"));
                }
                ReportFormat::Latex => {
                    assert!(content.contains("\\documentclass"));
                    assert!(content.contains("\\begin{document}"));
                    assert!(content.contains("\\end{document}"));
                }
                ReportFormat::Markdown => {
                    assert!(content.contains("# Family vs Family"));
                    assert!(content.contains("|"));
                }
                ReportFormat::Csv => {
                    assert!(content.contains("Problem Family,Optimizer Family"));
                    assert!(content.contains(","));
                }
            }
        }
    }

    #[test]
    fn test_family_vs_family_report_validation() {
        let report = FamilyVsFamilyReport::new();
        
        // Test empty data
        let empty_data = vec![];
        assert!(report.validate_data(&empty_data).is_err());

        // Test valid data
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();
        assert!(report.validate_data(&data_refs).is_ok());

        // Test data with empty results
        let empty_results_data = UnifiedReportTestSuite::create_empty_results_data();
        let empty_refs: Vec<_> = empty_results_data.iter().map(|(p, r)| (p, r.clone())).collect();
        assert!(report.validate_data(&empty_refs).is_err());
    }

    #[test]
    fn test_family_vs_family_report_metadata() {
        let report = FamilyVsFamilyReport::new();
        let test_data = UnifiedReportTestSuite::create_test_data();
        let data_refs: Vec<_> = test_data.iter().map(|(p, r)| (p, r.clone())).collect();

        let metadata = report.get_metadata(&data_refs);
        assert_eq!(metadata.report_type, "family_vs_family");
        assert!(metadata.problem_count > 0);
        assert!(metadata.optimizer_count > 0);
        assert!(metadata.data_points > 0);
    }
}