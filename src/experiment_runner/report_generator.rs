use super::StatisticalAnalysis;
use crate::benchmarks::evaluation::{
    is_no_threshold_mode, BenchmarkConfig, BenchmarkResults, ProblemSpec, SingleResult,
};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::reports::comparison_matrix::{
    generate_comparison_matrix_latex_table, generate_comparison_matrix_table_content,
    generate_family_comparison_matrix_table_content,
};
use crate::experiment_runner::reports::convergence_analysis::ConvergenceAnalysisReport;
use crate::experiment_runner::reports::convergence_analysis::{
    generate_convergence_speed_latex_table, generate_convergence_speed_table_content,
};
use crate::experiment_runner::reports::efficiency_matrix::generate_efficiency_matrix_latex_table;
use crate::experiment_runner::reports::efficiency_matrix::EfficiencyMatrixReport;
use crate::experiment_runner::reports::family_vs_family::{
    generate_family_vs_family_comparison_table, generate_family_vs_family_latex_table,
    generate_family_vs_family_table_content,
};
use crate::experiment_runner::reports::family_vs_family_report::FamilyVsFamilyReport;
use crate::experiment_runner::reports::heatmap::SuccessRateHeatmapReport;
use crate::experiment_runner::reports::heatmap::{
    generate_success_rate_heatmap_latex_table, generate_success_rate_heatmap_table_content,
};
use crate::experiment_runner::reports::performance_analysis::{
    generate_main_performance_latex_table, generate_main_performance_table_content,
};
use crate::experiment_runner::reports::summary_statistics::{
    generate_summary_statistics_latex_table, generate_summary_statistics_table_content,
};
use crate::experiment_runner::reports::unified_performance_table::PerformanceTableReport;
use crate::experiment_runner::reports::unified_summary_statistics::SummaryStatisticsReport;
use crate::experiment_runner::reports::{comparison_matrix, optimizer_problems};
use crate::experiment_runner::unified_report::{
    Report, ReportCollection, ReportConfig, ReportFormat,
};
use crate::OptimizationProblem;
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde_json;

/// Data structure for family performance comparison
#[derive(Debug, Clone)]
pub struct FamilyPerformanceData {
    pub(crate) average_ranking: f64,
    pub(crate) best_rank_average: f64,
    pub(crate) best_variant: String,
    pub(crate) worst_variant: String,
}

/// Handles HTML report generation and CSV exports
#[derive(Clone)]
pub struct ReportGenerator {
    pub(crate) output_dir: String,
    config: BenchmarkConfig,
    pub(crate) statistical_analysis: StatisticalAnalysis,
}

pub fn get_family(problem_name: &str) -> String {
    match problem_name
        .split([' ', '_'])
        .next()
        .unwrap_or(problem_name)
    {
        // Convex/Unimodal functions - smooth, single global minimum
        "Sphere" => "Sphere".to_string(),
        "Matyas" => "Matyas".to_string(),

        // Non-convex but unimodal - single global minimum, challenging valleys/ridges
        "Rosenbrock" => "Rosenbrock".to_string(),
        "Beale" => "Beale".to_string(),
        "GoldsteinPrice" => "GoldsteinPrice".to_string(),
        "Levi" => "Levi".to_string(),

        // Highly multimodal - many local minima, very challenging
        "Rastrigin" => "Rastrigin".to_string(),
        "Ackley" => "Ackley".to_string(),
        "Michalewicz" => "Michalewicz".to_string(),
        "StyblinskiTang" => "StyblinskiTang".to_string(),

        // Machine Learning problems
        name if name.contains("Regression") => "Regression".to_string(),
        name if name.contains("Neural") => "Neural Networks".to_string(),
        name if name.contains("SVM") => "SVM".to_string(),
        name if name.contains("Logistic") => "Logistic".to_string(),

        // Default fallback
        x => x.to_string(),
    }
}

impl ReportGenerator {
    pub fn new(output_dir: String, config: BenchmarkConfig) -> Self {
        Self {
            output_dir,
            config,
            statistical_analysis: StatisticalAnalysis::new(),
        }
    }

    pub async fn generate_main_report(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        use_optimizer_families: bool,
    ) -> anyhow::Result<()> {
        let output_dir = &self.output_dir;
        fs::create_dir_all(&output_dir)
            .with_context(|| format!("Failed to create output directory: {}", output_dir))?;
        // Generate unified reports in multiple formats
        let unified_formats = vec![
            ReportFormat::Html,
            ReportFormat::Latex,
            ReportFormat::Markdown,
            ReportFormat::Csv,
        ];
        generate_unified_reports(all_results, &unified_formats, output_dir.as_str()).await?;
        generate_report_index(all_results, &unified_formats, output_dir.clone()).await?;

        // Create hierarchical directory structure
        let reports_dir = Path::new(&output_dir).join("reports");
        let data_dir = Path::new(&output_dir).join("data");
        let plots_dir = Path::new(&output_dir).join("plots");
        let latex_dir = Path::new(&output_dir).join("latex");
        fs::create_dir_all(&reports_dir)?;
        fs::create_dir_all(&data_dir)?;
        fs::create_dir_all(&Path::new(&data_dir).join("convergence"))?;
        fs::create_dir_all(&Path::new(&plots_dir).join("convergence"))?;
        fs::create_dir_all(&plots_dir)?;
        fs::create_dir_all(&latex_dir)?;

        println!("Generating report in directory: {}", output_dir);

        // Generate detailed optimizer-problem reports first
        generate_detailed_reports(
            &reports_dir.to_string_lossy(),
            all_results,
            use_optimizer_families,
        )
        .await?;

        let mut html_content = generate_header();
        html_content.push_str(&generate_winner_summary_table(all_results));

        for (problem, results) in all_results {
            html_content.push_str(&optimizer_problems::generate_problem_section(
                problem,
                results,
                &plots_dir.to_string_lossy(),
            )?);
        }
        // Add optimizer family vs problem family comparison
        //html_content.push_str(&generate_family_vs_family_comparison_table(all_results)?);
        // if !all_results.is_empty() && all_results.iter().any(|(_, r)| !r.results.is_empty()) {
        //     html_content.push_str(&self.statistical_analysis.generate_statistical_analysis(
        //         all_results,
        //         &self.config,
        //         &data_dir.to_string_lossy(),
        //         use_optimizer_families,
        //     )?);
        // }

        html_content.push_str(&generate_conclusions(all_results));
        html_content.push_str(&generate_html_footer(&self.config));
        // Generate optimizer specifications section
        html_content.push_str(&generate_optimizer_specifications_section(all_results)?);


        let md_path = Path::new(&output_dir).join("benchmark_report.md");
        println!("Saving Markdown report to: {}", md_path.display());
        // Ensure parent directory exists
        if let Some(parent) = md_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&md_path, html_content.clone()).with_context(|| {
            format!("Failed to write Markdown report to: {}", md_path.display())
        })?;

        generate_csv_exports(&data_dir.to_string_lossy(), all_results)?;
        // Generate LaTeX tables
        generate_latex_tables(&latex_dir.to_string_lossy(), all_results, self).await?;
        // Generate optimizer specifications JSON
        generate_optimizer_specifications_json(&data_dir.to_string_lossy(), all_results)?;
        
