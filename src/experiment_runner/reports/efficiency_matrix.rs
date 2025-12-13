use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::{
    report_generator, Report, ReportConfig, ReportFormat, ReportMetadata,
};
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
/// Efficiency Matrix Report
///
/// Shows mean function evaluations ± standard deviation for successful runs only across problem families.
/// Lower values indicate higher efficiency.
pub struct EfficiencyMatrixReport;

impl EfficiencyMatrixReport {
    pub fn new() -> Self {
        Self
    }

    fn collect_families(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> (Vec<String>, Vec<String>) {
        let mut all_optimizer_families = std::collections::HashSet::new();
        let mut all_problem_families = std::collections::HashSet::new();

        for (problem, results) in all_results {
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

        (optimizer_families, problem_families)
    }

    fn calculate_efficiency_data(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> HashMap<(String, String), (f32, f32, usize)> {
        let mut efficiency_data = HashMap::new();
        let (optimizer_families, problem_families) = self.collect_families(all_results);

        for optimizer_family in &optimizer_families {
            for problem_family in &problem_families {
                let mut successful_evaluations = Vec::new();

                for (problem, results) in all_results {
                    if report_generator::get_family(&problem.get_name()) == *problem_family {
                        for result in &results.results {
                            let result_optimizer_family =
                                get_optimizer_family(&result.optimizer_name);
                            if result_optimizer_family == *optimizer_family
                                && result.convergence_achieved
                            {
                                successful_evaluations.push(result.function_evaluations as f32);
                            }
                        }
                    }
                }

                if !successful_evaluations.is_empty() {
                    let mean = successful_evaluations.iter().sum::<f32>()
                        / successful_evaluations.len() as f32;
                    let variance = successful_evaluations
                        .iter()
                        .map(|x| (x - mean).powi(2))
                        .sum::<f32>()
                        / successful_evaluations.len() as f32;
                    let std_dev = variance.sqrt();
                    efficiency_data.insert(
                        (optimizer_family.clone(), problem_family.clone()),
                        (mean, std_dev, successful_evaluations.len()),
                    );
                }
            }
        }

        efficiency_data
    }

    fn generate_html(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizer_families, problem_families) = self.collect_families(all_results);
        let efficiency_data = self.calculate_efficiency_data(all_results);

        if optimizer_families.is_empty() || problem_families.is_empty() {
            return Ok("<p>No data available for efficiency matrix.</p>".to_string());
        }

        let mut html_content = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Algorithm Efficiency Matrix</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: center; }
        th { background-color: #f2f2f2; font-weight: bold; }
        .problem-name { text-align: left; font-weight: bold; }
        .na-cell { color: #999; font-style: italic; }
        .description { margin: 20px 0; font-style: italic; }
    </style>
</head>
<body>
    <h1>Algorithm Efficiency Matrix</h1>
    <p class="description">Mean Function Evaluations for Successful Runs</p>
    <table>
        <thead>
            <tr>
                <th>Problem Family</th>"#,
        );

        for optimizer_family in &optimizer_families {
            html_content.push_str(&format!("<th>{}</th>", optimizer_family));
        }
        html_content.push_str("</tr></thead><tbody>");

        for problem_family in &problem_families {
            html_content.push_str(&format!(
                "<tr><td class=\"problem-name\">{}</td>",
                problem_family
            ));

            for optimizer_family in &optimizer_families {
                let cell_content = if let Some((mean, std_dev, _count)) =
                    efficiency_data.get(&(optimizer_family.clone(), problem_family.clone()))
                {
                    format!("{:.0} ± {:.0}", mean, std_dev)
                } else {
                    "<span class=\"na-cell\">N/A</span>".to_string()
                };
                html_content.push_str(&format!("<td>{}</td>", cell_content));
            }
            html_content.push_str("</tr>");
        }

        html_content.push_str(
            r#"</tbody>
    </table>
    <p><strong>Purpose:</strong> Shows mean function evaluations ± standard deviation for successful runs only across problem families. Lower values indicate higher efficiency.</p>
</body>
</html>"#
        );

        Ok(html_content)
    }

    fn generate_latex(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizer_families, problem_families) = self.collect_families(all_results);
        let efficiency_data = self.calculate_efficiency_data(all_results);

        if optimizer_families.is_empty() || problem_families.is_empty() {
            return Ok("No data available for efficiency matrix.".to_string());
        }

        let col_spec = format!("l{}", "c".repeat(optimizer_families.len()));

        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage[margin=1in]{geometry}
\begin{document}
"#,
        );

        latex_content.push_str(&format!(
            r#"\begin{{table}}[htbp]
\centering
\caption{{Algorithm Efficiency Matrix: Mean Function Evaluations for Successful Runs}}
\label{{tab:efficiency_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{col_spec}}}
\toprule
\textbf{{Problem Family}} {}\\
\midrule
"#,
            optimizer_families
                .iter()
                .map(|fam| format!("& \\textbf{{{}}}", report_generator::escape_latex(fam)))
                .collect::<Vec<_>>()
                .join(" ")
        ));

        for problem_family in &problem_families {
            latex_content.push_str(&format!(
                "\\textbf{{{}}} ",
                report_generator::escape_latex(problem_family)
            ));

            for optimizer_family in &optimizer_families {
                let cell_content = if let Some((mean, std_dev, _count)) =
                    efficiency_data.get(&(optimizer_family.clone(), problem_family.clone()))
                {
                    format!("{:.0} $\\pm$ {:.0}", mean, std_dev)
                } else {
                    "N/A".to_string()
                };
                latex_content.push_str(&format!("& {} ", cell_content));
            }
            latex_content.push_str("\\\\\n");
        }

        latex_content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Shows mean function evaluations $\pm$ standard deviation for successful runs only across problem families. Lower values indicate higher efficiency.
\end{document}
"#,
        );

