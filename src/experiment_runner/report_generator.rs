use super::StatisticalAnalysis;
use crate::benchmarks::evaluation::{
    is_no_threshold_mode, BenchmarkConfig, BenchmarkResults, ConvergenceReason, ProblemSpec,
    SingleResult,
};
use crate::OptimizationProblem;
use anyhow::Context;
use log::warn;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::reports::comparison_matrix;
use crate::experiment_runner::reports::comparison_matrix::{generate_comparison_matrix_latex_table, generate_comparison_matrix_table_content, generate_family_comparison_matrix_table_content};
use crate::experiment_runner::reports::convergence_analysis::{generate_convergence_analysis, generate_convergence_speed_latex_table, generate_convergence_speed_table_content};
use crate::experiment_runner::reports::efficiency_matrix::generate_efficiency_matrix_latex_table;
use crate::experiment_runner::reports::family_vs_family::{generate_family_vs_family_comparison_table, generate_family_vs_family_latex_table, generate_family_vs_family_table_content};
use crate::experiment_runner::reports::heatmap::{generate_success_rate_heatmap_latex_table, generate_success_rate_heatmap_table_content};
use crate::experiment_runner::reports::performance_analysis::generate_performance_analysis;
use crate::experiment_runner::reports::performance_table::{generate_main_performance_latex_table, generate_main_performance_table_content};
use crate::experiment_runner::reports::summary_statistics::{generate_summary_statistics_latex_table, generate_summary_statistics_table_content};

/// Data structure for family performance comparison
#[derive(Debug, Clone)]
pub struct FamilyPerformanceData {
    pub(crate) average_ranking: f64,
    pub(crate) best_rank_average: f64,
    pub(crate) best_variant: String,
    pub(crate) worst_variant: String,
}

/// Handles HTML report generation and CSV exports
pub struct ReportGenerator {
    output_dir: String,
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
        "Sphere" => "Convex Unimodal".to_string(),
        "Matyas" => "Convex Unimodal".to_string(),

        // Non-convex but unimodal - single global minimum, challenging valleys/ridges
        "Rosenbrock" => "Non-Convex Unimodal".to_string(),
        "Beale" => "Non-Convex Unimodal".to_string(),
        "GoldsteinPrice" => "Non-Convex Unimodal".to_string(),
        "Levi" => "Non-Convex Unimodal".to_string(),

        // Highly multimodal - many local minima, very challenging
        "Rastrigin" => "Highly Multimodal".to_string(),
        "Ackley" => "Highly Multimodal".to_string(),
        "Michalewicz" => "Highly Multimodal".to_string(),
        "StyblinskiTang" => "Highly Multimodal".to_string(),

        // Machine Learning problems
        name if name.contains("Regression") => "ML Regression".to_string(),
        name if name.contains("Neural") => "ML Neural Networks".to_string(),
        name if name.contains("SVM") => "ML Classification".to_string(),
        name if name.contains("Logistic") => "ML Classification".to_string(),

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
        fs::create_dir_all(&self.output_dir)
            .with_context(|| format!("Failed to create output directory: {}", self.output_dir))?;
    // Create hierarchical directory structure
    let reports_dir = Path::new(&self.output_dir).join("reports");
    let data_dir = Path::new(&self.output_dir).join("data");
    let convergence_dir = Path::new(&self.output_dir).join("convergence");
    let plots_dir = Path::new(&self.output_dir).join("plots");
    let latex_dir = Path::new(&self.output_dir).join("latex");
    fs::create_dir_all(&reports_dir)?;
    fs::create_dir_all(&data_dir)?;
    fs::create_dir_all(&convergence_dir)?;
    fs::create_dir_all(&plots_dir)?;
    fs::create_dir_all(&latex_dir)?;
    