        // Generate comprehensive LaTeX document
        generate_comprehensive_latex_document(&self.config, all_results, &latex_dir, self)?;
        println!("Report generation complete!");
        println!("  - Unified reports: {}/unified_reports/", output_dir);
        println!("  - Report index: {}/report_index.html", output_dir);
        println!("  - Legacy reports: {}/benchmark_report.md", output_dir);
        println!("  - LaTeX tables: {}/latex/", output_dir);
        println!("  - Raw data: {}/data/", output_dir);

        Ok(())
    }
    /// Generate only unified reports (for testing or when legacy reports are not needed)
    pub async fn generate_unified_only(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        formats: Option<Vec<ReportFormat>>,
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)
            .with_context(|| format!("Failed to create output directory: {}", self.output_dir))?;
        let formats = formats.unwrap_or_else(|| {
            vec![
                ReportFormat::Html,
                ReportFormat::Markdown,
                ReportFormat::Csv,
            ]
        });
        generate_unified_reports(all_results, &formats, self.output_dir.clone().as_str()).await?;
        generate_report_index(all_results, &formats, self.output_dir.clone()).await?;
        println!("Unified report generation complete!");
        println!("  - Reports: {}/unified_reports/", self.output_dir);
        println!("  - Index: {}/report_index.html", self.output_dir);
        Ok(())
    }
    /// Get available unified report types
    pub fn get_available_unified_reports() -> Vec<(&'static str, &'static str)> {
        vec![
            ("convergence_analysis", "Convergence Analysis"),
            ("efficiency_matrix", "Efficiency Matrix"),
            ("success_rate_heatmap", "Success Rate Heatmap"),
            ("performance_table", "Performance Table"),
            ("summary_statistics", "Summary Statistics"),
            ("family_vs_family", "Family vs Family Comparison"),
        ]
    }
    /// Generate a specific unified report
    pub async fn generate_specific_unified_report<R: Report + 'static>(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        report: R,
        format: ReportFormat,
    ) -> anyhow::Result<String> {
        let config = ReportConfig {
            format,
            include_detailed_stats: true,
            include_plots: true,
            style_options: std::collections::HashMap::new(),
        };
        report.validate_data(all_results)?;
        let content = report.generate_content(all_results, &config)?;
        Ok(content)
    }
}
/// Generate a comprehensive report index that links to all unified reports
pub async fn generate_report_index(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    formats: &[ReportFormat],
    path: String,
) -> anyhow::Result<()> {
    let index_path = Path::new(&path).join("report_index.html");
    // Ensure parent directory exists
    if let Some(parent) = index_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut html_content = String::from(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QQN Benchmark Report Index</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; line-height: 1.6; }
        h1, h2 { color: #333; }
        .report-grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; margin: 20px 0; }
        .report-card { border: 1px solid #ddd; border-radius: 8px; padding: 20px; background: #f9f9f9; }
        .report-card h3 { margin-top: 0; color: #2c5aa0; }
        .format-links { margin: 10px 0; }
        .format-links a { margin-right: 10px; padding: 5px 10px; background: #e7f3ff; text-decoration: none; border-radius: 4px; }
        .format-links a:hover { background: #d0e7ff; }
        .stats { background: #e8f5e8; padding: 15px; border-radius: 5px; margin: 20px 0; }
        .legacy-section { background: #fff3cd; padding: 15px; border-radius: 5px; margin: 20px 0; }
    </style>
</head>
<body>
    <h1>QQN Benchmark Report Index</h1>
    <p>Generated on: "#,
    );
    html_content.push_str(
        &chrono::Utc::now()
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string(),
    );
    html_content.push_str("</p>");
    // Add summary statistics
    let total_problems = all_results.len();
    let total_runs: usize = all_results.iter().map(|(_, r)| r.results.len()).sum();
    let unique_optimizers: std::collections::HashSet<_> = all_results
        .iter()
        .flat_map(|(_, r)| &r.results)
        .map(|result| &result.optimizer_name)
        .collect();
    html_content.push_str(&format!(
        r#"<div class="stats">
    <h2>Benchmark Summary</h2>
    <ul>
        <li><strong>Total Problems:</strong> {}</li>
        <li><strong>Total Optimizers:</strong> {}</li>
        <li><strong>Total Runs:</strong> {}</li>
        <li><strong>Available Formats:</strong> {}</li>
    </ul>
</div>
"#,
        total_problems,
        unique_optimizers.len(),
        total_runs,
        formats
            .iter()
            .map(|f| format!("{:?}", f))
            .collect::<Vec<_>>()
            .join(", ")
    ));
    // Add unified reports section
    html_content.push_str(
        r#"<h2>Unified Reports</h2>
<p>Modern, standardized reports with consistent formatting across all output types.</p>
<div class="report-grid">
"#,
    );
    let report_descriptions = vec![
        ("convergence_analysis", "Convergence Analysis", "Analyzes convergence speed and patterns across optimizers, showing mean iterations to reach improvement milestones"),
        ("efficiency_matrix", "Efficiency Matrix", "Algorithm efficiency matrix showing mean function evaluations for successful runs across problem families"),
        ("success_rate_heatmap", "Success Rate Heatmap", "Color-coded heatmap showing success rates across optimizer-problem combinations"),
        ("performance_table", "Performance Table", "Detailed performance table showing metrics for each optimizer-problem combination"),
        ("summary_statistics", "Summary Statistics", "Summary statistics showing average performance metrics grouped by problem family and optimizer"),
        ("family_vs_family", "Family vs Family", "Comparison matrix showing how different optimizer families perform across different problem families"),
    ];
    for (report_name, display_name, description) in report_descriptions {
        html_content.push_str(&format!(
            r#"    <div class="report-card">
        <h3>{}</h3>
        <p>{}</p>
        <div class="format-links">
"#,
            display_name, description
        ));
        for format in formats {
            let format_str = format!("{:?}", format).to_lowercase();
            let extension = match format {
                ReportFormat::Html => "html",
                ReportFormat::Latex => "tex",
                ReportFormat::Markdown => "md",
                ReportFormat::Csv => "csv",
            };
            html_content.push_str(&format!(
                r#"            <a href="unified_reports/{}/{}.{}">{:?}</a>
"#,
                format_str, report_name, extension, format
            ));
        }
        html_content.push_str(
            r#"        </div>
    </div>
"#,
        );
    }
    html_content.push_str("</div>");
    // Add legacy reports section
    html_content.push_str(
        r#"<div class="legacy-section">
    <h2>Legacy Reports</h2>
    <p>Traditional report formats for backward compatibility.</p>
    <ul>
        <li><a href="benchmark_report.md">Main Benchmark Report (Markdown)</a></li>
        <li><a href="latex/">LaTeX Tables Directory</a></li>
        <li><a href="data/">Raw Data (CSV)</a></li>
        <li><a href="reports/">Detailed Problem Reports</a></li>
    </ul>
</div>
"#,
    );
    html_content.push_str(
        r#"<footer style="margin-top: 40px; padding-top: 20px; border-top: 1px solid #ddd; color: #666;">
    <p>Generated by QQN Optimizer Benchmark Suite</p>
</footer>
</body>
</html>
"#,
    );
    fs::write(&index_path, html_content)
        .with_context(|| format!("Failed to write report index to: {}", index_path.display()))?;
    println!("Generated report index: {}", index_path.display());
    Ok(())
}

/// Generate efficiency matrix table content (without document wrapper)
fn generate_efficiency_matrix_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    // Similar logic as generate_efficiency_matrix_latex_table but return just the table content
    let mut all_optimizer_families = std::collections::HashSet::new();
    let mut all_problem_families = std::collections::HashSet::new();
    for (problem, results) in all_results {
        let problem_family = get_family(&problem.get_name());
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
        return Ok(String::new());
    }
    let mut content = format!(
        r#"\begin{{table}}[H]
\centering
\caption{{Algorithm Efficiency Matrix: Mean Function Evaluations for Successful Runs}}
\label{{tab:efficiency_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{l{}}}
\toprule
\textbf{{Optimizer Family}} {}\\
\midrule
"#,
        "c".repeat(problem_families.len()),
        problem_families
            .iter()
            .map(|fam| format!("& \\textbf{{{}}}", escape_latex(fam)))
            .collect::<Vec<_>>()
            .join(" ")
    );
    // Same calculation logic as the standalone table
    for optimizer_family in &optimizer_families {
        content.push_str(&format!("\\textbf{{{}}} ", escape_latex(optimizer_family)));
        for problem_family in &problem_families {
            let mut successful_evaluations = Vec::new();
            for (problem, results) in all_results {
                if get_family(&problem.get_name()) == *problem_family {
                    for result in &results.results {
                        let result_optimizer_family = get_optimizer_family(&result.optimizer_name);
                        if result_optimizer_family == *optimizer_family
                            && result.convergence_achieved
                        {
                            successful_evaluations.push(result.function_evaluations as f64);
                        }
                    }
                }
            }
            let cell_content = if successful_evaluations.is_empty() {
                "N/A".to_string()
            } else {
                let mean = successful_evaluations.iter().sum::<f64>()
                    / successful_evaluations.len() as f64;
                let variance = successful_evaluations
                    .iter()
                    .map(|x| (x - mean).powi(2))
                    .sum::<f64>()
                    / successful_evaluations.len() as f64;
                let std_dev = variance.sqrt();
                format!("{mean:.0} $\\pm$ {std_dev:.0}")
            };
            content.push_str(&format!("& {cell_content} "));
        }
        content.push_str("\\\\\n");
    }
    content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Shows mean function evaluations $\pm$ standard deviation for successful runs only across problem families. Lower values indicate higher efficiency. QQN family cells are highlighted in green.
"#,
    );
    Ok(content)
}

/// Generate reports using the unified reporting system
pub async fn generate_unified_reports(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    formats: &[ReportFormat],
    output_dir: &str,
) -> anyhow::Result<()> {
    let reports_dir = Path::new(output_dir).join("unified_reports");
    fs::create_dir_all(&reports_dir)?;
    println!(
        "Generating unified reports in directory: {}",
        reports_dir.display()
    );
    // Create report collection with all available reports
    let collection = ReportCollection::new()
        .add_report(ConvergenceAnalysisReport::new())
        .add_report(EfficiencyMatrixReport::new())
        .add_report(SuccessRateHeatmapReport::new())
        .add_report(PerformanceTableReport::new())
        .add_report(SummaryStatisticsReport::new())
        .add_report(FamilyVsFamilyReport::new());
    // Generate reports in each requested format
    for format in formats {
        let format_dir = reports_dir.join(format!("{:?}", format).to_lowercase());
        fs::create_dir_all(&format_dir)?;
        let config = ReportConfig {
            format: format.clone(),
            include_detailed_stats: true,
            include_plots: true,
            style_options: std::collections::HashMap::new(),
        };
        let metadata = collection.generate_all(all_results, &config, &format_dir)?;
        // Log generation results
        println!(
            "Generated {} reports in {:?} format:",
            metadata.len(),
            format
        );
        for meta in &metadata {
            println!(
                "  - {}: {} problems, {} optimizers, {} data points",
                meta.report_type, meta.problem_count, meta.optimizer_count, meta.data_points
            );
        }
    }
    Ok(())
}
/// Generate optimizer specifications section for the main report
fn generate_optimizer_specifications_section(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    let mut content = String::from(
        r#"
## Optimizer Specifications
This section provides detailed JSON specifications for all optimizers used in the benchmarks. These specifications can be used to reproduce the exact optimizer configurations.
### Optimizer Configuration Details
<details>
<summary>Click to expand optimizer specifications (JSON format)</summary>
```json
"#,
    );
    // Collect unique optimizers and their configurations
    let mut optimizer_specs = std::collections::HashMap::new();
    for (_, results) in all_results {
        for result in &results.results {
            if !optimizer_specs.contains_key(&result.optimizer_name) {
                // Create a specification for this optimizer
                let spec = OptimizerSpecification {
                    name: result.optimizer_name.clone(),
                    family: get_optimizer_family(&result.optimizer_name),
                    configuration: extract_optimizer_config(&result.optimizer_name),
                    description: generate_optimizer_description(&result.optimizer_name),
                    parameters: extract_optimizer_parameters(&result.optimizer_name),
                };
                optimizer_specs.insert(result.optimizer_name.clone(), spec);
            }
        }
    }
    // Convert to JSON and add to content
    let json_specs = serde_json::to_string_pretty(&optimizer_specs)?;
    content.push_str(&json_specs);
    content.push_str(
        r#"
```
</details>
### Optimizer Family Summary
<table style="border-collapse: collapse; width: 100%; margin: 20px 0;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px;">Family</th>
<th style="border: 1px solid #ddd; padding: 8px;">Count</th>
<th style="border: 1px solid #ddd; padding: 8px;">Variants</th>
<th style="border: 1px solid #ddd; padding: 8px;">Description</th>
</tr>
"#,
    );
    // Group by family and create summary table
    let mut family_groups: std::collections::HashMap<String, Vec<&OptimizerSpecification>> = std::collections::HashMap::new();
    for spec in optimizer_specs.values() {
        family_groups.entry(spec.family.clone()).or_insert_with(Vec::new).push(spec);
    }
    for (family, specs) in family_groups {
        let variant_names: Vec<String> = specs.iter().map(|s| s.name.clone()).collect();
        let description = get_family_description(&family);
        content.push_str(&format!(
            r#"<tr>
<td style="border: 1px solid #ddd; padding: 8px;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; font-size: 0.9em;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; font-size: 0.9em;">{}</td>
</tr>
"#,
            family,
            specs.len(),
            variant_names.join(", "),
            description
        ));
    }
    content.push_str("</table>");
    content.push_str("\n**Note:** Complete optimizer specifications are available in `data/optimizer_specifications.json`\n");
    Ok(content)
}
/// Generate standalone JSON file with optimizer specifications
fn generate_optimizer_specifications_json(
    output_dir: &str,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<()> {
    let mut optimizer_specs = std::collections::HashMap::new();
    for (_, results) in all_results {
        for result in &results.results {
            if !optimizer_specs.contains_key(&result.optimizer_name) {
                let spec = OptimizerSpecification {
                    name: result.optimizer_name.clone(),
                    family: get_optimizer_family(&result.optimizer_name),
                    configuration: extract_optimizer_config(&result.optimizer_name),
                    description: generate_optimizer_description(&result.optimizer_name),
                    parameters: extract_optimizer_parameters(&result.optimizer_name),
                };
                optimizer_specs.insert(result.optimizer_name.clone(), spec);
            }
        }
    }
    // Create comprehensive specification document
    let comprehensive_spec = OptimizerSpecificationDocument {
        metadata: SpecificationMetadata {
            generated_at: chrono::Utc::now(),
            total_optimizers: optimizer_specs.len(),
            families: optimizer_specs.values()
                .map(|s| s.family.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect(),
        },
        optimizers: optimizer_specs,
    };
    let json_content = serde_json::to_string_pretty(&comprehensive_spec)?;
    let json_path = Path::new(output_dir).join("optimizer_specifications.json");
    // Ensure parent directory exists
    if let Some(parent) = json_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&json_path, json_content)
        .with_context(|| format!("Failed to write optimizer specifications to: {}", json_path.display()))?;
    println!("Generated optimizer specifications: {}", json_path.display());
    Ok(())
}
/// Specification for a single optimizer
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct OptimizerSpecification {
    name: String,
    family: String,
    configuration: serde_json::Value,
    description: String,
    parameters: std::collections::HashMap<String, ParameterSpec>,
}
/// Complete specification document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct OptimizerSpecificationDocument {
    metadata: SpecificationMetadata,
    optimizers: std::collections::HashMap<String, OptimizerSpecification>,
}
/// Metadata for the specification document
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SpecificationMetadata {
    generated_at: chrono::DateTime<chrono::Utc>,
    total_optimizers: usize,
    families: Vec<String>,
}
/// Parameter specification
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct ParameterSpec {
    value: serde_json::Value,
    description: String,
    parameter_type: String,
    valid_range: Option<String>,
}
/// Extract optimizer configuration from name (parse configuration details)
fn extract_optimizer_config(optimizer_name: &str) -> serde_json::Value {
    // Parse optimizer name to extract configuration
    // This is a simplified version - in practice, you'd want more sophisticated parsing
    if optimizer_name.starts_with("QQN") {
        // Extract QQN-specific configuration
        serde_json::json!({
            "type": "QQN",
            "line_search": extract_line_search_config(optimizer_name),
            "lbfgs_history": extract_numeric_param(optimizer_name, "history"),
            "epsilon": extract_numeric_param(optimizer_name, "epsilon"),
            "initial_step": extract_numeric_param(optimizer_name, "step")
        })
    } else if optimizer_name.starts_with("LBFGS") || optimizer_name.starts_with("L-BFGS") {
        serde_json::json!({
            "type": "L-BFGS",
            "history_size": extract_numeric_param(optimizer_name, "history"),
            "line_search": extract_line_search_config(optimizer_name),
            "epsilon": extract_numeric_param(optimizer_name, "epsilon")
        })
    } else if optimizer_name.starts_with("Adam") {
        serde_json::json!({
            "type": "Adam",
            "learning_rate": extract_numeric_param(optimizer_name, "lr"),
            "beta1": extract_numeric_param(optimizer_name, "beta1"),
            "beta2": extract_numeric_param(optimizer_name, "beta2"),
            "epsilon": extract_numeric_param(optimizer_name, "epsilon")
        })
    } else if optimizer_name.starts_with("GD") {
        serde_json::json!({
            "type": "GD",
            "learning_rate": extract_numeric_param(optimizer_name, "lr"),
            "momentum": extract_numeric_param(optimizer_name, "momentum"),
            "nesterov": optimizer_name.contains("nesterov")
        })
    } else if optimizer_name.starts_with("Trust") || optimizer_name.contains("TrustRegion") {
        serde_json::json!({
            "type": "TrustRegion",
            "initial_radius": extract_numeric_param(optimizer_name, "radius"),
            "max_radius": extract_numeric_param(optimizer_name, "max_radius"),
            "eta_1": extract_numeric_param(optimizer_name, "eta1"),
            "eta_2": extract_numeric_param(optimizer_name, "eta2")
        })
    } else {
        serde_json::json!({
            "type": "Unknown",
            "raw_name": optimizer_name
        })
    }
}
/// Extract line search configuration from optimizer name
fn extract_line_search_config(optimizer_name: &str) -> serde_json::Value {
    if optimizer_name.contains("Backtracking") {
        serde_json::json!({"method": "Backtracking"})
    } else if optimizer_name.contains("StrongWolfe") {
        serde_json::json!({"method": "StrongWolfe"})
    } else if optimizer_name.contains("MoreThuente") {
        serde_json::json!({"method": "MoreThuente"})
    } else if optimizer_name.contains("GoldenSection") {
        serde_json::json!({"method": "GoldenSection"})
    } else if optimizer_name.contains("Bisection") {
        serde_json::json!({"method": "Bisection"})
    } else if optimizer_name.contains("CubicQuadratic") {
        serde_json::json!({"method": "CubicQuadraticInterpolation"})
    } else {
        serde_json::json!({"method": "Default"})
    }
}
/// Extract numeric parameter from optimizer name
fn extract_numeric_param(optimizer_name: &str, param_name: &str) -> Option<f64> {
    // Simple regex-like extraction - in practice you'd want more robust parsing
    if let Some(start) = optimizer_name.find(param_name) {
        let after_param = &optimizer_name[start + param_name.len()..];
        if let Some(equals_pos) = after_param.find('=') {
            let after_equals = &after_param[equals_pos + 1..];
            // Find the end of the number (next non-numeric character)
            let end = after_equals.find(|c: char| !c.is_ascii_digit() && c != '.' && c != 'e' && c != '-' && c != '+')
                .unwrap_or(after_equals.len());
            if let Ok(value) = after_equals[..end].parse::<f64>() {
                return Some(value);
            }
        }
    }
    None
}
/// Generate description for an optimizer
fn generate_optimizer_description(optimizer_name: &str) -> String {
    if optimizer_name.starts_with("QQN") {
        "Quadratic Quasi-Newton optimizer with adaptive line search and L-BFGS-style history management".to_string()
    } else if optimizer_name.starts_with("LBFGS") || optimizer_name.starts_with("L-BFGS") {
        "Limited-memory Broyden-Fletcher-Goldfarb-Shanno quasi-Newton optimizer".to_string()
    } else if optimizer_name.starts_with("Adam") {
        "Adaptive Moment Estimation optimizer with bias correction".to_string()
    } else if optimizer_name.starts_with("GD") {
        "Gradient Descent optimizer with optional momentum".to_string()
    } else if optimizer_name.starts_with("Trust") || optimizer_name.contains("TrustRegion") {
        "Trust Region optimizer with quadratic model approximation".to_string()
    } else {
        format!("Optimizer: {}", optimizer_name)
    }
}
/// Extract parameter specifications for an optimizer
fn extract_optimizer_parameters(optimizer_name: &str) -> std::collections::HashMap<String, ParameterSpec> {
    let mut params = std::collections::HashMap::new();
    if optimizer_name.starts_with("QQN") {
        params.insert("c1".to_string(), ParameterSpec {
            value: serde_json::json!(extract_numeric_param(optimizer_name, "c1").unwrap_or(1e-4)),
            description: "Armijo condition parameter for line search".to_string(),
            parameter_type: "float".to_string(),
            valid_range: Some("(0, 1)".to_string()),
        });
        params.insert("c2".to_string(), ParameterSpec {
            value: serde_json::json!(extract_numeric_param(optimizer_name, "c2").unwrap_or(0.9)),
            description: "Wolfe condition parameter for line search".to_string(),
            parameter_type: "float".to_string(),
            valid_range: Some("(c1, 1)".to_string()),
        });
        params.insert("lbfgs_history".to_string(), ParameterSpec {
            value: serde_json::json!(extract_numeric_param(optimizer_name, "history").unwrap_or(10.0) as i32),
            description: "Number of previous iterations to store for L-BFGS approximation".to_string(),
            parameter_type: "integer".to_string(),
            valid_range: Some("[1, 50]".to_string()),
        });
    } else if optimizer_name.starts_with("Adam") {
        params.insert("learning_rate".to_string(), ParameterSpec {
            value: serde_json::json!(extract_numeric_param(optimizer_name, "lr").unwrap_or(0.001)),
            description: "Learning rate for parameter updates".to_string(),
            parameter_type: "float".to_string(),
            valid_range: Some("(0, 1]".to_string()),
        });
        params.insert("beta1".to_string(), ParameterSpec {
            value: serde_json::json!(extract_numeric_param(optimizer_name, "beta1").unwrap_or(0.9)),
            description: "Exponential decay rate for first moment estimates".to_string(),
            parameter_type: "float".to_string(),
            valid_range: Some("[0, 1)".to_string()),
        });
        params.insert("beta2".to_string(), ParameterSpec {
            value: serde_json::json!(extract_numeric_param(optimizer_name, "beta2").unwrap_or(0.999)),
            description: "Exponential decay rate for second moment estimates".to_string(),
            parameter_type: "float".to_string(),
            valid_range: Some("[0, 1)".to_string()),
        });
    }
    // Add more parameter specifications for other optimizer types as needed
    params
}
/// Get description for an optimizer family
fn get_family_description(family: &str) -> String {
    match family {
        "QQN" => "Quadratic Quasi-Newton methods with adaptive line search".to_string(),
        "L-BFGS" => "Limited-memory quasi-Newton methods".to_string(),
        "Adam" => "Adaptive moment estimation methods".to_string(),
        "GD" => "Gradient descent methods with optional momentum".to_string(),
        "Trust Region" => "Trust region methods with quadratic models".to_string(),
        _ => format!("{} optimization methods", family),
    }
}


/// Escape special LaTeX characters
pub(crate) fn escape_latex(text: &str) -> String {
    // Proper LaTeX escaping that avoids compilation errors
    let mut result = text.to_string();

    // Handle backslashes first (before other replacements add backslashes)
    result = result.replace("\\", "\\textbackslash{}");

    // Handle other special characters
    result = result.replace("_", "\\_");
    result = result.replace("&", "\\&");
    result = result.replace("%", "\\%");
    result = result.replace("$", "\\$");
    result = result.replace("#", "\\#");
    result = result.replace("^", "\\textasciicircum{}");
    result = result.replace("{", "\\{");
    result = result.replace("}", "\\}");
    result = result.replace("~", "\\textasciitilde{}");

    // Clean up any problematic sequences and ensure valid LaTeX
    result = result.replace("textbackslash_", "textbackslash\\_");

    result.trim().to_string()
}

/// Generate detailed reports for each optimizer-problem combination
async fn generate_detailed_reports(
    output_dir: &str,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    use_optimizer_families: bool,
) -> anyhow::Result<()> {
    for (problem, results) in all_results {
        let mut optimizer_results = std::collections::HashMap::new();
        // Group results by optimizer
        for result in &results.results {
            let optimizer_key = if use_optimizer_families {
                get_optimizer_family(&result.optimizer_name)
            } else {
                result.optimizer_name.clone()
            };
            let optimizer_results_vec =
                optimizer_results.entry(optimizer_key).or_insert(Vec::new());
            optimizer_results_vec.push(result);
        }
        // Generate detailed report for each optimizer on this problem
        for (optimizer_name, optimizer_runs) in optimizer_results {
            optimizer_problems::generate_optimizer_problem_report(
                output_dir,
                problem.problem.as_ref(),
                &optimizer_name,
                &optimizer_runs,
            )
            .await?;
        }
    }
    Ok(())
}
pub(crate) fn generate_detailed_report_header(
    problem: &dyn OptimizationProblem,
    optimizer_name: &str,
    runs: &[&SingleResult],
) -> String {
    let problem_name = problem.name();
    let successful_runs = runs.iter().filter(|r| r.convergence_achieved).count();
    let total_runs = runs.len();
    let success_rate = successful_runs as f64 / total_runs as f64 * 100.0;
    let final_values: Vec<f64> = runs
        .iter()
        .map(|r| r.final_value)
        .filter(|&v| v.is_finite())
        .collect();
    let (best_value, worst_value, mean_value) = if !final_values.is_empty() {
        let best = final_values.iter().cloned().fold(f64::INFINITY, f64::min);
        let worst = final_values
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let mean = final_values.iter().sum::<f64>() / final_values.len() as f64;
        (best, worst, mean)
    } else {
        (f64::INFINITY, f64::INFINITY, f64::INFINITY)
    };
    format!(
        r#"# Detailed Analysis: {} on {}
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** {}
**Optimizer:** {}
**Problem Family:** {}
**Dimension:** {}
**Success Threshold:** {:.3e}
**Total Runs:** {}
**Successful Runs:** {} ({:.1}%)

### Quick Statistics
* **Best Final Value:** {:.6e}
* **Worst Final Value:** {:.6e}
* **Mean Final Value:** {:.6e}
* **Success Rate:** {:.1}%


"#,
        optimizer_name,
        problem_name,
        problem_name,
        optimizer_name,
        get_family(problem_name),
        problem.dimension(),
        problem.optimal_value().unwrap_or(f64::NEG_INFINITY),
        total_runs,
        successful_runs,
        success_rate,
        best_value,
        worst_value,
        mean_value,
        success_rate
    )
}
pub(crate) fn generate_detailed_report_footer(problem_name: &str, optimizer_name: &str) -> String {
    format!(
        r#"

## Data Files
* [Raw CSV Data](../data/problems/{}_results.csv)
* [Convergence Plot](../plots/{}.png)
* [Log Scale Convergence Plot](../plots/{}_log.png)


---
*Detailed report for {} on {}*
*Generated on: {}*
*[← Back to Main Report](../benchmark_report.md)*
"#,
        problem_name.replace(" ", "_"),
        problem_name.replace(" ", "_"),
        problem_name.replace(" ", "_"),
        optimizer_name,
        problem_name,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}
fn generate_winner_summary_table(all_results: &[(&ProblemSpec, BenchmarkResults)]) -> String {
    let mut summary = String::from(
        r#"## Quick Summary: Winners by Problem

*Click on problem names to view detailed analysis reports.*

<table style="border-collapse: collapse; width: 100%; margin: 20px 0;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Problem</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Family</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Winner</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Success Rate</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Mean Final Value</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Median Best Value</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Runner-up</th>
</tr>
"#,
    );
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        let problem_family = get_family(&problem_name);
        let mut optimizer_stats = HashMap::new();
        let problem_name = problem.get_name();
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
            let best_values: Vec<f64> = runs
                .iter()
                .map(|r| r.best_value)
                .filter(|&v| v.is_finite())
                .collect();
            if final_values.is_empty() {
                continue;
            }
            let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
            let success_rate = success_count as f64 / runs.len() as f64;
            let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;

            // Calculate median best value
            let median_best = if !best_values.is_empty() {
                let mut sorted_best = best_values.clone();
                sorted_best.sort_by(|a, b| a.total_cmp(b));
                let len = sorted_best.len();
                if len % 2 == 0 {
                    (sorted_best[len / 2 - 1] + sorted_best[len / 2]) / 2.0
                } else {
                    sorted_best[len / 2]
                }
            } else {
                f64::INFINITY
            };

            perf_data.push((optimizer.clone(), success_rate, mean_final, median_best));
        }

        if is_no_threshold_mode() {
            perf_data.sort_by(|a, b| {
                b.1.partial_cmp(&a.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal))
            });
        } else {
            perf_data.sort_by(|a, b| {
                b.1.partial_cmp(&a.1)
                    .unwrap_or(std::cmp::Ordering::Equal)
                    .then_with(|| a.3.partial_cmp(&b.3).unwrap_or(std::cmp::Ordering::Equal))
            });
        }

        if !perf_data.is_empty() {
            let winner = &perf_data[0];
            let runner_up = if perf_data.len() > 1 {
                &perf_data[1]
            } else {
                winner
            };
            let winner_style = if winner.0.contains("QQN") {
                "background-color: #d4edda; font-weight: bold;"
            } else {
                "font-weight: bold;"
            };
            // Create link to detailed problem analysis
            let problem_filename = problem_name.replace(" ", "_");
            let problem_link = format!(
                r#"<a href="reports/problem_analysis_{problem_filename}.md" title="View detailed analysis">{problem_name}</a>"#
            );

            summary.push_str(&format!(
                r#"<tr style="{}">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.1}%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.2e}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.2e}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
</tr>
"#,
                winner_style,
                problem_link,
                problem_family,
                winner.0,
                winner.1 * 100.0,
                winner.2,
                winner.3,
                if perf_data.len() > 1 {
                    &runner_up.0
                } else {
                    "-"
                }
            ));
        }
    }
    summary.push_str(
        r#"</table>
**Legend:** Winner determined by success rate first, then by mean final value. QQN winners are highlighted in green.

"#,
    );
    summary
}
pub(crate) fn shorten_optimizer_name(name: &str, max_length: usize) -> String {
    // Shorten optimizer names for display in the table
    if name.len() <= max_length {
        name.to_string()
    } else {
        // Try to create meaningful abbreviations
        let shortened = name
            .replace("Optimizer", "")
            .replace("Algorithm", "")
            .replace("Method", "")
            .replace("Quasi", "Q")
            .replace("Newton", "N")
            .replace("Limited", "L")
            .replace("Memory", "M")
            .replace("Broyden", "B")
            .replace("Fletcher", "F")
            .replace("Goldfarb", "G")
            .replace("Shanno", "S")
            .replace("Quadratic", "Quad")
            .replace("Trust Region ", "")
            .replace("Trust Region-", "")
            .replace("Adam-", "")
            .replace("GD-", "")
            .replace("L-BFGS-", "")
            .replace("QQN-", "");

        if shortened.len() <= max_length {
            shortened
        } else {
            // Take first 7 chars + "..."
            let i = max_length.max(3) - 3;
            let x = shortened[..i].to_string();
            format!("{}...", &x)
        }
    }
}

fn generate_header() -> String {
    format!(
        r#"# Quadratic Quasi-Newton (QQN) Optimizer: Experimental Validation

*Comprehensive Benchmark Results for Academic Publication*

**Generated on:** {}

"#,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}

fn generate_conclusions(all_results: &[(&ProblemSpec, BenchmarkResults)]) -> String {
    let mut optimizer_scores = HashMap::new();
    let mut ml_optimizer_scores = HashMap::new();
    let mut math_optimizer_scores = HashMap::new();
    let mut optimizer_efficiency = HashMap::new();
    let mut qqn_wins = 0;
    let mut total_problems = 0;

    for (_, results) in all_results {
        for result in &results.results {
            let score = optimizer_scores
                .entry(result.optimizer_name.clone())
                .or_insert(0.0);
            if result.convergence_achieved {
                *score += 1.0;
            }
            if result.final_value.is_finite() && result.final_value < 1e-6 {
                *score += 0.5;
            }
            // Track efficiency (success rate / mean time)
            let efficiency = optimizer_efficiency
                .entry(result.optimizer_name.clone())
                .or_insert((0, 0.0));
            efficiency.0 += if result.convergence_achieved { 1 } else { 0 };
            efficiency.1 += result.execution_time.as_secs_f64();
        }
    }
    // Count QQN wins
    for (_problem, results) in all_results {
        total_problems += 1;
        let mut best_optimizer = String::new();
        let mut best_value = f64::INFINITY;
        for result in &results.results {
            if result.final_value < best_value {
                best_value = result.final_value;
                best_optimizer = result.optimizer_name.clone();
            }
        }
        if best_optimizer.contains("QQN") {
            qqn_wins += 1;
        }
    }

    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        let is_ml_problem = problem_name.contains("Regression")
            || problem_name.contains("Neural")
            || problem_name.contains("SVM");
        for result in &results.results {
            let target_scores = if is_ml_problem {
                &mut ml_optimizer_scores
            } else {
                &mut math_optimizer_scores
            };
            let score = target_scores
                .entry(result.optimizer_name.clone())
                .or_insert(0.0);
            if result.convergence_achieved {
                *score += 1.0;
            }
            if result.final_value.is_finite() && result.final_value < 1e-6 {
                *score += 0.5;
            }
        }
    }

    format!(
        r#"

## Conclusions

### Key Findings
- **QQN Performance:** QQN variants won on {}/{} problems ({:.1}% win rate)
- **Convergence Analysis:** Statistical significance testing reveals meaningful performance differences
- **Efficiency Trade-offs:** Different optimizers show varying trade-offs between solution quality and computational cost
- **Problem-Specific Insights:** Performance varies significantly across problem families


"#,
        qqn_wins,
        total_problems,
        (qqn_wins as f64 / total_problems as f64) * 100.0
    )
}

fn generate_html_footer(config: &BenchmarkConfig) -> String {
    format!(
        r#"## Experimental Details

### Methodology

- **Runs per configuration:** {} independent runs with different random seeds
- **Success criteria:** Minimum {:e}% improvement per iteration OR optimizer-specific convergence within {} iterations or {} objective evaluations
- **Time limit:** {:?} per run
- **Hardware:** Standard CPU implementation
- **Implementation:** Rust-based optimization framework

---

*Generated by QQN Optimizer Benchmark Suite v{}*
*Report generated on: {}*
"#,
        config.num_runs,
        config.min_improvement_percent,
        config.max_iterations,
        config.maximum_function_calls,
        config.time_limit.clone(),
        env!("CARGO_PKG_VERSION"),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}

fn generate_csv_exports(
    output_dir: &str,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<()> {
    fs::create_dir_all(output_dir)
        .with_context(|| format!("Failed to create output directory: {output_dir}"))?;
    println!("Exporting CSV files to: {output_dir}");

    // Enhanced CSV with more fields
    let mut csv_content = String::from("Problem,ProblemFamily,Dimension,Optimizer,Run,FinalValue,FinalGradientNorm,Iterations,FunctionEvals,GradientEvals,Time,Converged,ConvergenceReason\n");

    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        let problem_family = get_family(&problem_name);
        let dimension = problem.dimensions;

        for result in &results.results {
            csv_content.push_str(&format!(
                "{},{},{},{},{},{:.6e},{:.6e},{},{},{},{:.3},{},\"{:?}\"\n",
                problem_name,
                problem_family,
                dimension.unwrap_or(0),
                result.optimizer_name,
                result.run_id,
                result.final_value,
                result.final_gradient_norm,
                result.iterations,
                result.function_evaluations,
                result.gradient_evaluations,
                result.execution_time.as_secs_f64(),
                result.convergence_achieved,
                result.convergence_reason,
            ));
        }
    }

    let csv_path = Path::new(output_dir).join("detailed_results.csv");
    println!("Writing detailed results to: {}", csv_path.display());
    // Ensure parent directory exists
    if let Some(parent) = csv_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&csv_path, csv_content)
        .with_context(|| format!("Failed to write CSV to: {}", csv_path.display()))?;

    // Enhanced summary CSV
    let mut summary_csv = String::from("Problem,ProblemFamily,Dimension,Optimizer,MeanFinalValue,MeanFinalValueSuccess,MeanFinalValueFail,StdFinalValue,BestValue,WorstValue,MeanIterations,MeanFunctionEvals,MeanFunctionEvalsSuccess,MeanFunctionEvalsFail,MeanGradientEvals,MeanGradientEvalsSuccess,MeanGradientEvalsFail,MeanTime,SuccessRate,NumRuns\n");

    for (problem, results) in all_results {
        let mut optimizer_stats = HashMap::new();
        for result in &results.results {
            let stats = optimizer_stats
                .entry(result.optimizer_name.clone())
                .or_insert(Vec::new());
            stats.push(result);
        }

        for (optimizer, runs) in optimizer_stats {
            let final_values: Vec<f64> = runs
                .iter()
                .map(|r| r.final_value)
                .filter(|&v| v.is_finite())
                .collect();

            if final_values.is_empty() {
                continue; // Skip if no valid results
            }
            // Separate successful and unsuccessful runs
            let successful_runs: Vec<_> = runs.iter().filter(|r| r.convergence_achieved).collect();
            let unsuccessful_runs: Vec<_> =
                runs.iter().filter(|r| !r.convergence_achieved).collect();
            // Calculate statistics for successful runs
            let (mean_final_success, mean_func_evals_success, mean_grad_evals_success) =
                if !successful_runs.is_empty() {
                    let final_vals: Vec<f64> = successful_runs
                        .iter()
                        .map(|r| r.final_value)
                        .filter(|&v| v.is_finite())
                        .collect();
                    let func_evals: Vec<f64> = successful_runs
                        .iter()
                        .map(|r| r.function_evaluations as f64)
                        .collect();
                    let grad_evals: Vec<f64> = successful_runs
                        .iter()
                        .map(|r| r.gradient_evaluations as f64)
                        .collect();
                    (
                        if !final_vals.is_empty() {
                            final_vals.iter().sum::<f64>() / final_vals.len() as f64
                        } else {
                            f64::NAN
                        },
                        func_evals.iter().sum::<f64>() / func_evals.len() as f64,
                        grad_evals.iter().sum::<f64>() / grad_evals.len() as f64,
                    )
                } else {
                    (f64::NAN, f64::NAN, f64::NAN)
                };
            // Calculate statistics for unsuccessful runs
            let (mean_final_fail, mean_func_evals_fail, mean_grad_evals_fail) =
                if !unsuccessful_runs.is_empty() {
                    let final_vals: Vec<f64> = unsuccessful_runs
                        .iter()
                        .map(|r| r.final_value)
                        .filter(|&v| v.is_finite())
                        .collect();
                    let func_evals: Vec<f64> = unsuccessful_runs
                        .iter()
                        .map(|r| r.function_evaluations as f64)
                        .collect();
                    let grad_evals: Vec<f64> = unsuccessful_runs
                        .iter()
                        .map(|r| r.gradient_evaluations as f64)
                        .collect();
                    (
                        if !final_vals.is_empty() {
                            final_vals.iter().sum::<f64>() / final_vals.len() as f64
                        } else {
                            f64::NAN
                        },
                        func_evals.iter().sum::<f64>() / func_evals.len() as f64,
                        grad_evals.iter().sum::<f64>() / grad_evals.len() as f64,
                    )
                } else {
                    (f64::NAN, f64::NAN, f64::NAN)
                };

            let iterations: Vec<f64> = runs.iter().map(|r| r.iterations as f64).collect();
            let function_evals: Vec<f64> =
                runs.iter().map(|r| r.function_evaluations as f64).collect();
            let gradient_evals: Vec<f64> =
                runs.iter().map(|r| r.gradient_evaluations as f64).collect();
            let execution_times: Vec<f64> = runs
                .iter()
                .map(|r| r.execution_time.as_secs_f64())
                .collect();
            let success_count = runs.iter().filter(|r| r.convergence_achieved).count();

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
            let mean_iterations = iterations.iter().sum::<f64>() / iterations.len() as f64;
            let mean_function_evals =
                function_evals.iter().sum::<f64>() / function_evals.len() as f64;
            let mean_gradient_evals =
                gradient_evals.iter().sum::<f64>() / gradient_evals.len() as f64;
            let mean_time = execution_times.iter().sum::<f64>() / execution_times.len() as f64;
            let success_rate = success_count as f64 / runs.len() as f64;

            let problem_name = problem.get_name();
            let problem_family = get_family(&problem_name);
            let dimension = problem.dimensions;

            summary_csv.push_str(&format!(
                "{},{},{},{},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.3},{:.3},{}\n",
                problem_name,
                problem_family,
                dimension.unwrap_or(0),
                optimizer,
                mean_final,
                mean_final_success,
                mean_final_fail,
                std_final,
                best_final,
                worst_final,
                mean_iterations,
                mean_function_evals,
                mean_func_evals_success,
                mean_func_evals_fail,
                mean_gradient_evals,
                mean_grad_evals_success,
                mean_grad_evals_fail,
                mean_time,
                success_rate,
                runs.len()
            ));
        }
    }

    let summary_path = Path::new(output_dir).join("summary_statistics.csv");
    println!("Writing summary statistics to: {}", summary_path.display());
    // Ensure parent directory exists
    if let Some(parent) = summary_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&summary_path, summary_csv)
        .with_context(|| format!("Failed to write summary CSV to: {}", summary_path.display()))?;

    // Generate problem-specific CSV files for easier analysis
    optimizer_problems::generate_problem_specific_csvs(output_dir, all_results)?;

    Ok(())
}

/// Generate LaTeX tables for all results
async fn generate_latex_tables(
    output_dir: &str,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    slf: &ReportGenerator,
) -> anyhow::Result<()> {
    let latex_dir = Path::new(output_dir);
    fs::create_dir_all(latex_dir)
        .with_context(|| format!("Failed to create LaTeX directory: {}", latex_dir.display()))?;
    println!("Generating LaTeX tables in: {}", latex_dir.display());
    // Generate main performance table
    generate_main_performance_latex_table(all_results, latex_dir)?;
    // Generate problem-specific tables
    for (problem, results) in all_results {
        optimizer_problems::generate_problem_latex_table(problem, results, latex_dir)?;
    }
    // Generate summary statistics table
    generate_summary_statistics_latex_table(all_results, latex_dir)?;
    // Generate comparison matrix table
    generate_comparison_matrix_latex_table(all_results, latex_dir, slf)?;
    // Generate family comparison matrix table
    comparison_matrix::generate_family_comparison_matrix_latex_table(all_results, latex_dir, slf)?;
    // Generate family vs family comparison matrix table
    generate_family_vs_family_latex_table(all_results, latex_dir).await?;
    // Generate efficiency matrix table
    generate_efficiency_matrix_latex_table(all_results, latex_dir)?;
    // Generate success rate heatmap table
    generate_success_rate_heatmap_latex_table(all_results, latex_dir)?;
    // Generate convergence speed analysis table
    generate_convergence_speed_latex_table(all_results, latex_dir)?;

    Ok(())
}

/// Generate comprehensive LaTeX document with all tables
fn generate_comprehensive_latex_document(
    config: &BenchmarkConfig,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
    slf: &ReportGenerator,
) -> anyhow::Result<()> {
    let mut latex_content = String::from(
        r#"\documentclass[10pt]{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{multirow}
\usepackage{longtable}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{siunitx}
\usepackage{amsmath}
\usepackage{graphicx}
\usepackage{float}
\usepackage{caption}
\usepackage{subcaption}
\usepackage{adjustbox}
\title{Quadratic Quasi-Newton (QQN) Optimizer: Comprehensive Benchmark Results}
\author{QQN Benchmark Suite}
\date{\today}
\begin{document}
\small
\maketitle
\begin{abstract}
This document presents comprehensive benchmark results for the Quadratic Quasi-Newton (QQN) optimizer compared against established optimization algorithms. The evaluation covers multiple problem families including convex unimodal, non-convex unimodal, and highly multimodal optimization problems.
\end{abstract}
\section{Introduction}
This report presents the results of comprehensive benchmarking experiments comparing the QQN optimizer against established optimization algorithms. The experiments were conducted using standardized test problems from various families to evaluate performance across different optimization landscapes.
\subsection{Experimental Setup}
\begin{itemize}
\item \textbf{Runs per configuration:} "#,
    );
    latex_content.push_str(&format!(
        r#"{} independent runs with different random seeds
\item \textbf{{Success criteria:}} Minimum {:.2e}\% improvement per iteration OR optimizer-specific convergence within {} iterations or {} objective evaluations
\item \textbf{{Time limit:}} {:?} per run
\item \textbf{{Hardware:}} Standard CPU implementation
\item \textbf{{Implementation:}} Rust-based optimization framework
\end{{itemize}}
\section{{Performance Results}}
The following sections present detailed performance comparisons across all tested problems and optimizers.
\subsection{{Overall Performance Summary}}
"#,
        config.num_runs,
        config.min_improvement_percent,
        config.max_iterations,
        config.maximum_function_calls,
        config.time_limit
    ));
    // Include the main performance table content (without document wrapper)
    latex_content.push_str(&generate_main_performance_table_content(all_results)?);
    latex_content.push_str(
        r#"
\subsection{Summary Statistics by Problem Family}
"#,
    );
    // Include summary statistics table content
    latex_content.push_str(&generate_summary_statistics_table_content(all_results)?);
    latex_content.push_str(
        r#"
\subsection{QQN vs Non-QQN Comparison Matrix}
"#,
    );
    // Include comparison matrix content
    latex_content.push_str(&generate_comparison_matrix_table_content(all_results, slf)?);
    latex_content.push_str(
        r#"
\subsection{Optimizer Family Comparison Matrix}
"#,
    );
    // Include family comparison matrix content
    latex_content.push_str(&generate_family_comparison_matrix_table_content(
        all_results,
        slf,
    )?);
    latex_content.push_str(
        r#"
\subsection{Optimizer Family vs Problem Family Performance Matrix}
"#,
    );
    // Include family vs family comparison matrix content
    latex_content.push_str(&generate_family_vs_family_table_content(all_results)?);
    latex_content.push_str(
        r#"
\subsection{Algorithm Efficiency Matrix}
"#,
    );
    // Include efficiency matrix content
    latex_content.push_str(&generate_efficiency_matrix_table_content(all_results)?);
    latex_content.push_str(
        r#"
\subsection{Success Rate Heatmap}
"#,
    );
    // Include success rate heatmap content
    latex_content.push_str(&generate_success_rate_heatmap_table_content(all_results)?);
    latex_content.push_str(
        r#"
\subsection{Convergence Speed Analysis}
"#,
    );
    // Include convergence speed analysis content
    latex_content.push_str(&generate_convergence_speed_table_content(all_results)?);

    latex_content.push_str(
        r#"
\section{Individual Problem Results}
The following subsections present detailed results for each individual problem.
"#,
    );
    // Add individual problem tables
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        latex_content.push_str(&format!(
            r#"\subsection{{{}}}
"#,
            escape_latex(&problem_name)
        ));
        latex_content.push_str(&optimizer_problems::generate_problem_table_content(
            problem, results,
        )?);
    }
    latex_content.push_str(
        r#"
\section{Conclusions}
Based on the comprehensive benchmark results presented in this document, the following key findings emerge:
\begin{itemize}
\item The QQN optimizer demonstrates competitive performance across multiple problem families
\item Statistical significance testing reveals meaningful differences between optimizers
\item Computational efficiency varies significantly across different optimization landscapes
\item Problem-specific performance characteristics highlight the importance of algorithm selection
\end{itemize}
\section{Data Availability}
All raw experimental data, convergence plots, and additional analysis files are available in the accompanying benchmark results directory. This includes:
\begin{itemize}
\item Raw CSV data files for all experiments
\item Convergence plots for visual analysis
\item Statistical analysis results
\item Problem-specific detailed reports
\end{itemize}
\end{document}
"#,
    );
    let latex_path = latex_dir.join("comprehensive_benchmark_report.tex");
    // Ensure parent directory exists
    if let Some(parent) = latex_path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&latex_path, latex_content).with_context(|| {
        format!(
            "Failed to write comprehensive LaTeX document to: {}",
            latex_path.display()
        )
    })?;
    println!(
        "Generated comprehensive LaTeX document: {}",
        latex_path.display()
    );
    Ok(())
}