        Ok(latex_content)
    }

    fn generate_markdown(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizer_families, problem_families) = self.collect_families(all_results);
        let efficiency_data = self.calculate_efficiency_data(all_results);

        if optimizer_families.is_empty() || problem_families.is_empty() {
            return Ok("No data available for efficiency matrix.".to_string());
        }

        let mut markdown_content = String::from("# Algorithm Efficiency Matrix\n\n");
        markdown_content.push_str("Mean Function Evaluations for Successful Runs\n\n");

        // Table header
        markdown_content.push_str("| Problem Family |");
        for optimizer_family in &optimizer_families {
            markdown_content.push_str(&format!(" {} |", optimizer_family));
        }
        markdown_content.push_str("\n|");

        // Table separator
        markdown_content.push_str("---|");
        for _ in &optimizer_families {
            markdown_content.push_str("---|");
        }
        markdown_content.push_str("\n");

        // Table rows
        for problem_family in &problem_families {
            markdown_content.push_str(&format!("| **{}** |", problem_family));

            for optimizer_family in &optimizer_families {
                let cell_content = if let Some((mean, std_dev, _count)) =
                    efficiency_data.get(&(optimizer_family.clone(), problem_family.clone()))
                {
                    format!("{:.0} ± {:.0}", mean, std_dev)
                } else {
                    "N/A".to_string()
                };
                markdown_content.push_str(&format!(" {} |", cell_content));
            }
            markdown_content.push_str("\n");
        }

        markdown_content.push_str("\n**Purpose:** Shows mean function evaluations ± standard deviation for successful runs only across problem families. Lower values indicate higher efficiency.\n");

        Ok(markdown_content)
    }

    fn generate_csv(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizer_families, problem_families) = self.collect_families(all_results);
        let efficiency_data = self.calculate_efficiency_data(all_results);

        if optimizer_families.is_empty() || problem_families.is_empty() {
            return Ok("No data available for efficiency matrix.".to_string());
        }

        let mut csv_content = String::from("Problem Family");
        for optimizer_family in &optimizer_families {
            csv_content.push_str(&format!(",{}", optimizer_family));
        }
        csv_content.push_str("\n");

        for problem_family in &problem_families {
            csv_content.push_str(problem_family);

            for optimizer_family in &optimizer_families {
                let cell_content = if let Some((mean, std_dev, _count)) =
                    efficiency_data.get(&(optimizer_family.clone(), problem_family.clone()))
                {
                    format!("{:.0} ± {:.0}", mean, std_dev)
                } else {
                    "N/A".to_string()
                };
                csv_content.push_str(&format!(",{}", cell_content));
            }
            csv_content.push_str("\n");
        }

        Ok(csv_content)
    }
}