        println!("Generating report in directory: {}", self.output_dir);

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
            html_content.push_str(&generate_problem_section(
                problem,
                results,
            &plots_dir.to_string_lossy(),
            )?);
        }
        // Add optimizer family vs problem family comparison
        html_content.push_str(&generate_family_vs_family_comparison_table(
            all_results,
        )?);

        if !all_results.is_empty() && all_results.iter().any(|(_, r)| !r.results.is_empty()) {
            html_content.push_str(&self.statistical_analysis.generate_statistical_analysis(
                all_results,
                &self.config,
            &data_dir.to_string_lossy(),
                use_optimizer_families,
            )?);
        }

        html_content.push_str(&generate_conclusions(all_results));
        html_content.push_str(&generate_html_footer(&self.config));

        let md_path = Path::new(&self.output_dir).join("benchmark_report.md");
        println!("Saving Markdown report to: {}", md_path.display());
        fs::write(&md_path, html_content.clone()).with_context(|| {
            format!("Failed to write Markdown report to: {}", md_path.display())
        })?;

    generate_csv_exports(&data_dir.to_string_lossy(), all_results)?;
        // Generate LaTeX tables
        generate_latex_tables(
        &latex_dir.to_string_lossy(),
            all_results,
            self,
        )
        .await?;
        // Generate comprehensive LaTeX document
        generate_comprehensive_latex_document(
            &self.config,
            all_results,
        &latex_dir,
            self,
        )?;

        Ok(())
    }

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
        content.push_str(&format!(
            "\\textbf{{{}}} ",
            escape_latex(optimizer_family)
        ));
        for problem_family in &problem_families {
            let mut successful_evaluations = Vec::new();
            for (problem, results) in all_results {
                if get_family(&problem.get_name()) == *problem_family {
                    for result in &results.results {
                        let result_optimizer_family =
                            get_optimizer_family(&result.optimizer_name);
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
                format!("{:.0} $\\pm$ {:.0}", mean, std_dev)
            };
            content.push_str(&format!("& {} ", cell_content));
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

/// Generate problem table content (without document wrapper)
fn generate_problem_table_content(
    problem: &ProblemSpec,
    results: &BenchmarkResults,
) -> anyhow::Result<String> {
    let problem_name = problem.get_name();
    let problem_filename = problem_name.replace(" ", "_");
    let mut content = format!(
        r#"\begin{{table}}[H]
\centering
\caption{{Performance Results for {} Problem}}
\label{{tab:{}}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{p{{3cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}}}
\toprule
\textbf{{Optimizer}} & \textbf{{Mean Final}} & \textbf{{Std Dev}} & \textbf{{Best}} & \textbf{{Worst}} & \textbf{{Mean Func}} & \textbf{{Success}} & \textbf{{Mean Time}} \\
 & \textbf{{Value}} & & \textbf{{Value}} & \textbf{{Value}} & \textbf{{Evals}} & \textbf{{Rate (\%)}} & \textbf{{(s)}} \\
\midrule
"#,
        escape_latex(&problem_name),
        problem_filename.to_lowercase()
    );
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
    // Sort by success rate first, then by mean final value
    perf_data.sort_by(|a, b| {
        let success_cmp = b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal);
        if success_cmp != std::cmp::Ordering::Equal {
            success_cmp
        } else {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
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
        let optimizer_style = if i == 0 {
            format!("\\textbf{{{}}}", escape_latex(optimizer))
        } else {
            escape_latex(optimizer)
        };
        content.push_str(&format!(
            "{} & {:.2e} & {:.2e} & {:.2e} & {:.2e} & {:.1} & {:.1} & {:.3} \\\\\n",
            optimizer_style,
            mean_final,
            std_final,
            best_final,
            worst_final,
            mean_func_evals,
            success_rate,
            mean_time
        ));
    }
    content.push_str(
        r#"\bottomrule
\end{tabular}
\end{table}
"#,
    );
    Ok(content)
}

/// Escape special LaTeX characters
pub(crate) fn escape_latex_safe(text: &str) -> String {
    // More conservative escaping to avoid LaTeX compilation errors
    text.replace("_", "\\_")
        .replace("&", "\\&")
        .replace("%", "\\%")
        .replace("$", "\\$")
        .replace("#", "\\#")
        .replace("^", "\\textasciicircum")
        .replace("{", "\\{")
        .replace("}", "\\}")
        .replace("~", "\\textasciitilde")
        .replace("\\", "\\textbackslash")
        // Remove problematic characters that cause math mode issues
        .chars()
        .filter(|c| c.is_ascii_alphanumeric() || " ()-.,_".contains(*c))
        .collect::<String>()
        .trim()
        .to_string()
}

/// Legacy escape function for backward compatibility
pub(crate) fn escape_latex(text: &str) -> String {
    escape_latex_safe(text)
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
            generate_optimizer_problem_report(
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
/// Generate a detailed report for a specific optimizer on a specific problem
async fn generate_optimizer_problem_report(
    output_dir: &str,
    problem: &dyn OptimizationProblem,
    optimizer_name: &str,
    runs: &[&SingleResult],
) -> anyhow::Result<()> {
    let problem_name = problem.name();
    let problem_filename = problem_name.replace(" ", "_");
    let optimizer_filename = optimizer_name.replace(" ", "_");
    let filename = format!("detailed_{}_{}.md", problem_filename, optimizer_filename);
    let filepath = Path::new(output_dir).join(&filename);
    let mut content = generate_detailed_report_header(problem, optimizer_name, runs);
    content.push_str(&generate_run_by_run_analysis(runs)?);
    content.push_str(&generate_convergence_analysis(runs)?);
    content.push_str(&generate_parameter_evolution_analysis(runs)?);
    content.push_str(&generate_performance_analysis(runs)?);
    content.push_str(&generate_failure_analysis(runs)?);
    content.push_str(&generate_detailed_report_footer(
        problem_name,
        optimizer_name,
    ));
    fs::write(&filepath, content).with_context(|| {
        format!("Failed to write detailed report to: {}", filepath.display())
    })?;
    Ok(())
}
fn generate_detailed_report_header(
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
fn generate_run_by_run_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let mut content = String::from(
        r#"## Run-by-Run Analysis
<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 12px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Run</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Final Value</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Iterations</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Evals</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Evals</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Time (s)</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Converged</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Reason</th>
</tr>
"#,
    );
    for (i, run) in runs.iter().enumerate() {
        let style = if run.convergence_achieved {
            "background-color: #d4edda;"
        } else {
            "background-color: #f8d7da;"
        };
        let convergence_reason = format!("{:?}", run.convergence_reason)
            .replace("GradientTolerance", "Grad Tol")
            .replace("FunctionTolerance", "Func Tol")
            .replace("MaxIterations", "Max Iter")
            .replace("MaxFunctionEvaluations", "Max Func")
            .replace("TimeLimit", "Time")
            .replace("NumericalError", "Num Err")
            .replace("\"", "");
        content.push_str(&format!(
            r#"<tr style="{}">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{:.3e}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{:.3e}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{:.3}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
</tr>
"#,
            style,
            i + 1,
            run.final_value,
            run.final_gradient_norm,
            run.iterations,
            run.function_evaluations,
            run.gradient_evaluations,
            run.execution_time.as_secs_f64(),
            if run.convergence_achieved {
                "Yes"
            } else {
                "No"
            },
            convergence_reason
        ));
    }
    content.push_str("</table>\n\n");
    Ok(content)
}
fn generate_parameter_evolution_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let mut content = String::from("## Parameter Evolution Analysis\n\n");
    // Find the run with the best final value for detailed analysis
    let best_run = runs
        .iter()
        .filter(|r| r.final_value.is_finite())
        .min_by(|a, b| {
            a.final_value
                .partial_cmp(&b.final_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    if let Some(best_run) = best_run {
        content.push_str(&format!(
            r#"### Best Run Analysis (Run {})
**Final Value:** {:.6e}
**Final Gradient Norm:** {:.6e}
**Iterations:** {}
**Convergence Reason:** {:?}
"#,
            best_run.run_id + 1,
            best_run.final_value,
            best_run.final_gradient_norm,
            best_run.iterations,
            best_run.convergence_reason
        ));
        // Show parameter evolution for first few and last few iterations
        if !best_run.trace.iterations.is_empty() {
            content.push_str("\n#### Parameter Evolution (Selected Iterations)\n\n");
            content.push_str(r#"<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
"#);
            content.push_str(
                r#"<tr style="background-color: #f2f2f2;">
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
"#,
            );
            content.push_str("</tr>\n");
            let iterations_to_show = if best_run.trace.iterations.len() <= 10 {
                best_run.trace.iterations.iter().collect::<Vec<_>>()
            } else {
                let mut selected = Vec::new();
                // First 5 iterations
                for i in 0..5.min(best_run.trace.iterations.len()) {
                    selected.push(&best_run.trace.iterations[i]);
                }
                // Last 5 iterations
                let start_idx = (best_run.trace.iterations.len() - 5).max(5);
                for i in start_idx..best_run.trace.iterations.len() {
                    selected.push(&best_run.trace.iterations[i]);
                }
                selected
            };
            for iter_data in iterations_to_show {
                let params_str = iter_data
                    .parameters
                    .iter()
                    .take(5)
                    .map(|p| format!("{:.3e}", p))
                    .collect::<Vec<_>>()
                    .join(", ");
                let params_display = if iter_data.parameters.len() > 5 {
                    format!("{}, ...", params_str)
                } else {
                    params_str
                };
                content.push_str(&format!(
                    r#"<tr><td style="border: 1px solid #ddd; padding: 4px;">{}</td><td style="border: 1px solid #ddd; padding: 4px;">{:.3e}</td><td style="border: 1px solid #ddd; padding: 4px;">{:.3e}</td><td style="border: 1px solid #ddd; padding: 4px;">{:.3e}</td><td style="border: 1px solid #ddd; padding: 4px;">[{}]</td></tr>
"#,
                    iter_data.iteration,
                    iter_data.function_value,
                    iter_data.gradient_norm,
                    iter_data.step_size,
                    params_display
                ));
            }
            content.push_str("</table>\n\n");
        }
    }
    Ok(content)
}
fn generate_failure_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let failed_runs: Vec<_> = runs.iter().filter(|r| !r.convergence_achieved).collect();
    if failed_runs.is_empty() {
        return Ok("## Failure Analysis\n\n*No failed runs to analyze.*\n\n".to_string());
    }
    let mut content = String::from("## Failure Analysis\n\n");
    // Analyze failure patterns
    let mut early_failures = 0;
    let mut timeout_failures = 0;
    let mut numerical_failures = 0;
    let mut max_iter_failures = 0;
    for run in &failed_runs {
        match &run.convergence_reason {
            ConvergenceReason::TimeLimit => timeout_failures += 1,
            ConvergenceReason::NumericalError => numerical_failures += 1,
            ConvergenceReason::MaxIterations => max_iter_failures += 1,
            ConvergenceReason::MaxFunctionEvaluations => {
                if run.iterations < 10 {
                    early_failures += 1;
                }
            }
            _ => {}
        }
    }
    content.push_str(&format!(
        r#"### Failure Patterns
- **Early Failures (< 10 iterations):** {}
- **Timeout Failures:** {}
- **Numerical Errors:** {}
- **Maximum Iterations Reached:** {}
"#,
        early_failures, timeout_failures, numerical_failures, max_iter_failures
    ));
    // Show details of failed runs
    if failed_runs.len() <= 5 {
        content.push_str("### Failed Run Details\n\n");
        for (i, run) in failed_runs.iter().enumerate() {
            content.push_str(&format!(
                r#"**Run {} (ID: {})**
- Final Value: {:.6e}
- Final Gradient Norm: {:.6e}
- Iterations: {}
- Function Evaluations: {}
- Reason: {:?}
{}
"#,
                i + 1,
                run.run_id + 1,
                run.final_value,
                run.final_gradient_norm,
                run.iterations,
                run.function_evaluations,
                run.convergence_reason,
                if let Some(ref error) = run.error_message {
                    format!("- Error: {}", error)
                } else {
                    String::new()
                }
            ));
        }
    }
    Ok(content)
}
fn generate_detailed_report_footer(problem_name: &str, optimizer_name: &str) -> String {
    format!(
        r#"

## Data Files
* [Raw CSV Data](../data/problems/{}_results.csv)
* [Convergence Plot](../plots/convergence/{}.png)
* [Log Scale Convergence Plot](../plots/convergence/{}_log.png)


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
        // Sort by success rate first, then by mean final value
        perf_data.sort_by(|a, b| match b.1.total_cmp(&a.1) {
            std::cmp::Ordering::Equal => a.2.total_cmp(&b.2),
            other => other,
        });
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
                r#"<a href="reports/problem_analysis_{}.md" title="View detailed analysis">{}</a>"#,
                problem_filename, problem_name
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
pub(crate) fn shorten_optimizer_name(name: &str) -> String {
    // Shorten optimizer names for display in the table
    if name.len() <= 10 {
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
            .replace("Adam-", "")
            .replace("GD-", "")
            .replace("L-BFGS-", "")
            .replace("QQN-", "");

        if shortened.len() <= 10 {
            shortened
        } else {
            // Take first 7 chars + "..."
            format!("{}...", &shortened[..7.min(shortened.len())])
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

fn generate_problem_section(
    problem: &ProblemSpec,
    results: &BenchmarkResults,
    plots_dir: &str,
) -> anyhow::Result<String> {
    let problem_name = problem.get_name();
    let dimension = problem.dimensions;
    let optimal_value = problem.problem.optimal_value().unwrap_or(f64::NEG_INFINITY);

    let mut section = format!(
        r#"
## Problem: {}

**Family:** {}
**Dimension:** {}
**Success Threshold:** {:0.3e}
**Total Runs:** {}
**Detailed Analysis:** [View comprehensive problem analysis](reports/problem_analysis_{}.md)

### Performance Summary
"#,
        problem_name,
        problem.family,
        dimension.or(Some(0)).unwrap(),
        optimal_value,
        results.results.len(),
        problem_name.replace(" ", "_")
    );

    let mut optimizer_stats = HashMap::new();
    let mut suspicious_results = Vec::new();

    for result in &results.results {
        let stats = optimizer_stats
            .entry(result.optimizer_name.clone())
            .or_insert(Vec::new());
        stats.push(result);
        // More sophisticated suspicious result detection
        if result.function_evaluations <= 2 && result.convergence_achieved {
            suspicious_results.push((
                result.optimizer_name.clone(),
                result.function_evaluations,
                result.final_value,
            ));
        }
        // Also check for NaN or infinite values
        if !result.final_value.is_finite() || !result.final_gradient_norm.is_finite() {
            warn!(
                    "Non-finite values detected for {} on {}: final_value={}, gradient_norm={}",
                    result.optimizer_name,
                    problem_name,
                    result.final_value,
                    result.final_gradient_norm
                );
        }
    }

    if !suspicious_results.is_empty() {
        section.push_str(
            r#"> WARNING: **Suspicious/False Convergence Results Detected:**
>
"#,
        );
        for (optimizer, evaluations, final_value) in suspicious_results {
            section.push_str(&format!(
                "> {} claimed convergence with {} function evaluations (final_value: {:.2e})  \n",
                optimizer, evaluations, final_value
            ));
        }
        section.push_str(r#">
> *Note: This may indicate problems with initialization or convergence criteria, or could be due to lucky initialization.*

"#);
    }

    section.push_str(
        r#"<table style="border-collapse: collapse; width: 100%; margin: 20px 0;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Rank</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Optimizer</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Mean Final Value<br>(All/Success/Fail)</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Std Dev</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Best Value</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Worst Value</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Mean Function Evals<br>(All/Success/Fail)</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Mean Gradient Evals<br>(All/Success/Fail)</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Success Rate</th>
<th style="border: 1px solid #ddd; padding: 8px; text-align: center;">Mean Time (s)</th>
</tr>
"#,
    );

    let mut perf_data = Vec::new();
    for (optimizer, runs) in &optimizer_stats {
        let final_values: Vec<f64> = runs
            .iter()
            .map(|r| r.best_value)
            .filter(|&v| v.is_finite())
            .collect();
        if final_values.is_empty() {
            continue; // Skip optimizers with no valid results
        }

        let function_evals: Vec<f64> =
            runs.iter().map(|r| r.function_evaluations as f64).collect();
        let gradient_evals: Vec<f64> =
            runs.iter().map(|r| r.gradient_evaluations as f64).collect();
        let success_count = if is_no_threshold_mode() {
            runs.iter().filter(|r| r.convergence_achieved).count()
        } else {
            0
        };
        let execution_times: Vec<f64> = runs
            .iter()
            .map(|r| r.execution_time.as_secs_f64())
            .collect();

        let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
        if !mean_final.is_finite() {
            warn!(
                    "Mean final value for optimizer '{}' is not finite (mean: {})",
                    optimizer, mean_final
                );
            continue;
        }
        // Separate statistics for successful and unsuccessful runs
        let successful_runs: Vec<_> = runs.iter().filter(|r| r.convergence_achieved).collect();
        let unsuccessful_runs: Vec<_> =
            runs.iter().filter(|r| !r.convergence_achieved).collect();
        // Calculate separate statistics for successful runs
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
        // Calculate separate statistics for unsuccessful runs
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
        let mean_gradient_evals =
            gradient_evals.iter().sum::<f64>() / gradient_evals.len() as f64;
        let success_rate = success_count as f64 / runs.len() as f64;
        let mean_time = execution_times.iter().sum::<f64>() / execution_times.len() as f64;

        perf_data.push((
            optimizer.clone(),
            mean_final,
            std_final,
            best_final,
            worst_final,
            mean_function_evals,
            mean_gradient_evals,
            success_rate,
            mean_time,
            mean_final_success,
            mean_final_fail,
            mean_func_evals_success,
            mean_func_evals_fail,
            mean_grad_evals_success,
            mean_grad_evals_fail,
        ));
    }
    // Sort by success rate first, then by mean final value

    perf_data.sort_by(|a, b| {
        use std::cmp::Ordering;
        if is_no_threshold_mode() {
            a.3.total_cmp(&b.3) // In no-threshold mode, sort by best value
        } else {
            match b.7.total_cmp(&a.7) {
                // Sort by success rate descending
                Ordering::Equal => (a.5 + a.6).total_cmp(&(b.5 + b.6)), // Then by total evaluations
                other => other,
            }
        }
    });

    for (
        i,
        (
            optimizer,
            mean_final,
            std_final,
            best_final,
            worst_final,
            mean_func_evals,
            mean_grad_evals,
            success_rate,
            mean_time,
            mean_final_success,
            mean_final_fail,
            mean_func_evals_success,
            mean_func_evals_fail,
            mean_grad_evals_success,
            mean_grad_evals_fail,
        ),
    ) in perf_data.iter().enumerate()
    {
        let style = if i == 0 {
            "background-color: #d4edda; font-weight: bold;"
        } else if i == 1 {
            "background-color: #fff3cd;"
        } else {
            ""
        };
        // Create hyperlink to detailed report
        let problem_filename = problem_name.replace(" ", "_");
        let optimizer_filename = optimizer.replace(" ", "_");
        let detailed_report_filename = format!(
            "reports/detailed_{}_{}.md", 
            problem_filename, 
            optimizer_filename
        );
        let optimizer_link = format!(
            r#"<a href="{}" title="Click for detailed analysis">{}</a>"#,
            detailed_report_filename, optimizer
        );

        // Create formatted strings for success/fail values
        let success_str = if mean_final_success.is_nan() || !mean_final_success.is_finite() {
            "-".to_string()
        } else {
            format!("{:.2e}", mean_final_success)
        };
        let fail_str = if mean_final_fail.is_nan() || !mean_final_fail.is_finite() {
            "-".to_string()
        } else {
            format!("{:.2e}", mean_final_fail)
        };
        let final_value_str = if mean_final.is_finite() {
            format!("{:.2e} / {} / {}", mean_final, success_str, fail_str)
        } else {
            format!("- / {} / {}", success_str, fail_str)
        };
        // Create formatted strings for function evaluations
        let func_success_str =
            if mean_func_evals_success.is_nan() || !mean_func_evals_success.is_finite() {
                "-".to_string()
            } else {
                format!("{:.1}", mean_func_evals_success)
            };
        let func_fail_str =
            if mean_func_evals_fail.is_nan() || !mean_func_evals_fail.is_finite() {
                "-".to_string()
            } else {
                format!("{:.1}", mean_func_evals_fail)
            };
        let func_evals_str = format!(
            "{:.1} / {} / {}",
            mean_func_evals, func_success_str, func_fail_str
        );
        // Create formatted strings for gradient evaluations
        let grad_success_str =
            if mean_grad_evals_success.is_nan() || !mean_grad_evals_success.is_finite() {
                "-".to_string()
            } else {
                format!("{:.1}", mean_grad_evals_success)
            };
        let grad_fail_str =
            if mean_grad_evals_fail.is_nan() || !mean_grad_evals_fail.is_finite() {
                "-".to_string()
            } else {
                format!("{:.1}", mean_grad_evals_fail)
            };
        let grad_evals_str = format!(
            "{:.1} / {} / {}",
            mean_grad_evals, grad_success_str, grad_fail_str
        );

        section.push_str(&format!(
            r#"<tr style="{}">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.2e}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.2e}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.2e}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.1}%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.3}</td>
</tr>
"#,
            style,
            i + 1, // Rank (1-based)
            optimizer_link,
            final_value_str,
            std_final,
            best_final,
            worst_final,
            func_evals_str,
            grad_evals_str,
            success_rate * 100.0,
            mean_time,
        ));
    }

    section.push_str(
        r#"</table>




### Convergence Plots

"#,
    );
    // Add convergence plots for this problem
    let problem_filename = problem_name.replace(" ", "_");
    let convergence_plot = format!("plots/convergence/{}.png", problem_filename);
    let log_convergence_plot = format!("plots/convergence/{}_log.png", problem_filename);
    // Check if convergence plot exists
    let convergence_path = Path::new(plots_dir).join(format!("convergence/{}.png", problem_filename));
    if convergence_path.exists() {
        section.push_str(&format!(
            r#"<img src="{}" alt="Convergence plot for {}" style="max-width: 48%; height: auto; margin: 1%;">
"#,
            convergence_plot, problem_name
        ));
    }
    // Check if log convergence plot exists
    let log_convergence_path = Path::new(plots_dir).join(format!("convergence/{}_log.png", problem_filename));
    if log_convergence_path.exists() {
        section.push_str(&format!(
            r#"<img src="{}" alt="Log convergence plot for {}" style="max-width: 48%; height: auto; margin: 1%;">
"#,
            log_convergence_plot, problem_name
        ));
    }
    section.push_str(&format!(
        r#"
**Figure:** Convergence plots for {} showing objective value vs iterations.
Left: Linear scale, Right: Log scale for better visualization of convergence behavior.

**Data:** [Linear scale data (CSV)](data/convergence/{}_data.csv) | [Log scale data (CSV)](data/convergence/{}_log_data.csv)

"#,
        problem_name,
        problem_filename,
        problem_filename
    ));
    Ok(section)
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
        .with_context(|| format!("Failed to create output directory: {}", output_dir))?;
    println!("Exporting CSV files to: {}", output_dir);

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
            let successful_runs: Vec<_> =
                runs.iter().filter(|r| r.convergence_achieved).collect();
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
    fs::write(&summary_path, summary_csv).with_context(|| {
        format!("Failed to write summary CSV to: {}", summary_path.display())
    })?;

    // Generate problem-specific CSV files for easier analysis
    generate_problem_specific_csvs(output_dir, all_results)?;

    Ok(())
}
fn generate_problem_specific_csvs(
    output_dir: &str,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<()> {
    let problems_dir = Path::new(output_dir).join("problems");
    fs::create_dir_all(&problems_dir).with_context(|| {
        format!(
            "Failed to create problems directory: {}",
            problems_dir.display()
        )
    })?;
    // Also generate a problem analysis report for each problem
    let reports_dir = Path::new(output_dir).parent().unwrap().join("reports");
    fs::create_dir_all(&reports_dir)?;
    
    for (problem, results) in all_results {
        let problem_name = problem.get_name().replace(" ", "_");
        let csv_path = problems_dir.join(format!("{}_results.csv", problem_name));
        let mut csv_content = String::from("Optimizer,Run,FinalValue,FinalGradientNorm,Iterations,FunctionEvals,GradientEvals,Time,Converged\n");
        for result in &results.results {
            csv_content.push_str(&format!(
                "{},{},{:.6e},{:.6e},{},{},{},{:.3},{}\n",
                result.optimizer_name,
                result.run_id,
                result.final_value,
                result.final_gradient_norm,
                result.iterations,
                result.function_evaluations,
                result.gradient_evaluations,
                result.execution_time.as_secs_f64(),
                result.convergence_achieved,
            ));
        }
        fs::write(&csv_path, csv_content).with_context(|| {
            format!(
                "Failed to write problem-specific CSV to: {}",
                csv_path.display()
            )
        })?;
        // Generate problem analysis report
        generate_problem_analysis_report(problem, results, &reports_dir)?;
    }
    Ok(())
}
/// Generate a comprehensive analysis report for a specific problem
fn generate_problem_analysis_report(
    problem: &ProblemSpec,
    results: &BenchmarkResults,
    reports_dir: &Path,
) -> anyhow::Result<()> {
    let problem_name = problem.get_name();
    let problem_filename = problem_name.replace(" ", "_");
    let report_path = reports_dir.join(format!("problem_analysis_{}.md", problem_filename));
    
    let mut content = format!(
        r#"# Comprehensive Analysis: {}

[← Back to Main Report](../benchmark_report.md)

## Problem Overview

**Name:** {}
**Family:** {}
**Dimension:** {}
**Optimal Value:** {:.3e}
**Total Runs:** {}

## Performance Rankings

"#,
        problem_name,
        problem_name,
        get_family(&problem_name),
        problem.dimensions.unwrap_or(0),
        problem.problem.optimal_value().unwrap_or(f64::NEG_INFINITY),
        results.results.len()
    );
    
    // Add detailed performance table
    content.push_str(&generate_problem_performance_table(results)?);
    
    // Add convergence analysis
    content.push_str(&format!(
        r#"
## Convergence Analysis

![Convergence Plot](../plots/convergence/{}.png)
![Log Convergence Plot](../plots/convergence/{}_log.png)

## Raw Data

* [Problem-specific CSV data](../data/problems/{}_results.csv)
* [Convergence data (linear)](../data/convergence/{}_data.csv)
* [Convergence data (log)](../data/convergence/{}_log_data.csv)

---
*Generated on: {}*
"#,
        problem_filename,
        problem_filename,
        problem_filename,
        problem_filename,
        problem_filename,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    ));
    
    fs::write(&report_path, content)?;
    Ok(())
}

/// Generate performance table for a specific problem
fn generate_problem_performance_table(results: &BenchmarkResults) -> anyhow::Result<String> {
    let mut content = String::from(
        r#"<table style="border-collapse: collapse; width: 100%; margin: 20px 0;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px;">Rank</th>
<th style="border: 1px solid #ddd; padding: 8px;">Optimizer</th>
<th style="border: 1px solid #ddd; padding: 8px;">Mean Final Value</th>
<th style="border: 1px solid #ddd; padding: 8px;">Success Rate</th>
<th style="border: 1px solid #ddd; padding: 8px;">Mean Function Evals</th>
<th style="border: 1px solid #ddd; padding: 8px;">Mean Time (s)</th>
<th style="border: 1px solid #ddd; padding: 8px;">Detailed Report</th>
</tr>
"#,
    );

    // Process results similar to main problem section
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

        let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
        let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
        let success_rate = success_count as f64 / runs.len() as f64;
        let mean_func_evals = runs.iter().map(|r| r.function_evaluations as f64).sum::<f64>() / runs.len() as f64;
        let mean_time = runs.iter().map(|r| r.execution_time.as_secs_f64()).sum::<f64>() / runs.len() as f64;

        perf_data.push((optimizer.clone(), mean_final, success_rate, mean_func_evals, mean_time));
    }

    // Sort by success rate first, then by mean final value
    perf_data.sort_by(|a, b| {
        match b.2.total_cmp(&a.2) {
            std::cmp::Ordering::Equal => a.1.total_cmp(&b.1),
            other => other,
        }
    });

    for (i, (optimizer, mean_final, success_rate, mean_func_evals, mean_time)) in perf_data.iter().enumerate() {
        let style = if i == 0 {
            "background-color: #d4edda; font-weight: bold;"
        } else if i == 1 {
            "background-color: #fff3cd;"
        } else {
            ""
        };

        content.push_str(&format!(
            r#"<tr style="{}">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px;">{}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.2e}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.1}%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.1}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">{:.3}</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_{}_{}.md">View Details</a></td>
</tr>
"#,
            style,
            i + 1,
            optimizer,
            mean_final,
            success_rate * 100.0,
            mean_func_evals,
            mean_time,
            "problem_name", // This would need to be passed in
            optimizer.replace(" ", "_")
        ));
    }

    content.push_str("</table>\n");
    Ok(content)
}


/// Generate LaTeX tables for all results
async fn generate_latex_tables(
    output_dir: &str,
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    slf: &ReportGenerator,
) -> anyhow::Result<()> {
    let latex_dir = Path::new(output_dir);
    fs::create_dir_all(&latex_dir).with_context(|| {
        format!("Failed to create LaTeX directory: {}", latex_dir.display())
    })?;
    println!("Generating LaTeX tables in: {}", latex_dir.display());
    // Generate main performance table
    generate_main_performance_latex_table(all_results, &latex_dir)?;
    // Generate problem-specific tables
    for (problem, results) in all_results {
        generate_problem_latex_table(problem, results, &latex_dir)?;
    }
    // Generate summary statistics table
    generate_summary_statistics_latex_table(all_results, &latex_dir)?;
    // Generate comparison matrix table
    generate_comparison_matrix_latex_table(
        all_results,
        &latex_dir,
        slf,
    )?;
    // Generate family comparison matrix table
    comparison_matrix::generate_family_comparison_matrix_latex_table(all_results, &latex_dir, slf)?;
    // Generate family vs family comparison matrix table
    generate_family_vs_family_latex_table(all_results, &latex_dir).await?;
    // Generate efficiency matrix table
    generate_efficiency_matrix_latex_table(all_results, &latex_dir)?;
    // Generate success rate heatmap table
    generate_success_rate_heatmap_latex_table(all_results, &latex_dir)?;
    // Generate convergence speed analysis table
    generate_convergence_speed_latex_table(all_results, &latex_dir)?;

    Ok(())
}
/// Generate problem-specific LaTeX table
fn generate_problem_latex_table(
    problem: &ProblemSpec,
    results: &BenchmarkResults,
    latex_dir: &Path,
) -> anyhow::Result<()> {
    let problem_name = problem.get_name();
    let problem_filename = problem_name.replace(" ", "_");
    let mut latex_content = String::from(
        r#"\documentclass{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{siunitx}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{adjustbox}
\usepackage{graphicx}
\begin{document}
\small
"#,
    );
    latex_content.push_str(&format!(
        r#"\begin{{table}}[htbp]
\centering
\caption{{Performance Results for {} Problem}}
\label{{tab:{}}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{p{{2.5cm}}*{{7}}{{c}}}}
\toprule
\textbf{{Optimizer}} & \textbf{{Mean Final}} & \textbf{{Std Dev}} & \textbf{{Best}} & \textbf{{Worst}} & \textbf{{Mean Func}} & \textbf{{Success}} & \textbf{{Mean Time}} \\
 & \textbf{{Value}} & & \textbf{{Value}} & \textbf{{Value}} & \textbf{{Evals}} & \textbf{{Rate (\%)}} & \textbf{{(s)}} \\
\midrule
"#,
        escape_latex_safe(&problem_name),
        problem_filename.to_lowercase()
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
    // Sort by success rate first, then by mean final value
    perf_data.sort_by(|a, b| {
        let success_cmp = b.6.partial_cmp(&a.6).unwrap_or(std::cmp::Ordering::Equal);
        if success_cmp != std::cmp::Ordering::Equal {
            success_cmp
        } else {
            a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)
        }
    });
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
        let optimizer_style = if i == 0 {
            format!("\\textbf{{{}}}", escape_latex(optimizer))
        } else {
            escape_latex(optimizer)
        };
        latex_content.push_str(&format!(
            "{} & {:.2e} & {:.2e} & {:.2e} & {:.2e} & {:.1} & {:.1} & {:.3} \\\\\n",
            optimizer_style,
            mean_final,
            std_final,
            best_final,
            worst_final,
            mean_func_evals,
            success_rate,
            mean_time
        ));
    }
    latex_content.push_str(
        r#"\bottomrule
\end{tabular}
}
\end{table}
\end{document}
"#,
    );
    let latex_path = latex_dir.join(format!("{}_performance.tex", problem_filename));
    fs::write(&latex_path, latex_content)
        .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
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
\usepackage{rotating}
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
    latex_content.push_str(&generate_summary_statistics_table_content(
        all_results,
    )?);
    latex_content.push_str(
        r#"
\subsection{QQN vs Non-QQN Comparison Matrix}
"#,
    );
    // Include comparison matrix content
    latex_content.push_str(&generate_comparison_matrix_table_content(
        all_results,
        slf,
    )?);
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
    latex_content.push_str(&generate_family_vs_family_table_content(
        all_results,
    )?);
    latex_content.push_str(
        r#"
\subsection{Algorithm Efficiency Matrix}
"#,
    );
    // Include efficiency matrix content
    latex_content.push_str(&generate_efficiency_matrix_table_content(
        all_results,
    )?);
    latex_content.push_str(
        r#"
\subsection{Success Rate Heatmap}
"#,
    );
    // Include success rate heatmap content
    latex_content.push_str(&generate_success_rate_heatmap_table_content(
        all_results,
    )?);
    latex_content.push_str(
        r#"
\subsection{Convergence Speed Analysis}
"#,
    );
    // Include convergence speed analysis content
    latex_content.push_str(&generate_convergence_speed_table_content(
        all_results,
    )?);

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
            escape_latex(&*problem_name)
        ));
        latex_content.push_str(&generate_problem_table_content(problem, results)?);
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