use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::{
    report_generator, Report, ReportConfig, ReportFormat, ReportMetadata,
};
use anyhow::Context;
use html_escape::encode_text;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Success Rate Heatmap Report
pub struct SuccessRateHeatmapReport;
impl SuccessRateHeatmapReport {
    pub fn new() -> Self {
        Self
    }
}
impl Report for SuccessRateHeatmapReport {
    fn name(&self) -> &'static str {
        "success_rate_heatmap"
    }
    fn description(&self) -> &'static str {
        "Color-coded heatmap showing success rates across optimizer-problem combinations"
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
        // Ensure parent directories exist
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
                "Failed to write success rate heatmap report to: {}",
                output_path.display()
            )
        })?;
        Ok(())
    }
    fn validate_data(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> anyhow::Result<()> {
        if data.is_empty() {
            return Err(anyhow::anyhow!("No benchmark data provided"));
        }
        // Check that we have at least one optimizer result
        let has_results = data.iter().any(|(_, results)| !results.results.is_empty());
        if !has_results {
            return Err(anyhow::anyhow!(
                "No optimizer results found in benchmark data"
            ));
        }
        Ok(())
    }
    fn get_metadata(&self, data: &[(&ProblemSpec, BenchmarkResults)]) -> ReportMetadata {
        let mut all_optimizers = std::collections::HashSet::new();
        let problem_count = data.len();
        let mut total_data_points = 0;

        for (_, results) in data {
            for result in &results.results {
                all_optimizers.insert(result.optimizer_name.clone());
                total_data_points += 1;
            }
        }
        let mut metadata = HashMap::new();
        metadata.insert(
            "optimizer_count".to_string(),
            all_optimizers.len().to_string(),
        );
        metadata.insert("problem_count".to_string(), problem_count.to_string());
        metadata.insert("report_type".to_string(), "heatmap".to_string());
        ReportMetadata {
            report_type: "success_rate_heatmap".to_string(),
            generated_at: Default::default(),
            problem_count,
            optimizer_count: all_optimizers.len(),
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
impl SuccessRateHeatmapReport {
    fn generate_html(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizers, _) = self.collect_optimizers_and_problems(data);
        if optimizers.is_empty() {
            return Ok("<p>No data available for heatmap generation.</p>".to_string());
        }
        let mut html = String::from(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Success Rate Heatmap</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        table { border-collapse: collapse; width: 100%; margin: 20px 0; }
        th, td { border: 1px solid #ddd; padding: 8px; text-align: center; }
        th { background-color: #f2f2f2; font-weight: bold; }
        .excellent { background-color: #90EE90; color: black; }
        .good { background-color: #FFFF99; color: black; }
        .poor { background-color: #FFB366; color: black; }
        .very-poor { background-color: #FF6B6B; color: white; }
        .no-data { background-color: #D3D3D3; color: white; }
        .legend { margin: 20px 0; }
        .legend-item { display: inline-block; margin: 5px; padding: 5px 10px; border: 1px solid #ccc; }
    </style>
</head>
<body>
    <h1>Success Rate Heatmap</h1>
    <p>Color-coded success rates across all optimizer-problem combinations</p>
    <table>
        <thead>
            <tr>
                <th>Problem</th>"#,
        );
        for optimizer in &optimizers {
            html.push_str(&format!("<th>{}</th>", encode_text(optimizer)));
        }
        html.push_str("</tr></thead><tbody>");
        for (problem, results) in data {
            let problem_name = problem.get_name();
            html.push_str(&format!(
                "<tr><td><strong>{}</strong></td>",
                encode_text(&problem_name)
            ));
            for optimizer in &optimizers {
                let (success_rate, has_data) = self.calculate_success_rate(results, optimizer);
                let (class, display_text) = self.get_html_cell_style(success_rate, has_data);
                html.push_str(&format!("<td class=\"{}\">{}</td>", class, display_text));
            }
            html.push_str("</tr>");
        }
        html.push_str(
            r#"</tbody></table>
    <div class="legend">
        <strong>Legend:</strong>
        <span class="legend-item excellent">90-100% Excellent</span>
        <span class="legend-item good">50-89% Good</span>
        <span class="legend-item poor">10-49% Poor</span>
        <span class="legend-item very-poor">0-9% Very Poor</span>
        <span class="legend-item no-data">N/A No Data</span>
    </div>
</body>
</html>"#,
        );
        Ok(html)
    }
    fn generate_latex(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizers, _) = self.collect_optimizers_and_problems(data);
        if optimizers.is_empty() {
            return Ok(String::new());
        }
        let col_spec = format!("l{}", "c".repeat(optimizers.len()));
        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage[table]{xcolor}
\usepackage{adjustbox}
\usepackage{rotating}
\usepackage[margin=1in]{geometry}
\begin{document}
"#,
        );
        latex_content.push_str(&format!(
            r#"\begin{{table}}[htbp]
\centering
\caption{{Success Rate Heatmap: Color-coded Success Rates Across All Optimizer-Problem Combinations}}
\label{{tab:success_rate_heatmap}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{}}}
\toprule
\textbf{{Problem}} {}\\
\midrule
"#,
            col_spec,
            optimizers
                .iter()
                .map(|opt| format!("& \\rotatebox{{90}}{{\\textbf{{{}}}}}", report_generator::escape_latex(opt)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        for (problem, results) in data {
            let problem_name = problem.get_name();
            latex_content.push_str(&format!(
                "\\textbf{{{}}} ",
                report_generator::escape_latex(&problem_name)
            ));
            for optimizer in &optimizers {
                let (success_rate, has_data) = self.calculate_success_rate(results, optimizer);
                let cell_content = self.get_latex_cell_content(success_rate, has_data);
                latex_content.push_str(&cell_content);
            }
            latex_content.push_str(" \\\\\n");
        }
        latex_content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Legend:}
\colorbox{green!70}{90-100\%} Excellent,
\colorbox{yellow!70}{50-89\%} Good,
\colorbox{orange!70}{10-49\%} Poor,
\colorbox{red!70}{\textcolor{white}{0-9\%}} Very Poor,
\colorbox{gray!30}{\textcolor{white}{N/A}} No Data.
Quickly identifies which optimizers work on which problem types.
\end{document}
"#,
        );
        Ok(latex_content)
    }
    fn generate_markdown(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizers, _) = self.collect_optimizers_and_problems(data);
        if optimizers.is_empty() {
            return Ok("No data available for heatmap generation.".to_string());
        }
        let mut markdown = String::from("# Success Rate Heatmap\n\nColor-coded success rates across all optimizer-problem combinations\n\n");
        // Table header
        markdown.push_str("| Problem |");
        for optimizer in &optimizers {
            markdown.push_str(&format!(" {} |", optimizer));
        }
        markdown.push('\n');
        // Table separator
        markdown.push_str("|---------|");
        for _ in &optimizers {
            markdown.push_str("---------|");
        }
        markdown.push('\n');
        // Table rows
        for (problem, results) in data {
            let problem_name = problem.get_name();
            markdown.push_str(&format!("| **{}** |", problem_name));
            for optimizer in &optimizers {
                let (success_rate, has_data) = self.calculate_success_rate(results, optimizer);
                let display_text = if has_data {
                    format!("{:.0}%", success_rate)
                } else {
                    "N/A".to_string()
                };
                markdown.push_str(&format!(" {} |", display_text));
            }
            markdown.push('\n');
        }
        markdown.push_str("\n**Legend:**\n");
        markdown.push_str("- 90-100%: Excellent\n");
        markdown.push_str("- 50-89%: Good\n");
        markdown.push_str("- 10-49%: Poor\n");
        markdown.push_str("- 0-9%: Very Poor\n");
        markdown.push_str("- N/A: No Data\n");
        Ok(markdown)
    }
    fn generate_csv(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
        _config: &ReportConfig,
    ) -> anyhow::Result<String> {
        let (optimizers, _) = self.collect_optimizers_and_problems(data);
        if optimizers.is_empty() {
            return Ok(
                "Problem,Message\nNo Data,No data available for heatmap generation".to_string(),
            );
        }
        let mut csv = String::from("Problem");
        for optimizer in &optimizers {
            csv.push_str(&format!(",{}", optimizer));
        }
        csv.push('\n');
        for (problem, results) in data {
            let problem_name = problem.get_name();
            csv.push_str(&problem_name);
            for optimizer in &optimizers {
                let (success_rate, has_data) = self.calculate_success_rate(results, optimizer);
                let value = if has_data {
                    format!("{:.1}", success_rate)
                } else {
                    "N/A".to_string()
                };
                csv.push_str(&format!(",{}", value));
            }
            csv.push('\n');
        }
        Ok(csv)
    }
    fn collect_optimizers_and_problems(
        &self,
        data: &[(&ProblemSpec, BenchmarkResults)],
    ) -> (Vec<String>, Vec<String>) {
        let mut all_optimizers = std::collections::HashSet::new();
        let mut all_problems = Vec::new();
        for (problem, results) in data {
            all_problems.push(problem.get_name());
            for result in &results.results {
                all_optimizers.insert(result.optimizer_name.clone());
            }
        }
        let mut optimizers: Vec<_> = all_optimizers.into_iter().collect();
        optimizers.sort();
        (optimizers, all_problems)
    }
    fn calculate_success_rate(&self, results: &BenchmarkResults, optimizer: &str) -> (f64, bool) {
        let optimizer_results: Vec<_> = results
            .results
            .iter()
            .filter(|r| r.optimizer_name == optimizer)
            .collect();
        if optimizer_results.is_empty() {
            (0.0, false)
        } else {
            let successful = optimizer_results
                .iter()
                .filter(|r| r.convergence_achieved)
                .count();
            let success_rate = successful as f64 / optimizer_results.len() as f64 * 100.0;
            (success_rate, true)
        }
    }
    fn get_html_cell_style(&self, success_rate: f64, has_data: bool) -> (&'static str, String) {
        if !has_data {
            ("no-data", "N/A".to_string())
        } else if success_rate >= 90.0 {
            ("excellent", format!("{:.0}%", success_rate))
        } else if success_rate >= 50.0 {
            ("good", format!("{:.0}%", success_rate))
        } else if success_rate >= 10.0 {
            ("poor", format!("{:.0}%", success_rate))
        } else {
            ("very-poor", format!("{:.0}%", success_rate))
        }
    }
    fn get_latex_cell_content(&self, success_rate: f64, has_data: bool) -> String {
        if !has_data {
            "& \\cellcolor{gray!30}\\textcolor{white}{N/A}".to_string()
        } else {
            let (color, text_color) = if success_rate >= 90.0 {
                ("green!70", "black")
            } else if success_rate >= 50.0 {
                ("yellow!70", "black")
            } else if success_rate >= 10.0 {
                ("orange!70", "black")
            } else {
                ("red!70", "white")
            };
            format!("& \\cellcolor{{{color}}}\\textcolor{{{text_color}}}{{{success_rate:.0}\\%}}")
        }
    }
}

/// Generate success rate heatmap table content (without document wrapper)
pub fn generate_success_rate_heatmap_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    let report = SuccessRateHeatmapReport::new();
    let (optimizers, all_problems) = report.collect_optimizers_and_problems(all_results);

    if optimizers.is_empty() || all_problems.is_empty() {
        return Ok(String::new());
    }

    let mut content = format!(
        r#"\begin{{table}}[H]
\centering
\caption{{Success Rate Heatmap: Color-coded Success Rates Across All Optimizer-Problem Combinations}}
\label{{tab:success_rate_heatmap}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{l{}}}
\toprule
\textbf{{Problem}} {}\\
\midrule
"#,
        "c".repeat(optimizers.len()),
        optimizers
            .iter()
            .map(|opt| format!(
                "& \\rotatebox{{90}}{{\\textbf{{{}}}}}",
                report_generator::escape_latex(opt)
            ))
            .collect::<Vec<_>>()
            .join(" ")
    );

    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(&problem_name)
        ));
        for optimizer in &optimizers {
            let (success_rate, has_data) = report.calculate_success_rate(results, optimizer);
            let cell_content = report.get_latex_cell_content(success_rate, has_data);
            content.push_str(&cell_content);
        }
        content.push_str(" \\\\\n");
    }
    content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Legend:}
\colorbox{green!70}{90-100\%} Excellent,
\colorbox{yellow!70}{50-89\%} Good,
\colorbox{orange!70}{10-49\%} Poor,
\colorbox{red!70}{\textcolor{white}{0-9\%}} Very Poor,
\colorbox{gray!30}{\textcolor{white}{N/A}} No Data.
Quickly identifies which optimizers work on which problem types.
"#,
    );
    Ok(content)
}

/// Generate success rate heatmap LaTeX table
pub fn generate_success_rate_heatmap_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    let report = SuccessRateHeatmapReport::new();
    let (optimizers, all_problems) = report.collect_optimizers_and_problems(all_results);

    if optimizers.is_empty() || all_problems.is_empty() {
        return Ok(());
    }

    let col_spec = format!("l{}", "c".repeat(optimizers.len()));

    let mut latex_content = String::from(
        r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage[table]{xcolor}
\usepackage{adjustbox}
\usepackage{rotating}
\usepackage[margin=1in]{geometry}
\begin{document}
"#,
    );
    latex_content.push_str(&format!(
        r#"\begin{{table}}[htbp]
\centering
\caption{{Success Rate Heatmap: Color-coded Success Rates Across All Optimizer-Problem Combinations}}
\label{{tab:success_rate_heatmap}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{}}}
\toprule
\textbf{{Problem}} {}\\
\midrule
"#,
        col_spec,
        optimizers
            .iter()
            .map(|opt| format!("& \\rotatebox{{90}}{{\\textbf{{{}}}}}", report_generator::escape_latex(opt)))
            .collect::<Vec<_>>()
            .join(" ")
    ));

    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        latex_content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(&problem_name)
        ));
        for optimizer in &optimizers {
            let (success_rate, has_data) = report.calculate_success_rate(results, optimizer);
            let cell_content = report.get_latex_cell_content(success_rate, has_data);
            latex_content.push_str(&cell_content);
        }
        latex_content.push_str(" \\\\\n");
    }
    latex_content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Legend:}
\colorbox{green!70}{90-100\%} Excellent,
\colorbox{yellow!70}{50-89\%} Good,
\colorbox{orange!70}{10-49\%} Poor,
\colorbox{red!70}{\textcolor{white}{0-9\%}} Very Poor,
\colorbox{gray!30}{\textcolor{white}{N/A}} No Data.
Quickly identifies which optimizers work on which problem types.
\end{document}
"#,
    );
    // Ensure parent directories exist
    if let Some(parent) = latex_dir.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "Failed to create parent directories for: {}",
                latex_dir.display()
            )
        })?;
    }
    let latex_path = latex_dir.join("success_rate_heatmap.tex");
    fs::write(&latex_path, latex_content).with_context(|| {
        format!(
            "Failed to write success rate heatmap LaTeX table to: {}",
            latex_path.display()
        )
    })?;
    println!(
        "Generated success rate heatmap LaTeX table: {}",
        latex_path.display()
    );
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::experiment_runner::UnifiedReportTestSuite;
    #[test]
    fn test_success_rate_heatmap_report() {
        let report = SuccessRateHeatmapReport::new();
        UnifiedReportTestSuite::test_report(&report).unwrap();
    }
}
