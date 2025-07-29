use super::experiment_runner;
use super::{StatisticalAnalysis};
use crate::benchmarks::evaluation::{
    is_no_threshold_mode, BenchmarkConfig, BenchmarkResults, ConvergenceReason, ProblemSpec,
    SingleResult,
};
use crate::OptimizationProblem;
use anyhow::Context;
use experiment_runner::get_optimizer_family;
use log::warn;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Data structure for family performance comparison
#[derive(Debug, Clone)]
struct FamilyPerformanceData {
    average_ranking: f64,
    best_rank_average: f64,
    best_variant: String,
    worst_variant: String,
}

/// Handles HTML report generation and CSV exports
pub struct ReportGenerator {
    output_dir: String,
    config: BenchmarkConfig,
    statistical_analysis: StatisticalAnalysis,
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
        println!("Generating report in directory: {}", self.output_dir);

        // Generate detailed optimizer-problem reports first
        Self::generate_detailed_reports(
            &self.output_dir,
            all_results,
            use_optimizer_families,
        )
        .await?;

        let mut html_content = Self::generate_header();
        html_content.push_str(&Self::generate_winner_summary_table(all_results));

        for (problem, results) in all_results {
            html_content.push_str(&Self::generate_problem_section(
                problem,
                results,
                &self.output_dir,
            )?);
        }
        // Add optimizer family vs problem family comparison
        html_content.push_str(&Self::generate_family_vs_family_comparison_table(
            all_results,
        )?);

        if !all_results.is_empty() && all_results.iter().any(|(_, r)| !r.results.is_empty()) {
            html_content.push_str(&self.statistical_analysis.generate_statistical_analysis(
                all_results,
                &self.config,
                &self.output_dir,
                use_optimizer_families,
            )?);
        }

        html_content.push_str(&Self::generate_conclusions(all_results));
        html_content.push_str(&Self::generate_html_footer(&self.config));

        let md_path = Path::new(&self.output_dir).join("benchmark_report.md");
        println!("Saving Markdown report to: {}", md_path.display());
        fs::write(&md_path, html_content.clone()).with_context(|| {
            format!("Failed to write Markdown report to: {}", md_path.display())
        })?;

        Self::generate_csv_exports(&self.output_dir, all_results)?;
        // Generate LaTeX tables
        Self::generate_latex_tables(
            &self.output_dir,
            all_results,
            self,
        )
        .await?;
        // Generate comprehensive LaTeX document
        Self::generate_comprehensive_latex_document(
            &self.config,
            all_results,
            &Path::new(&self.output_dir).join("latex"),
            self,
        )?;