impl Report for EfficiencyMatrixReport {
    fn name(&self) -> &'static str {
        "efficiency_matrix"
    }

    fn description(&self) -> &'static str {
        "Algorithm Efficiency Matrix showing mean function evaluations for successful runs across problem families"
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
                "Failed to write efficiency matrix report to: {}",
                output_path.display()
            )
        })?;
        Ok(())
    }

    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> anyhow::Result<()> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("No benchmark data provided"));
        }

        // Check that we have at least some results with convergence data
        let has_convergence_data = data
            .iter()
            .any(|(_, results)| results.results.iter().any(|r| r.convergence_achieved));

        if !has_convergence_data {
            return Err(anyhow::anyhow!(
                "No convergence data found in benchmark results"
            ));
        }

        Ok(())
    }

    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let (optimizer_families, problem_families) = self.collect_families(data);
        let efficiency_data = self.calculate_efficiency_data(data);

        let total_successful_runs: usize =
            efficiency_data.values().map(|(_, _, count)| count).sum();

        let mut metadata = HashMap::new();
        metadata.insert(
            "optimizer_families".to_string(),
            optimizer_families.len().to_string(),
        );
        metadata.insert(
            "problem_families".to_string(),
            problem_families.len().to_string(),
        );
        metadata.insert(
            "total_successful_runs".to_string(),
            total_successful_runs.to_string(),
        );
        metadata.insert(
            "matrix_cells".to_string(),
            (optimizer_families.len() * problem_families.len()).to_string(),
        );

        ReportMetadata {
            report_type: self.name().to_string(),
            generated_at: chrono::Utc::now(),
            problem_count: data.len(),
            optimizer_count: data
                .iter()
                .flat_map(|(_, results)| &results.results)
                .map(|r| &r.optimizer_name)
                .collect::<std::collections::HashSet<_>>()
                .len(),
            data_points: data.iter().map(|(_, results)| results.results.len()).sum(),
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

/// Legacy function for backward compatibility
/// Generate efficiency matrix LaTeX table
pub fn generate_efficiency_matrix_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    let report = EfficiencyMatrixReport::new();
    let config = ReportConfig {
        format: ReportFormat::Latex,
        ..Default::default()
    };

    let latex_path = latex_dir.join("efficiency_matrix.tex");
    if let Some(parent) = latex_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create parent directories for: {}",
                latex_path.display()
            )
        })?;
    }
    report.export_to_file(all_results, &config, &latex_path)?;

    println!(
        "Generated efficiency matrix LaTeX table: {}",
        latex_path.display()
    );

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::UnifiedReportTestSuite;
    #[test]
    fn test_efficiency_matrix_report() {
        let report = EfficiencyMatrixReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }
    #[test]
    fn test_efficiency_matrix_basic_functionality() {
        let report = EfficiencyMatrixReport::new();
        UnifiedReportTestSuite::test_basic_functionality(&report).unwrap();
    }
    #[test]
    fn test_efficiency_matrix_content_generation() {
        let report = EfficiencyMatrixReport::new();
        UnifiedReportTestSuite::test_content_generation(&report).unwrap();
    }
}