        Ok(())
    }

    fn generate_winner_summary_table(all_results: &[(&ProblemSpec, BenchmarkResults)]) -> String {
        let mut summary = String::from(
            r#"## Quick Summary: Winners by Problem
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
                    problem_name,
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
    fn generate_family_vs_family_comparison_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        let mut content = String::from(
            r#"## Optimizer Family vs Problem Family Performance Matrix
This table shows how different optimizer families perform across different problem families. Each cell contains:
- **Average Ranking**: Mean rank of the optimizer family across all problems in the problem family
- **Best Rank**: Average of the best rank achieved by any variant in the optimizer family for each problem
- **Best Variant**: The specific optimizer variant that achieved the best average rank
- **Worst Variant**: The specific optimizer variant that achieved the worst average rank
"#,
        );
        // Collect all optimizer families and problem families
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
            return Ok("*No data available for family comparison.*\n\n".to_string());
        }
        // Create the table header
        content.push_str(r#"<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px; text-align: center; font-weight: bold;">Problem Family</th>
"#);
        for optimizer_family in &optimizer_families {
            content.push_str(&format!(
                r#"<th style="border: 1px solid #ddd; padding: 8px; text-align: center; font-weight: bold; writing-mode: vertical-lr; text-orientation: mixed;">{}</th>
"#,
                optimizer_family
            ));
        }
        content.push_str("</tr>\n");
        // For each problem family, calculate statistics
        for problem_family in &problem_families {
            content.push_str(&format!(
                r#"<tr>
<td style="border: 1px solid #ddd; padding: 8px; font-weight: bold; background-color: #f8f9fa;">{}</td>
"#,
                problem_family
            ));
            // Get all problems in this family
            let problems_in_family: Vec<_> = all_results
                .iter()
                .filter(|(problem, _)| get_family(&problem.get_name()) == *problem_family)
                .collect();
            for optimizer_family in &optimizer_families {
                let cell_data =
                    Self::calculate_family_performance_data(&problems_in_family, optimizer_family)?;
                let cell_style = if optimizer_family == "QQN" {
                    "border: 1px solid #ddd; padding: 6px; text-align: center; background-color: #e8f5e8; font-size: 10px;"
                } else {
                    "border: 1px solid #ddd; padding: 6px; text-align: center; font-size: 10px;"
                };
                content.push_str(&format!(
                    r#"<td style="{}">
<div><strong>Avg Rank:</strong> {:.1}</div>
<div><strong>Best Rank:</strong> {:.1}</div>
<div><strong>Best Var:</strong> {}</div>
<div><strong>Worst Var:</strong> {}</div>
</td>
"#,
                    cell_style,
                    cell_data.average_ranking,
                    cell_data.best_rank_average,
                    cell_data.best_variant,
                    cell_data.worst_variant
                ));
            }
            content.push_str("</tr>\n");
        }
        content.push_str(
            r#"</table>
**Legend:**
- **Avg Rank**: Average ranking of all variants in the optimizer family across problems in the problem family (lower is better)
- **Best Rank**: Average of the best rank achieved by any variant in the optimizer family for each problem (lower is better)
- **Best Var**: The specific optimizer variant that achieved the best average rank in this problem family
- **Worst Var**: The specific optimizer variant that achieved the worst average rank in this problem family
- QQN family cells are highlighted in green
"#,
        );
        Ok(content)
    }
    fn calculate_family_performance_data(
        problems_in_family: &[&(&ProblemSpec, BenchmarkResults)],
        optimizer_family: &str,
    ) -> anyhow::Result<FamilyPerformanceData> {
        let mut all_rankings = Vec::new();
        let mut best_ranks_per_problem = Vec::new();
        let mut variant_performance = std::collections::HashMap::new();
        for (_, results) in problems_in_family {
            // Calculate rankings for this problem
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
                    .map(|r| r.best_value)
                    .filter(|&v| v.is_finite())
                    .collect();
                if final_values.is_empty() {
                    continue;
                }
                let success_count = if is_no_threshold_mode() {
                    runs.iter().filter(|r| r.convergence_achieved).count()
                } else {
                    0
                };
                let success_rate = success_count as f64 / runs.len() as f64;
                let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
                let mean_func_evals = runs
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;
                let mean_grad_evals = runs
                    .iter()
                    .map(|r| r.gradient_evaluations as f64)
                    .sum::<f64>()
                    / runs.len() as f64;
                perf_data.push((
                    optimizer.clone(),
                    success_rate,
                    mean_final,
                    mean_func_evals + mean_grad_evals,
                ));
            }
            // Sort by success rate first, then by mean final value, then by total evaluations
            perf_data.sort_by(|a, b| {
                if is_no_threshold_mode() {
                    // In no-threshold mode, sort by best value, then by total evaluations
                    match a.2.total_cmp(&b.2) {
                        std::cmp::Ordering::Equal => a.3.total_cmp(&b.3),
                        ord => ord,
                    }
                } else {
                    match b.1.total_cmp(&a.1) {
                        std::cmp::Ordering::Equal => match a.2.total_cmp(&b.2) {
                            std::cmp::Ordering::Equal => a.3.total_cmp(&b.3),
                            ord => ord,
                        },
                        ord => ord,
                    }
                }
            });
            // Assign rankings and collect data for this optimizer family
            let mut family_ranks_this_problem = Vec::new();
            let mut best_rank_this_problem = f64::INFINITY;
            for (rank, (optimizer, _, mean_final, _)) in perf_data.iter().enumerate() {
                let current_family = get_optimizer_family(optimizer);
                let rank_value = (rank + 1) as f64;
                if current_family == *optimizer_family {
                    all_rankings.push(rank_value);
                    family_ranks_this_problem.push(rank_value);
                    best_rank_this_problem = best_rank_this_problem.min(rank_value);

                    // Track individual variant performance
                    variant_performance
                        .entry(optimizer.clone())
                        .or_insert_with(Vec::new)
                        .push((rank_value, *mean_final));
                }
            }
            if best_rank_this_problem.is_finite() {
                best_ranks_per_problem.push(best_rank_this_problem);
            }
        }
        // Calculate averages
        let average_ranking = if !all_rankings.is_empty() {
            all_rankings.iter().sum::<f64>() / all_rankings.len() as f64
        } else {
            f64::INFINITY
        };
        let best_rank_average = if !best_ranks_per_problem.is_empty() {
            best_ranks_per_problem.iter().sum::<f64>() / best_ranks_per_problem.len() as f64
        } else {
            f64::INFINITY
        };
        // Find best and worst variants
        let mut variant_averages = Vec::new();
        for (variant, ranks_and_values) in variant_performance {
            if !ranks_and_values.is_empty() {
                let avg_rank = ranks_and_values.iter().map(|(rank, _)| *rank).sum::<f64>()
                    / ranks_and_values.len() as f64;
                variant_averages.push((variant, avg_rank));
            }
        }
        variant_averages.sort_by(|a, b| a.1.total_cmp(&b.1));
        let best_variant = variant_averages
            .first()
            .map(|(name, _)| Self::shorten_optimizer_name(name))
            .unwrap_or_else(|| "N/A".to_string());
        let worst_variant = variant_averages
            .last()
            .map(|(name, _)| Self::shorten_optimizer_name(name))
            .unwrap_or_else(|| "N/A".to_string());
        Ok(FamilyPerformanceData {
            average_ranking,
            best_rank_average,
            best_variant,
            worst_variant,
        })
    }
    fn shorten_optimizer_name(name: &str) -> String {
        // Shorten optimizer names for display in the table
        if name.len() <= 12 {
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
                .replace("Quadratic", "Quad");

            if shortened.len() <= 12 {
                shortened
            } else {
                // Take first 9 chars + "..."
                format!("{}...", &shortened[..9.min(shortened.len())])
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
        output_dir: &str,
    ) -> anyhow::Result<String> {
        let problem_name = problem.get_name();
        let dimension = problem.dimensions;
        let optimal_value = problem.problem.optimal_value().unwrap_or(f64::NEG_INFINITY);

        let mut section = format!(
            r#"## Problem: {}

**Family:** {}  
**Dimension:** {}  
**Success Threshold:** {:0.3e}  
**Total Runs:** {}

### Performance Summary
"#,
            problem_name,
            problem.family,
            dimension.or(Some(0)).unwrap(),
            optimal_value,
            results.results.len()
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
            let detailed_report_filename =
                format!("detailed_{}_{}.md", problem_filename, optimizer_filename);
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
        let convergence_plot = format!("convergence_{}.png", problem_filename);
        let log_convergence_plot = format!("convergence_{}_log.png", problem_filename);
        // Check if convergence plot exists
        let convergence_path = Path::new(output_dir).join(&convergence_plot);
        if convergence_path.exists() {
            section.push_str(&format!(
                r#"<img src="{}" alt="Convergence plot for {}" style="max-width: 48%; height: auto; margin: 1%;">
"#,
                convergence_plot, problem_name
            ));
        }
        // Check if log convergence plot exists
        let log_convergence_path = Path::new(output_dir).join(&log_convergence_plot);
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

**Data:** [Linear scale data (CSV)](convergence_{}_data.csv) | [Log scale data (CSV)](convergence_{}_log_data.csv)

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
        Self::generate_problem_specific_csvs(output_dir, all_results)?;

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
        }
        Ok(())
    }
    /// Generate LaTeX tables for all results
    async fn generate_latex_tables(
        output_dir: &str,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        slf: &ReportGenerator,
    ) -> anyhow::Result<()> {
        let latex_dir = Path::new(output_dir).join("latex");
        fs::create_dir_all(&latex_dir).with_context(|| {
            format!("Failed to create LaTeX directory: {}", latex_dir.display())
        })?;
        println!("Generating LaTeX tables in: {}", latex_dir.display());
        // Generate main performance table
        Self::generate_main_performance_latex_table(all_results, &latex_dir)?;
        // Generate problem-specific tables
        for (problem, results) in all_results {
            Self::generate_problem_latex_table(problem, results, &latex_dir)?;
        }
        // Generate summary statistics table
        Self::generate_summary_statistics_latex_table(all_results, &latex_dir)?;
        // Generate comparison matrix table
        Self::generate_comparison_matrix_latex_table(
            all_results,
            &latex_dir,
            slf,
        )?;
        // Generate family comparison matrix table
        Self::generate_family_comparison_matrix_latex_table(slf, all_results, &latex_dir)?;
        // Generate family vs family comparison matrix table
        Self::generate_family_vs_family_latex_table(all_results, &latex_dir).await?;
        // Generate efficiency matrix table
        Self::generate_efficiency_matrix_latex_table(all_results, &latex_dir)?;
        // Generate success rate heatmap table
        Self::generate_success_rate_heatmap_latex_table(all_results, &latex_dir)?;
        // Generate convergence speed analysis table
        Self::generate_convergence_speed_latex_table(slf, all_results, &latex_dir)?;

        Ok(())
    }
    /// Generate main performance LaTeX table
    fn generate_main_performance_latex_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
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
\usepackage{rotating}
\usepackage{graphicx}
\begin{document}
\tiny
\begin{adjustbox}{width=\textwidth,center}
\begin{longtable}{p{2cm}p{2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}p{1.2cm}}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endhead
\midrule \multicolumn{9}{r}{{Continued on next page}} \\ \midrule
\endfoot
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
                let problem_cell = if i == 0 {
                    format!(
                        "\\multirow{{{}}}{{*}}{{{}}}",
                        perf_data.len(),
                        Self::escape_latex(&problem_name)
                    )
                } else {
                    String::new()
                };
                let optimizer_style = if i == 0 {
                    format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
                } else {
                    Self::escape_latex(optimizer)
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
            }
            if !perf_data.is_empty() {
                latex_content.push_str("\\midrule\n");
            }
        }
        latex_content.push_str(
            r#"\end{longtable}
\end{adjustbox}
\end{document}
"#,
        );
        let latex_path = latex_dir.join("main_performance_table.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
        println!(
            "Generated main performance LaTeX table: {}",
            latex_path.display()
        );
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
            Self::escape_latex_safe(&problem_name),
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
                format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
            } else {
                Self::escape_latex(optimizer)
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
    /// Generate summary statistics LaTeX table
    fn generate_summary_statistics_latex_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{siunitx}
\usepackage{multirow}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{adjustbox}
\usepackage{graphicx}
\begin{document}
\small
\begin{table}[htbp]
\centering
\caption{Summary Statistics by Problem Family}
\label{tab:summary_statistics}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{p{2.5cm}p{2.5cm}*{5}{c}}
\toprule
\textbf{Problem Family} & \textbf{Optimizer} & \textbf{Avg Success} & \textbf{Avg Final} & \textbf{Avg Func} & \textbf{Avg Grad} & \textbf{Avg Time} \\
 & & \textbf{Rate (\%)} & \textbf{Value} & \textbf{Evals} & \textbf{Evals} & \textbf{(s)} \\
\midrule
"#,
        );
        // Group by problem family
        let mut family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> =
            HashMap::new();
        for (problem, results) in all_results {
            let family = get_family(&problem.get_name());
            for result in &results.results {
                family_results
                    .entry(family.clone())
                    .or_insert_with(HashMap::new)
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        let mut families: Vec<_> = family_results.keys().cloned().collect();
        families.sort();
        for family in families {
            if let Some(optimizers) = family_results.get(&family) {
                let mut optimizer_data = Vec::new();
                for (optimizer, runs) in optimizers {
                    let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                    let success_rate = success_count as f64 / runs.len() as f64 * 100.0;
                    let final_values: Vec<f64> = runs
                        .iter()
                        .map(|r| r.final_value)
                        .filter(|&v| v.is_finite())
                        .collect();
                    let avg_final = if !final_values.is_empty() {
                        final_values.iter().sum::<f64>() / final_values.len() as f64
                    } else {
                        f64::INFINITY
                    };
                    let avg_func_evals = runs
                        .iter()
                        .map(|r| r.function_evaluations as f64)
                        .sum::<f64>()
                        / runs.len() as f64;
                    let avg_grad_evals = runs
                        .iter()
                        .map(|r| r.gradient_evaluations as f64)
                        .sum::<f64>()
                        / runs.len() as f64;
                    let avg_time = runs
                        .iter()
                        .map(|r| r.execution_time.as_secs_f64())
                        .sum::<f64>()
                        / runs.len() as f64;
                    optimizer_data.push((
                        optimizer.clone(),
                        success_rate,
                        avg_final,
                        avg_func_evals,
                        avg_grad_evals,
                        avg_time,
                    ));
                }
                // Sort by success rate
                optimizer_data.sort_by(|a, b| b.1.total_cmp(&a.1));
                for (
                    i,
                    (optimizer, success_rate, avg_final, avg_func_evals, avg_grad_evals, avg_time),
                ) in optimizer_data.iter().enumerate()
                {
                    let family_cell = if i == 0 {
                        format!(
                            "\\multirow{{{}}}{{*}}{{{}}}",
                            optimizer_data.len(),
                            Self::escape_latex(&family)
                        )
                    } else {
                        String::new()
                    };
                    let optimizer_style = if i == 0 {
                        format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
                    } else {
                        Self::escape_latex(optimizer)
                    };
                    latex_content.push_str(&format!(
                        "{} & {} & {:.1} & {:.2e} & {:.1} & {:.1} & {:.3} \\\\\n",
                        family_cell,
                        optimizer_style,
                        success_rate,
                        avg_final,
                        avg_func_evals,
                        avg_grad_evals,
                        avg_time
                    ));
                }
                if !optimizer_data.is_empty() {
                    latex_content.push_str("\\midrule\n");
                }
            }
        }
        latex_content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\end{document}
"#,
        );
        let latex_path = latex_dir.join("summary_statistics.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
        println!(
            "Generated summary statistics LaTeX table: {}",
            latex_path.display()
        );
        Ok(())
    }
    /// Generate comparison matrix LaTeX table
    fn generate_comparison_matrix_latex_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
        slf: &ReportGenerator,
    ) -> anyhow::Result<()> {
        // Collect all optimizers
        let mut all_optimizers = std::collections::HashSet::new();
        for (_, results) in all_results {
            for result in &results.results {
                all_optimizers.insert(result.optimizer_name.clone());
            }
        }
        let mut qqn_optimizers = Vec::new();
        let mut non_qqn_optimizers = Vec::new();
        for optimizer in all_optimizers {
            if optimizer.contains("QQN") {
                qqn_optimizers.push(optimizer);
            } else {
                non_qqn_optimizers.push(optimizer);
            }
        }
        if qqn_optimizers.is_empty() || non_qqn_optimizers.is_empty() {
            return Ok(());
        }
        qqn_optimizers.sort();
        non_qqn_optimizers.sort();
        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage[margin=0.5in]{geometry}
\usepackage{booktabs}
\usepackage{array}
\usepackage{colortbl}
\usepackage{xcolor}
\usepackage{multirow}
\usepackage{adjustbox}
\usepackage{graphicx}
\begin{document}
\tiny
"#,
        );
        // Calculate column specification dynamically
        let col_spec = format!("l{}", "c".repeat(non_qqn_optimizers.len()));

        latex_content.push_str(&format!(
            r#"\begin{{table}}[htbp]
\centering
\caption{{QQN vs Non-QQN Optimizer Comparison Matrix}}
\label{{tab:comparison_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{col_spec}}}
\toprule
\textbf{{QQN Optimizer}} {}\\ 
\midrule
"#,
            non_qqn_optimizers
                .iter()
                .map(|opt| format!("& \\textbf{{{}}}", Self::escape_latex_safe(opt)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        // Group results by problem for comparison
        let mut problem_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> =
            HashMap::new();
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            for result in &results.results {
                problem_results
                    .entry(problem_name.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        for qqn_opt in &qqn_optimizers {
            latex_content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex_safe(qqn_opt)
            ));
            for non_qqn_opt in &non_qqn_optimizers {
                let mut wins = 0;
                let mut losses = 0;
                let mut ties = 0;
                for (_, optimizers) in &problem_results {
                    if let (Some(qqn_results), Some(non_qqn_results)) =
                        (optimizers.get(qqn_opt), optimizers.get(non_qqn_opt))
                    {
                        if qqn_results.len() >= 2 && non_qqn_results.len() >= 2 {
                            let qqn_final_values: Vec<f64> = qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            let non_qqn_final_values: Vec<f64> = non_qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            if !qqn_final_values.is_empty() && !non_qqn_final_values.is_empty() {
                                let qqn_mean = qqn_final_values.iter().sum::<f64>()
                                    / qqn_final_values.len() as f64;
                                let non_qqn_mean = non_qqn_final_values.iter().sum::<f64>()
                                    / non_qqn_final_values.len() as f64;
                                if let Ok((_, p_value)) = slf
                                    .statistical_analysis
                                    .welch_t_test_public(&qqn_final_values, &non_qqn_final_values)
                                {
                                    if p_value < 0.05 {
                                        if qqn_mean < non_qqn_mean {
                                            wins += 1;
                                        } else {
                                            losses += 1;
                                        }
                                    } else {
                                        ties += 1;
                                    }
                                } else {
                                    ties += 1;
                                }
                            }
                        }
                    }
                }
                let cell_content = if wins > losses {
                    format!(
                        "\\textcolor{{green!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else if losses > wins {
                    format!(
                        "\\textcolor{{red!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else {
                    format!("{}W-{}L-{}T", wins, losses, ties)
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
\textbf{Legend:} W = Wins (statistically significant better performance), L = Losses (statistically significant worse performance), T = Ties (no significant difference). Green indicates QQN variant dominance, red indicates non-QQN dominance.
\end{document}
"#,
        );
        let latex_path = latex_dir.join("comparison_matrix.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
        println!(
            "Generated comparison matrix LaTeX table: {}",
            latex_path.display()
        );
        Ok(())
    }
    /// Generate family comparison matrix LaTeX table
    fn generate_family_comparison_matrix_latex_table(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
        // Collect all optimizer families
        let mut all_families = std::collections::HashSet::new();
        for (_, results) in all_results {
            for result in &results.results {
                let family = get_optimizer_family(&result.optimizer_name);
                all_families.insert(family);
            }
        }
        let mut qqn_families = Vec::new();
        let mut non_qqn_families = Vec::new();
        for family in all_families {
            if family == "QQN" {
                qqn_families.push(family);
            } else {
                non_qqn_families.push(family);
            }
        }
        if qqn_families.is_empty() || non_qqn_families.is_empty() {
            return Ok(());
        }
        qqn_families.sort();
        non_qqn_families.sort();
        // Calculate column specification dynamically
        let col_spec = format!("l{}", "c".repeat(non_qqn_families.len()));

        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage{xcolor}
\usepackage{multirow}
\usepackage{adjustbox}
\usepackage[margin=1in]{geometry}
\begin{document}
"#,
        );
        latex_content.push_str(&format!(
            r#"\begin{{table}}[htbp]
\centering
\caption{{Optimizer Family Comparison Matrix}}
\label{{tab:family_comparison_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{col_spec}}}
\toprule
\textbf{{QQN Family}} {}\\ 
\midrule
"#,
            non_qqn_families
                .iter()
                .map(|fam| format!("& \\textbf{{{}}}", Self::escape_latex(fam)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        // Group results by problem and family for comparison
        let mut problem_family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> =
            HashMap::new();
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            for result in &results.results {
                let family = get_optimizer_family(&result.optimizer_name);
                problem_family_results
                    .entry(problem_name.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(family)
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        for qqn_fam in &qqn_families {
            latex_content.push_str(&format!("\\textbf{{{}}} ", Self::escape_latex(qqn_fam)));
            for non_qqn_fam in &non_qqn_families {
                let mut wins = 0;
                let mut losses = 0;
                let mut ties = 0;
                for (_, families) in &problem_family_results {
                    if let (Some(qqn_results), Some(non_qqn_results)) =
                        (families.get(qqn_fam), families.get(non_qqn_fam))
                    {
                        if qqn_results.len() >= 2 && non_qqn_results.len() >= 2 {
                            let qqn_final_values: Vec<f64> = qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            let non_qqn_final_values: Vec<f64> = non_qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            if !qqn_final_values.is_empty() && !non_qqn_final_values.is_empty() {
                                let qqn_mean = qqn_final_values.iter().sum::<f64>()
                                    / qqn_final_values.len() as f64;
                                let non_qqn_mean = non_qqn_final_values.iter().sum::<f64>()
                                    / non_qqn_final_values.len() as f64;
                                if let Ok((_, p_value)) = self
                                    .statistical_analysis
                                    .welch_t_test_public(&qqn_final_values, &non_qqn_final_values)
                                {
                                    if p_value < 0.05 {
                                        if qqn_mean < non_qqn_mean {
                                            wins += 1;
                                        } else {
                                            losses += 1;
                                        }
                                    } else {
                                        ties += 1;
                                    }
                                } else {
                                    ties += 1;
                                }
                            }
                        }
                    }
                }
                let cell_content = if wins > losses {
                    format!(
                        "\\textcolor{{green!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else if losses > wins {
                    format!(
                        "\\textcolor{{red!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else {
                    format!("{}W-{}L-{}T", wins, losses, ties)
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
\textbf{Legend:} W = Wins (statistically significant better performance), L = Losses (statistically significant worse performance), T = Ties (no significant difference). Green indicates QQN family dominance, red indicates non-QQN family dominance.
\end{document}
"#,
        );
        let latex_path = latex_dir.join("family_comparison_matrix.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
        println!(
            "Generated family comparison matrix LaTeX table: {}",
            latex_path.display()
        );
        Ok(())
    }
    /// Generate family vs family comparison LaTeX table
    async fn generate_family_vs_family_latex_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
        // Collect all optimizer families and problem families
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
            return Ok(());
        }
        // Calculate column specification dynamically
        let col_spec = format!("l{}", "c".repeat(optimizer_families.len()));

        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage{multirow}
\usepackage{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage{rotating}
\usepackage[margin=1in]{geometry}
\begin{document}
"#,
        );
        latex_content.push_str(&format!(
            r#"\begin{{table}}[htbp]
\centering
\caption{{Optimizer Family vs Problem Family Performance Matrix}}
\label{{tab:family_vs_family_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{col_spec}}}
\toprule
\textbf{{Problem Family}} {}\\ 
\midrule
"#,
            optimizer_families
                .iter()
                .map(|fam| format!(
                    "& \\rotatebox{{90}}{{\\textbf{{{}}}}}",
                    Self::escape_latex(fam)
                ))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        // For each problem family, calculate statistics
        for problem_family in &problem_families {
            latex_content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex(problem_family)
            ));
            // Get all problems in this family
            let problems_in_family: Vec<_> = all_results
                .iter()
                .filter(|(problem, _)| get_family(&problem.get_name()) == *problem_family)
                .collect();
            for optimizer_family in &optimizer_families {
                let cell_data =
                    Self::calculate_family_performance_data(&problems_in_family, optimizer_family)?;
                let cell_content =                     format!(
                    "& \\begin{{tabular}}{{@{{}}c@{{}}}} {:.1} \\\\ {:.1} \\\\ \\tiny{{{}}} \\\\ \\tiny{{{}}} \\end{{tabular}}",
                    cell_data.average_ranking,
                    cell_data.best_rank_average,
                    Self::escape_latex(&cell_data.best_variant),
                    Self::escape_latex(&cell_data.worst_variant)
                )
                    ;
                latex_content.push_str(&cell_content);
            }
            latex_content.push_str(" \\\\\n");
        }
        latex_content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Legend:} Each cell contains four values arranged vertically:
\begin{itemize}
\item \textbf{Top line:} Average Ranking across all problems in the family (lower is better)
\item \textbf{Second line:} Best Rank Average - average of best ranks achieved (lower is better)
\item \textbf{Third line:} Best Variant - optimizer that achieved the best average rank
\item \textbf{Bottom line:} Worst Variant - optimizer that achieved the worst average rank
\end{itemize}
QQN family cells are highlighted in green for easy identification.
\end{document}
"#,
        );
        let latex_path = latex_dir.join("family_vs_family_matrix.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
        println!(
            "Generated family vs family comparison LaTeX table: {}",
            latex_path.display()
        );
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
        latex_content.push_str(&Self::generate_main_performance_table_content(all_results)?);
        latex_content.push_str(
            r#"
\subsection{Summary Statistics by Problem Family}
"#,
        );
        // Include summary statistics table content
        latex_content.push_str(&Self::generate_summary_statistics_table_content(
            all_results,
        )?);
        latex_content.push_str(
            r#"
\subsection{QQN vs Non-QQN Comparison Matrix}
"#,
        );
        // Include comparison matrix content
        latex_content.push_str(&Self::generate_comparison_matrix_table_content(
            all_results,
            slf,
        )?);
        latex_content.push_str(
            r#"
\subsection{Optimizer Family Comparison Matrix}
"#,
        );
        // Include family comparison matrix content
        latex_content.push_str(&Self::generate_family_comparison_matrix_table_content(
            all_results,
            slf,
        )?);
        latex_content.push_str(
            r#"
\subsection{Optimizer Family vs Problem Family Performance Matrix}
"#,
        );
        // Include family vs family comparison matrix content
        latex_content.push_str(&Self::generate_family_vs_family_table_content(
            all_results,
        )?);
        latex_content.push_str(
            r#"
\subsection{Algorithm Efficiency Matrix}
"#,
        );
        // Include efficiency matrix content
        latex_content.push_str(&Self::generate_efficiency_matrix_table_content(
            all_results,
        )?);
        latex_content.push_str(
            r#"
\subsection{Success Rate Heatmap}
"#,
        );
        // Include success rate heatmap content
        latex_content.push_str(&Self::generate_success_rate_heatmap_table_content(
            all_results,
        )?);
        latex_content.push_str(
            r#"
\subsection{Convergence Speed Analysis}
"#,
        );
        // Include convergence speed analysis content
        latex_content.push_str(&Self::generate_convergence_speed_table_content(
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
                Self::escape_latex(&*problem_name)
            ));
            latex_content.push_str(&Self::generate_problem_table_content(problem, results)?);
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
    /// Generate main performance table content (without document wrapper)
    fn generate_main_performance_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        let mut content = String::from(
            r#"\tiny
\begin{adjustbox}{width=\textwidth,center}
\begin{longtable}{p{2cm}p{2cm}*{7}{c}}
\caption{Comprehensive Performance Comparison of Optimization Algorithms} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endfirsthead
\multicolumn{9}{c}%
{{\bfseries \tablename\ \thetable{} -- continued from previous page}} \\
\toprule
\textbf{Problem} & \textbf{Optimizer} & \textbf{Mean Final} & \textbf{Std Dev} & \textbf{Best} & \textbf{Worst} & \textbf{Mean Func} & \textbf{Success} & \textbf{Mean Time} \\
 & & \textbf{Value} & & \textbf{Value} & \textbf{Value} & \textbf{Evals} & \textbf{Rate (\%)} & \textbf{(s)} \\
\midrule
\endhead
\midrule \multicolumn{9}{r}{{Continued on next page}} \\ \midrule
\endfoot
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
                let problem_cell = if i == 0 {
                    format!(
                        "\\multirow{{{}}}{{*}}{{{}}}",
                        perf_data.len(),
                        Self::escape_latex(&problem_name)
                    )
                } else {
                    String::new()
                };
                let optimizer_style = if i == 0 {
                    format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
                } else {
                    Self::escape_latex(optimizer)
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
            }
            if !perf_data.is_empty() {
                content.push_str("\\midrule\n");
            }
        }
        content.push_str("\\end{longtable}\n}\n");
        content.push_str("\\end{adjustbox}\n");
        Ok(content)
    }
    /// Generate summary statistics table content (without document wrapper)
    fn generate_summary_statistics_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        // Similar implementation as generate_summary_statistics_latex_table but return just the table content
        let mut content = String::from(
            r#"\begin{table}[H]
\centering
\caption{Summary Statistics by Problem Family}
\label{tab:summary_statistics}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{p{{2.5cm}}p{{2.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}p{{1.5cm}}}
\toprule
\textbf{Problem Family} & \textbf{Optimizer} & \textbf{Avg Success} & \textbf{Avg Final} & \textbf{Avg Func} & \textbf{Avg Grad} & \textbf{Avg Time} \\
 & & \textbf{Rate (\%)} & \textbf{Value} & \textbf{Evals} & \textbf{Evals} & \textbf{(s)} \\
\midrule
"#,
        );
        // Group by problem family (same logic as before)
        let mut family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> =
            HashMap::new();
        for (problem, results) in all_results {
            let family = get_family(&problem.get_name());
            for result in &results.results {
                family_results
                    .entry(family.clone())
                    .or_insert_with(HashMap::new)
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        let mut families: Vec<_> = family_results.keys().cloned().collect();
        families.sort();
        for family in families {
            if let Some(optimizers) = family_results.get(&family) {
                let mut optimizer_data = Vec::new();
                for (optimizer, runs) in optimizers {
                    let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
                    let success_rate = success_count as f64 / runs.len() as f64 * 100.0;
                    let final_values: Vec<f64> = runs
                        .iter()
                        .map(|r| r.final_value)
                        .filter(|&v| v.is_finite())
                        .collect();
                    let avg_final = if !final_values.is_empty() {
                        final_values.iter().sum::<f64>() / final_values.len() as f64
                    } else {
                        f64::INFINITY
                    };
                    let avg_func_evals = runs
                        .iter()
                        .map(|r| r.function_evaluations as f64)
                        .sum::<f64>()
                        / runs.len() as f64;
                    let avg_grad_evals = runs
                        .iter()
                        .map(|r| r.gradient_evaluations as f64)
                        .sum::<f64>()
                        / runs.len() as f64;
                    let avg_time = runs
                        .iter()
                        .map(|r| r.execution_time.as_secs_f64())
                        .sum::<f64>()
                        / runs.len() as f64;
                    optimizer_data.push((
                        optimizer.clone(),
                        success_rate,
                        avg_final,
                        avg_func_evals,
                        avg_grad_evals,
                        avg_time,
                    ));
                }
                optimizer_data
                    .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                for (
                    i,
                    (optimizer, success_rate, avg_final, avg_func_evals, avg_grad_evals, avg_time),
                ) in optimizer_data.iter().enumerate()
                {
                    let family_cell = if i == 0 {
                        format!(
                            "\\multirow{{{}}}{{*}}{{{}}}",
                            optimizer_data.len(),
                            Self::escape_latex(&family)
                        )
                    } else {
                        String::new()
                    };
                    let optimizer_style = if i == 0 {
                        format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
                    } else {
                        Self::escape_latex(optimizer)
                    };
                    content.push_str(&format!(
                        "{} & {} & {:.1} & {:.2e} & {:.1} & {:.1} & {:.3} \\\\\n",
                        family_cell,
                        optimizer_style,
                        success_rate,
                        avg_final,
                        avg_func_evals,
                        avg_grad_evals,
                        avg_time
                    ));
                }
                if !optimizer_data.is_empty() {
                    content.push_str("\\midrule\n");
                }
            }
        }
        content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
"#,
        );
        Ok(content)
    }
    /// Generate comparison matrix table content (without document wrapper)
    fn generate_comparison_matrix_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        slf: &ReportGenerator,
    ) -> anyhow::Result<String> {
        // Similar logic as generate_comparison_matrix_latex_table but return just the table content
        let mut all_optimizers = std::collections::HashSet::new();
        for (_, results) in all_results {
            for result in &results.results {
                all_optimizers.insert(result.optimizer_name.clone());
            }
        }
        let mut qqn_optimizers = Vec::new();
        let mut non_qqn_optimizers = Vec::new();
        for optimizer in all_optimizers {
            if optimizer.contains("QQN") {
                qqn_optimizers.push(optimizer);
            } else {
                non_qqn_optimizers.push(optimizer);
            }
        }
        if qqn_optimizers.is_empty() || non_qqn_optimizers.is_empty() {
            return Ok(String::new());
        }
        qqn_optimizers.sort();
        non_qqn_optimizers.sort();

        let mut content = format!(
            r#"\begin{{table}}[H]
\centering
\caption{{QQN vs Non-QQN Optimizer Comparison Matrix}}
\label{{tab:comparison_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{l{}}}
\toprule
\textbf{{QQN Optimizer}} {}\\ 
\midrule
"#,
            "c".repeat(non_qqn_optimizers.len()),
            non_qqn_optimizers
                .iter()
                .map(|opt| format!("& \\textbf{{{}}}", Self::escape_latex(opt)))
                .collect::<Vec<_>>()
                .join(" ")
        );
        // Same comparison logic as before...
        let mut problem_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> =
            HashMap::new();
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            for result in &results.results {
                problem_results
                    .entry(problem_name.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        for qqn_opt in &qqn_optimizers {
            content.push_str(&format!("\\textbf{{{}}} ", Self::escape_latex(qqn_opt)));
            for non_qqn_opt in &non_qqn_optimizers {
                let mut wins = 0;
                let mut losses = 0;
                let mut ties = 0;
                for (_, optimizers) in &problem_results {
                    if let (Some(qqn_results), Some(non_qqn_results)) =
                        (optimizers.get(qqn_opt), optimizers.get(non_qqn_opt))
                    {
                        if qqn_results.len() >= 2 && non_qqn_results.len() >= 2 {
                            let qqn_final_values: Vec<f64> = qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            let non_qqn_final_values: Vec<f64> = non_qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            if !qqn_final_values.is_empty() && !non_qqn_final_values.is_empty() {
                                let qqn_mean = qqn_final_values.iter().sum::<f64>()
                                    / qqn_final_values.len() as f64;
                                let non_qqn_mean = non_qqn_final_values.iter().sum::<f64>()
                                    / non_qqn_final_values.len() as f64;
                                if let Ok((_, p_value)) = slf
                                    .statistical_analysis
                                    .welch_t_test_public(&qqn_final_values, &non_qqn_final_values)
                                {
                                    if p_value < 0.05 {
                                        if qqn_mean < non_qqn_mean {
                                            wins += 1;
                                        } else {
                                            losses += 1;
                                        }
                                    } else {
                                        ties += 1;
                                    }
                                } else {
                                    ties += 1;
                                }
                            }
                        }
                    }
                }
                let cell_content = if wins > losses {
                    format!(
                        "\\textcolor{{green!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else if losses > wins {
                    format!(
                        "\\textcolor{{red!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else {
                    format!("{}W-{}L-{}T", wins, losses, ties)
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
\textbf{Legend:} W = Wins (statistically significant better performance), L = Losses (statistically significant worse performance), T = Ties (no significant difference). Green indicates QQN variant dominance, red indicates non-QQN dominance.
"#,
        );
        Ok(content)
    }
    /// Generate family comparison matrix table content (without document wrapper)
    fn generate_family_comparison_matrix_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        slf: &ReportGenerator,
    ) -> anyhow::Result<String> {
        // Collect all optimizer families
        let mut all_families = std::collections::HashSet::new();
        for (_, results) in all_results {
            for result in &results.results {
                let family = get_optimizer_family(&result.optimizer_name);
                all_families.insert(family);
            }
        }
        let mut qqn_families = Vec::new();
        let mut non_qqn_families = Vec::new();
        for family in all_families {
            if family == "QQN" {
                qqn_families.push(family);
            } else {
                non_qqn_families.push(family);
            }
        }
        if qqn_families.is_empty() || non_qqn_families.is_empty() {
            return Ok(String::new());
        }
        qqn_families.sort();
        non_qqn_families.sort();
        let mut content = format!(
            r#"\begin{{table}}[H]
\centering
\caption{{Optimizer Family Comparison Matrix}}
\label{{tab:family_comparison_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{l{}}}
\toprule
\textbf{{QQN Family}} {}\\ 
\midrule
"#,
            "c".repeat(non_qqn_families.len()),
            non_qqn_families
                .iter()
                .map(|fam| format!("& \\textbf{{{}}}", Self::escape_latex(fam)))
                .collect::<Vec<_>>()
                .join(" ")
        );
        // Group results by problem and family for comparison
        let mut problem_family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> =
            HashMap::new();
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            for result in &results.results {
                let family = get_optimizer_family(&result.optimizer_name);
                problem_family_results
                    .entry(problem_name.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(family)
                    .or_insert_with(Vec::new)
                    .push(result);
            }
        }
        for qqn_fam in &qqn_families {
            content.push_str(&format!("\\textbf{{{}}} ", Self::escape_latex(qqn_fam)));
            for non_qqn_fam in &non_qqn_families {
                let mut wins = 0;
                let mut losses = 0;
                let mut ties = 0;
                for (_, families) in &problem_family_results {
                    if let (Some(qqn_results), Some(non_qqn_results)) =
                        (families.get(qqn_fam), families.get(non_qqn_fam))
                    {
                        if qqn_results.len() >= 2 && non_qqn_results.len() >= 2 {
                            let qqn_final_values: Vec<f64> = qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            let non_qqn_final_values: Vec<f64> = non_qqn_results
                                .iter()
                                .map(|r| r.final_value)
                                .filter(|&v| v.is_finite())
                                .collect();
                            if !qqn_final_values.is_empty() && !non_qqn_final_values.is_empty() {
                                let qqn_mean = qqn_final_values.iter().sum::<f64>()
                                    / qqn_final_values.len() as f64;
                                let non_qqn_mean = non_qqn_final_values.iter().sum::<f64>()
                                    / non_qqn_final_values.len() as f64;
                                if let Ok((_, p_value)) = slf
                                    .statistical_analysis
                                    .welch_t_test_public(&qqn_final_values, &non_qqn_final_values)
                                {
                                    if p_value < 0.05 {
                                        if qqn_mean < non_qqn_mean {
                                            wins += 1;
                                        } else {
                                            losses += 1;
                                        }
                                    } else {
                                        ties += 1;
                                    }
                                } else {
                                    ties += 1;
                                }
                            }
                        }
                    }
                }
                let cell_content = if wins > losses {
                    format!(
                        "\\textcolor{{green!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else if losses > wins {
                    format!(
                        "\\textcolor{{red!70!black}}{{{}W-{}L-{}T}}",
                        wins, losses, ties
                    )
                } else {
                    format!("{}W-{}L-{}T", wins, losses, ties)
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
\textbf{Legend:} W = Wins (statistically significant better performance), L = Losses (statistically significant worse performance), T = Ties (no significant difference). Green indicates QQN family dominance, red indicates non-QQN family dominance.
"#,
        );
        Ok(content)
    }
    /// Generate family vs family table content (without document wrapper)
    fn generate_family_vs_family_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        // Collect all optimizer families and problem families
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
\caption{{Optimizer Family vs Problem Family Performance Matrix}}
\label{{tab:family_vs_family_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{l{}}}
\toprule
\textbf{{Problem Family}} {}\\ 
\midrule
"#,
            "c".repeat(optimizer_families.len()),
            optimizer_families
                .iter()
                .map(|fam| format!(
                    "& \\rotatebox{{90}}{{\\textbf{{{}}}}}",
                    Self::escape_latex(fam)
                ))
                .collect::<Vec<_>>()
                .join(" ")
        );
        // For each problem family, calculate statistics
        for problem_family in &problem_families {
            content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex(problem_family)
            ));
            // Get all problems in this family
            let problems_in_family: Vec<_> = all_results
                .iter()
                .filter(|(problem, _)| get_family(&problem.get_name()) == *problem_family)
                .collect();
            for optimizer_family in &optimizer_families {
                let cell_data =
                    Self::calculate_family_performance_data(&problems_in_family, optimizer_family)?;
                let cell_content =                     format!(
                    "& \\begin{{tabular}}{{@{{}}c@{{}}}} {:.1} \\\\ {:.1} \\\\ \\tiny{{{}}} \\\\ \\tiny{{{}}} \\end{{tabular}}",
                    cell_data.average_ranking,
                    cell_data.best_rank_average,
                    Self::escape_latex(&cell_data.best_variant),
                    Self::escape_latex(&cell_data.worst_variant)
                )
                    ;
                content.push_str(&cell_content);
            }
            content.push_str(" \\\\\n");
        }
        content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Legend:} Each cell contains four values arranged vertically:
\begin{itemize}
\item \textbf{Top line:} Average Ranking across all problems in the family (lower is better)
\item \textbf{Second line:} Best Rank Average - average of best ranks achieved (lower is better)  
\item \textbf{Third line:} Best Variant - optimizer that achieved the best average rank
\item \textbf{Bottom line:} Worst Variant - optimizer that achieved the worst average rank
\end{itemize}
QQN family cells are highlighted in green for easy identification.
"#,
        );
        Ok(content)
    }
    /// Generate efficiency matrix LaTeX table
    fn generate_efficiency_matrix_latex_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
        // Collect all optimizer families and problem families
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
            return Ok(());
        }
        // Calculate column specification dynamically
        let col_spec = format!("l{}", "c".repeat(problem_families.len()));

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
\textbf{{Optimizer Family}} {}\\ 
\midrule
"#,
            problem_families
                .iter()
                .map(|fam| format!("& \\textbf{{{}}}", Self::escape_latex(fam)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        // Calculate efficiency data for each optimizer family across problem families
        for optimizer_family in &optimizer_families {
            latex_content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex(optimizer_family)
            ));
            for problem_family in &problem_families {
                // Get all successful runs for this optimizer family on this problem family
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
        let latex_path = latex_dir.join("efficiency_matrix.tex");
        fs::write(&latex_path, latex_content).with_context(|| {
            format!(
                "Failed to write efficiency matrix LaTeX table to: {}",
                latex_path.display()
            )
        })?;
        println!(
            "Generated efficiency matrix LaTeX table: {}",
            latex_path.display()
        );
        Ok(())
    }
    /// Generate success rate heatmap LaTeX table
    fn generate_success_rate_heatmap_latex_table(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
        // Collect all optimizers and problems
        let mut all_optimizers = std::collections::HashSet::new();
        let mut all_problems = Vec::new();
        for (problem, results) in all_results {
            all_problems.push(problem.get_name());
            for result in &results.results {
                all_optimizers.insert(result.optimizer_name.clone());
            }
        }
        let mut optimizers: Vec<_> = all_optimizers.into_iter().collect();
        optimizers.sort();
        if optimizers.is_empty() || all_problems.is_empty() {
            return Ok(());
        }
        // Calculate column specification dynamically
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
                .map(|opt| format!("& \\rotatebox{{90}}{{\\textbf{{{}}}}}", Self::escape_latex(opt)))
                .collect::<Vec<_>>()
                .join(" ")
        ));
        // Calculate success rates for each problem-optimizer combination
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            latex_content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex(&problem_name)
            ));
            for optimizer in &optimizers {
                let optimizer_results: Vec<_> = results
                    .results
                    .iter()
                    .filter(|r| r.optimizer_name == *optimizer)
                    .collect();
                let success_rate = if optimizer_results.is_empty() {
                    0.0
                } else {
                    let successful = optimizer_results
                        .iter()
                        .filter(|r| r.convergence_achieved)
                        .count();
                    successful as f64 / optimizer_results.len() as f64 * 100.0
                };
                let (color, text_color) = if success_rate >= 90.0 {
                    ("green!70", "black")
                } else if success_rate >= 50.0 {
                    ("yellow!70", "black")
                } else if success_rate >= 10.0 {
                    ("orange!70", "black")
                } else {
                    ("red!70", "white")
                };
                let cell_content = if optimizer_results.is_empty() {
                    format!("& \\cellcolor{{gray!30}}\\textcolor{{white}}{{N/A}}")
                } else {
                    format!(
                        "& \\cellcolor{{{}}}\\textcolor{{{}}}{{{:.0}\\%}}",
                        color, text_color, success_rate
                    )
                };
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
    /// Generate convergence speed analysis LaTeX table
    fn generate_convergence_speed_latex_table(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        latex_dir: &Path,
    ) -> anyhow::Result<()> {
        // Collect convergence speed data for all optimizers
        let mut optimizer_speed_data = std::collections::HashMap::new();
        for (_, results) in all_results {
            for result in &results.results {
                if result.convergence_achieved && !result.trace.iterations.is_empty() {
                    let speed_data = optimizer_speed_data
                        .entry(result.optimizer_name.clone())
                        .or_insert(Vec::new());
                    // Calculate convergence milestones
                    let initial_value = result
                        .trace
                        .iterations
                        .first()
                        .map(|iter| iter.function_value)
                        .unwrap_or(result.final_value);
                    let final_value = result.final_value;
                    let improvement_50 = initial_value - 0.5 * (initial_value - final_value);
                    let improvement_90 = initial_value - 0.9 * (initial_value - final_value);
                    let mut iter_to_50 = None;
                    let mut iter_to_90 = None;
                    for iter_data in &result.trace.iterations {
                        if iter_to_50.is_none() && iter_data.function_value <= improvement_50 {
                            iter_to_50 = Some(iter_data.iteration);
                        }
                        if iter_to_90.is_none() && iter_data.function_value <= improvement_90 {
                            iter_to_90 = Some(iter_data.iteration);
                        }
                    }
                    speed_data.push((
                        iter_to_50.unwrap_or(result.iterations),
                        iter_to_90.unwrap_or(result.iterations),
                        result.iterations,
                    ));
                }
            }
        }
        if optimizer_speed_data.is_empty() {
            return Ok(());
        }
        let mut latex_content = String::from(
            r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage[table]{xcolor}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage[margin=1in]{geometry}
\begin{document}
\begin{table}[htbp]
\centering
\caption{Convergence Speed Analysis: Mean Iterations to Reach Improvement Milestones}
\label{tab:convergence_speed}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{p{3cm}p{2cm}p{2cm}p{2cm}}
\toprule
\textbf{Optimizer} & \textbf{Mean Iterations} & \textbf{Mean Iterations} & \textbf{Final Convergence} \\
 & \textbf{to 50\% Improvement} & \textbf{to 90\% Improvement} & \textbf{Iteration} \\
\midrule
"#,
        );
        // Calculate averages and sort by overall convergence speed
        let mut optimizer_averages = Vec::new();
        for (optimizer, speed_data) in optimizer_speed_data {
            if !speed_data.is_empty() {
                let avg_50 = speed_data
                    .iter()
                    .map(|(iter_50, _, _)| *iter_50 as f64)
                    .sum::<f64>()
                    / speed_data.len() as f64;
                let avg_90 = speed_data
                    .iter()
                    .map(|(_, iter_90, _)| *iter_90 as f64)
                    .sum::<f64>()
                    / speed_data.len() as f64;
                let avg_final = speed_data
                    .iter()
                    .map(|(_, _, final_iter)| *final_iter as f64)
                    .sum::<f64>()
                    / speed_data.len() as f64;
                optimizer_averages.push((optimizer, avg_50, avg_90, avg_final));
            }
        }
        // Sort by fastest overall convergence (weighted average of milestones)
        optimizer_averages.sort_by(|a, b| {
            let score_a = 0.3 * a.1 + 0.4 * a.2 + 0.3 * a.3;
            let score_b = 0.3 * b.1 + 0.4 * b.2 + 0.3 * b.3;
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for (i, (optimizer, avg_50, avg_90, avg_final)) in optimizer_averages.iter().enumerate() {
            let optimizer_style = if i == 0 {
                format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
            } else if optimizer.contains("QQN") {
                format!(
                    "\\textcolor{{green!70!black}}{{{}}}",
                    Self::escape_latex(optimizer)
                )
            } else {
                Self::escape_latex(optimizer)
            };
            latex_content.push_str(&format!(
                "{} & {:.1} & {:.1} & {:.1} \\\\\n",
                optimizer_style, avg_50, avg_90, avg_final
            ));
        }
        latex_content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Compares convergence rates for different optimizers. Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.
\end{document}
"#,
        );
        let latex_path = latex_dir.join("convergence_speed_table.tex");
        fs::write(&latex_path, latex_content).with_context(|| {
            format!(
                "Failed to write convergence speed LaTeX table to: {}",
                latex_path.display()
            )
        })?;
        println!(
            "Generated convergence speed analysis LaTeX table: {}",
            latex_path.display()
        );
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
                .map(|fam| format!("& \\textbf{{{}}}", Self::escape_latex(fam)))
                .collect::<Vec<_>>()
                .join(" ")
        );
        // Same calculation logic as the standalone table
        for optimizer_family in &optimizer_families {
            content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex(optimizer_family)
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
    /// Generate success rate heatmap table content (without document wrapper)
    fn generate_success_rate_heatmap_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        // Similar logic as generate_success_rate_heatmap_latex_table but return just the table content
        let mut all_optimizers = std::collections::HashSet::new();
        let mut all_problems = Vec::new();
        for (problem, results) in all_results {
            all_problems.push(problem.get_name());
            for result in &results.results {
                all_optimizers.insert(result.optimizer_name.clone());
            }
        }
        let mut optimizers: Vec<_> = all_optimizers.into_iter().collect();
        optimizers.sort();
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
                    Self::escape_latex(opt)
                ))
                .collect::<Vec<_>>()
                .join(" ")
        );
        // Same calculation logic as the standalone table
        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            content.push_str(&format!(
                "\\textbf{{{}}} ",
                Self::escape_latex(&problem_name)
            ));
            for optimizer in &optimizers {
                let optimizer_results: Vec<_> = results
                    .results
                    .iter()
                    .filter(|r| r.optimizer_name == *optimizer)
                    .collect();
                let success_rate = if optimizer_results.is_empty() {
                    0.0
                } else {
                    let successful = optimizer_results
                        .iter()
                        .filter(|r| r.convergence_achieved)
                        .count();
                    successful as f64 / optimizer_results.len() as f64 * 100.0
                };
                let (color, text_color) = if success_rate >= 90.0 {
                    ("green!70", "black")
                } else if success_rate >= 50.0 {
                    ("yellow!70", "black")
                } else if success_rate >= 10.0 {
                    ("orange!70", "black")
                } else {
                    ("red!70", "white")
                };
                let cell_content = if optimizer_results.is_empty() {
                    format!("& \\cellcolor{{gray!30}}\\textcolor{{white}}{{N/A}}")
                } else {
                    format!(
                        "& \\cellcolor{{{}}}\\textcolor{{{}}}{{{:.0}\\%}}",
                        color, text_color, success_rate
                    )
                };
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
    /// Generate convergence speed table content (without document wrapper)
    fn generate_convergence_speed_table_content(
        all_results: &[(&ProblemSpec, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        // Similar logic as generate_convergence_speed_latex_table but return just the table content
        let mut optimizer_speed_data = std::collections::HashMap::new();
        for (_, results) in all_results {
            for result in &results.results {
                if result.convergence_achieved && !result.trace.iterations.is_empty() {
                    let speed_data = optimizer_speed_data
                        .entry(result.optimizer_name.clone())
                        .or_insert(Vec::new());
                    let initial_value = result
                        .trace
                        .iterations
                        .first()
                        .map(|iter| iter.function_value)
                        .unwrap_or(result.final_value);
                    let final_value = result.final_value;
                    let improvement_50 = initial_value - 0.5 * (initial_value - final_value);
                    let improvement_90 = initial_value - 0.9 * (initial_value - final_value);
                    let mut iter_to_50 = None;
                    let mut iter_to_90 = None;
                    for iter_data in &result.trace.iterations {
                        if iter_to_50.is_none() && iter_data.function_value <= improvement_50 {
                            iter_to_50 = Some(iter_data.iteration);
                        }
                        if iter_to_90.is_none() && iter_data.function_value <= improvement_90 {
                            iter_to_90 = Some(iter_data.iteration);
                        }
                    }
                    speed_data.push((
                        iter_to_50.unwrap_or(result.iterations),
                        iter_to_90.unwrap_or(result.iterations),
                        result.iterations,
                    ));
                }
            }
        }
        if optimizer_speed_data.is_empty() {
            return Ok(String::new());
        }
        let mut content = String::from(
            r#"\begin{table}[H]
\centering
\caption{Convergence Speed Analysis: Mean Iterations to Reach Improvement Milestones}
\label{tab:convergence_speed}
\adjustbox{width=\textwidth,center}{
\begin{tabular}{lrrr}
\toprule
\textbf{Optimizer} & \textbf{Mean Iterations} & \textbf{Mean Iterations} & \textbf{Final Convergence} \\
 & \textbf{to 50\% Improvement} & \textbf{to 90\% Improvement} & \textbf{Iteration} \\
\midrule
"#,
        );
        // Same calculation and sorting logic as the standalone table
        let mut optimizer_averages = Vec::new();
        for (optimizer, speed_data) in optimizer_speed_data {
            if !speed_data.is_empty() {
                let avg_50 = speed_data
                    .iter()
                    .map(|(iter_50, _, _)| *iter_50 as f64)
                    .sum::<f64>()
                    / speed_data.len() as f64;
                let avg_90 = speed_data
                    .iter()
                    .map(|(_, iter_90, _)| *iter_90 as f64)
                    .sum::<f64>()
                    / speed_data.len() as f64;
                let avg_final = speed_data
                    .iter()
                    .map(|(_, _, final_iter)| *final_iter as f64)
                    .sum::<f64>()
                    / speed_data.len() as f64;
                optimizer_averages.push((optimizer, avg_50, avg_90, avg_final));
            }
        }
        optimizer_averages.sort_by(|a, b| {
            let score_a = 0.3 * a.1 + 0.4 * a.2 + 0.3 * a.3;
            let score_b = 0.3 * b.1 + 0.4 * b.2 + 0.3 * b.3;
            score_a
                .partial_cmp(&score_b)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        for (i, (optimizer, avg_50, avg_90, avg_final)) in optimizer_averages.iter().enumerate() {
            let optimizer_style = if i == 0 {
                format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
            } else if optimizer.contains("QQN") {
                format!(
                    "\\textcolor{{green!70!black}}{{{}}}",
                    Self::escape_latex(optimizer)
                )
            } else {
                Self::escape_latex(optimizer)
            };
            content.push_str(&format!(
                "{} & {:.1} & {:.1} & {:.1} \\\\\n",
                optimizer_style, avg_50, avg_90, avg_final
            ));
        }
        content.push_str(
            r#"\bottomrule
\end{tabular}
}
\end{table}
\textbf{Purpose:} Compares convergence rates for different optimizers. Sorted by fastest overall convergence (weighted average). Best performer is highlighted in bold, QQN variants in green.
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
            Self::escape_latex(&problem_name),
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
                format!("\\textbf{{{}}}", Self::escape_latex(optimizer))
            } else {
                Self::escape_latex(optimizer)
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
    fn escape_latex_safe(text: &str) -> String {
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
    fn escape_latex(text: &str) -> String {
        Self::escape_latex_safe(text)
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
                Self::generate_optimizer_problem_report(
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
        let mut content = Self::generate_detailed_report_header(problem, optimizer_name, runs);
        content.push_str(&Self::generate_run_by_run_analysis(runs)?);
        content.push_str(&Self::generate_convergence_analysis(runs)?);
        content.push_str(&Self::generate_parameter_evolution_analysis(runs)?);
        content.push_str(&Self::generate_performance_analysis(runs)?);
        content.push_str(&Self::generate_failure_analysis(runs)?);
        content.push_str(&Self::generate_detailed_report_footer(
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
    fn generate_convergence_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
        let successful_runs: Vec<_> = runs.iter().filter(|r| r.convergence_achieved).collect();
        let failed_runs: Vec<_> = runs.iter().filter(|r| !r.convergence_achieved).collect();
        let mut content = String::from("## Convergence Analysis\n\n");
        if !successful_runs.is_empty() {
            let iterations: Vec<usize> = successful_runs.iter().map(|r| r.iterations).collect();
            let function_evals: Vec<usize> = successful_runs
                .iter()
                .map(|r| r.function_evaluations)
                .collect();
            let times: Vec<f64> = successful_runs
                .iter()
                .map(|r| r.execution_time.as_secs_f64())
                .collect();
            let mean_iterations = iterations.iter().sum::<usize>() as f64 / iterations.len() as f64;
            let mean_func_evals =
                function_evals.iter().sum::<usize>() as f64 / function_evals.len() as f64;
            let mean_time = times.iter().sum::<f64>() / times.len() as f64;
            // Find min and max times with their corresponding iterations
            let (min_iter, min_time) = successful_runs
                .iter()
                .map(|r| (r.iterations, r.execution_time.as_secs_f64()))
                .min_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, 0.0));
            let (max_iter, max_time) = successful_runs
                .iter()
                .map(|r| (r.iterations, r.execution_time.as_secs_f64()))
                .max_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, 0.0));

            content.push_str(&format!(
                r#"### Successful Runs ({} out of {})

* **Average Iterations to Convergence:** {:.1}
* **Average Function Evaluations:** {:.1}
* **Average Time to Convergence:** {:.3}s
* **Fastest Convergence:** {} iterations ({:.3}s)
* **Slowest Convergence:** {} iterations ({:.3}s)

"#,
                successful_runs.len(),
                runs.len(),
                mean_iterations,
                mean_func_evals,
                mean_time,
                min_iter,
                min_time,
                max_iter,
                max_time
            ));
        }
        if !failed_runs.is_empty() {
            let mut failure_reasons = std::collections::HashMap::new();
            for run in &failed_runs {
                *failure_reasons
                    .entry(format!("{:?}", run.convergence_reason))
                    .or_insert(0) += 1;
            }
            content.push_str(&format!(
                "### Failed Runs ({} out of {})\n\n**Failure Reasons:**\n",
                failed_runs.len(),
                runs.len()
            ));
            for (reason, count) in failure_reasons {
                content.push_str(&format!("- {}: {} runs\n", reason, count));
            }
            content.push_str("\n");
        }
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
    fn generate_performance_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
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
* [Raw CSV Data](problems/{}_results.csv)
* [Convergence Plot](convergence_{}.png)
* [Log Scale Convergence Plot](convergence_{}_log.png)


---
*Detailed report for {} on {}*
*Generated on: {}*
*[← Back to Main Report](benchmark_report.md)*
"#,
            problem_name.replace(" ", "_"),
            problem_name.replace(" ", "_"),
            problem_name.replace(" ", "_"),
            optimizer_name,
            problem_name,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }
}
