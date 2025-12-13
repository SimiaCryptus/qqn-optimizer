use crate::benchmarks::evaluation::{is_no_threshold_mode, BenchmarkResults, ProblemSpec};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::report_generator::FamilyPerformanceData;
use crate::experiment_runner::{report_generator, shorten_optimizer_name};
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
// Define consistent colors
const BEST_COLOR_LATEX: &str = "\\cellcolor{bestgreen!30}";
const WORST_COLOR_LATEX: &str = "\\cellcolor{worstred!20}";
const BEST_COLOR_LATEX_INLINE: &str = "\\cellcolor{green!20}";
const WORST_COLOR_LATEX_INLINE: &str = "\\cellcolor{red!15}";
const MAX_NAME_SIZE: usize = 14;

/// Generate family vs family comparison LaTeX table
pub async fn generate_family_vs_family_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
) -> anyhow::Result<()> {
    // Collect all optimizer families and problem families
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

    if optimizer_families.is_empty() || problem_families.is_empty() {
        return Ok(());
    }
    // Ensure parent directory exists
    if let Some(parent) = latex_dir.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }
    fs::create_dir_all(latex_dir)
        .with_context(|| format!("Failed to create latex directory: {}", latex_dir.display()))?;

    // Calculate column specification dynamically
    let col_spec = format!("l{}", "c".repeat(optimizer_families.len()));

    let mut latex_content = String::from(
        r#"\documentclass{article}
\usepackage{booktabs}
\usepackage{array}
\usepackage{multirow}
\usepackage{xcolor}
\usepackage{colortbl}
\usepackage{siunitx}
\usepackage{adjustbox}
\usepackage{rotating}
\usepackage{makecell}
\usepackage[margin=0.5in]{geometry}
\usepackage{longtable}
\definecolor{bestgreen}{RGB}{0,150,0}
\definecolor{worstred}{RGB}{200,0,0}
\begin{document}
"#,
    );
    // Find best and worst performers for coloring
    let mut family_scores: HashMap<String, Vec<f32>> = HashMap::new();
    for problem_family in &problem_families {
        let problems_in_family: Vec<_> = all_results
            .iter()
            .filter(|(problem, _)| {
                report_generator::get_family(&problem.get_name()) == *problem_family
            })
            .collect();
        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;
            let key = format!("{problem_family}_{optimizer_family}");
            family_scores.insert(
                key,
                vec![cell_data.average_ranking, cell_data.best_rank_average],
            );
        }
    }

    latex_content.push_str(&format!(
        r#"\begin{{longtable}}{{{col_spec}}}
\caption{{Optimizer Family vs Problem Family Performance Matrix}}
\label{{tab:family_vs_family_matrix}} \\
\toprule
\makecell{{Problem\\Family}} {} \\
\midrule
\endfirsthead
\multicolumn{{{}}}{{l}}{{\tablename\ \thetable\ -- continued from previous page}} \\
\toprule
\makecell{{Problem\\Family}} {} \\
\midrule
\endhead
\midrule
\multicolumn{{{}}}{{l}}{{Continued on next page}} \\
\endfoot
\bottomrule
\endlastfoot
"#,
        optimizer_families
            .iter()
            .map(|fam| format!(
                "& \\makecell{{\\rotatebox{{90}}{{\\textbf{{{}}}}}}}",
                report_generator::escape_latex(fam)
            ))
            .collect::<Vec<_>>()
            .join(" "),
        optimizer_families.len() + 1,
        optimizer_families
            .iter()
            .map(|fam| format!(
                "& \\makecell{{\\rotatebox{{90}}{{\\textbf{{{}}}}}}}",
                report_generator::escape_latex(fam)
            ))
            .collect::<Vec<_>>()
            .join(" "),
        optimizer_families.len() + 1
    ));

    // For each problem family, calculate statistics
    for problem_family in &problem_families {
        latex_content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(problem_family)
        ));

        // Get all problems in this family
        let problems_in_family: Vec<_> = all_results
            .iter()
            .filter(|(problem, _)| {
                report_generator::get_family(&problem.get_name()) == *problem_family
            })
            .collect();
        // Collect scores for this row to find best/worst
        let mut row_scores = Vec::new();
        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;
            row_scores.push((optimizer_family.clone(), cell_data.average_ranking));
        }

        // Find best and worst in this row
        let best_family = row_scores
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(f, _)| f.clone())
            .unwrap_or_default();
        let worst_family = row_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(f, _)| f.clone())
            .unwrap_or_default();

        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;

            // Determine cell color
            let color_cmd = if cell_data.average_ranking.is_finite() {
                if optimizer_family.to_string() == best_family {
                    BEST_COLOR_LATEX
                } else if optimizer_family.to_string() == worst_family {
                    WORST_COLOR_LATEX
                } else {
                    ""
                }
            } else {
                ""
            };

            let cell_content = format!(
                "& {} \\makecell{{{:.1} / {:.1} \\\\ \\scriptsize{{{}}} \\\\ \\scriptsize{{{}}}}}",
                color_cmd,
                cell_data.average_ranking,
                cell_data.best_rank_average,
                report_generator::escape_latex(&truncate_name(
                    &cell_data.best_variant,
                    MAX_NAME_SIZE
                )),
                report_generator::escape_latex(&truncate_name(
                    &cell_data.worst_variant,
                    MAX_NAME_SIZE
                ))
            );
            latex_content.push_str(&cell_content);
        }
        latex_content.push_str(" \\\\\n");
    }
    latex_content.push_str(
        r#"\end{longtable}
\vspace{0.5em}
\textbf{Legend:} Each cell contains:
\begin{itemize}
\item \textbf{Top line:} Average Ranking / Best Rank Average (lower is better)
\item \textbf{Middle line:} Best performing variant in this optimizer family
\item \textbf{Bottom line:} Worst performing variant in this optimizer family
\end{itemize}
\textcolor{bestgreen}{Green cells} indicate the best performing optimizer family for that problem family.
\textcolor{worstred}{Red cells} indicate the worst performing optimizer family.
\end{document}
"#,
    );
    let latex_path = latex_dir.join("family_vs_family_matrix.tex".to_string());
    fs::write(&latex_path, latex_content)
        .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
    println!(
        "Generated family vs family comparison LaTeX table: {}",
        latex_path.display()
    );
    Ok(())
}
/// Truncate a name to a maximum length
fn truncate_name(name: &str, max_len: usize) -> String {
    if max_len == 0 {
        return String::new();
    }
    let name = shorten_optimizer_name(name, max_len);
    if name.len() <= max_len {
        name.to_string()
    } else {
        let truncate_at = max_len.saturating_sub(3);
        if truncate_at == 0 {
            "...".to_string()
        } else {
            format!("{}...", &name[..truncate_at])
        }
    }
}

/// Generate family vs family table content (without document wrapper)
pub fn generate_family_vs_family_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<String> {
    // Collect all optimizer families and problem families
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
    if optimizer_families.is_empty() || problem_families.is_empty() {
        return Ok(String::new());
    }
    // Find best and worst performers for coloring
    let mut family_scores: HashMap<String, Vec<f32>> = HashMap::new();
    for problem_family in &problem_families {
        let problems_in_family: Vec<_> = all_results
            .iter()
            .filter(|(problem, _)| {
                report_generator::get_family(&problem.get_name()) == *problem_family
            })
            .collect();
        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;
            let key = format!("{problem_family}_{optimizer_family}");
            family_scores.insert(
                key,
                vec![cell_data.average_ranking, cell_data.best_rank_average],
            );
        }
    }

    let mut content = format!(
        r#"\begin{{longtable}}{{l{}}}
\caption{{Optimizer Family vs Problem Family Performance Matrix}}
\label{{tab:family_vs_family_matrix}} \\
\toprule
\makecell{{Problem\\Family}} {} \\
\midrule
\endfirsthead
\multicolumn{{{}}}{{l}}{{\tablename\ \thetable\ -- continued from previous page}} \\
\toprule
\makecell{{Problem\\Family}} {} \\
\midrule
\endhead
\midrule
\multicolumn{{{}}}{{l}}{{Continued on next page}} \\
\endfoot
\bottomrule
\endlastfoot
"#,
        "c".repeat(optimizer_families.len()),
        optimizer_families
            .iter()
            .map(|fam| format!(
                "& \\makecell{{\\rotatebox{{90}}{{\\textbf{{{}}}}}}}",
                report_generator::escape_latex(fam)
            ))
            .collect::<Vec<_>>()
            .join(" "),
        optimizer_families.len() + 1,
        optimizer_families
            .iter()
            .map(|fam| format!(
                "& \\makecell{{\\rotatebox{{90}}{{\\textbf{{{}}}}}}}",
                report_generator::escape_latex(fam)
            ))
            .collect::<Vec<_>>()
            .join(" "),
        optimizer_families.len() + 1
    );

    // For each problem family, calculate statistics
    for problem_family in &problem_families {
        content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(problem_family)
        ));

        // Get all problems in this family
        let problems_in_family: Vec<_> = all_results
            .iter()
            .filter(|(problem, _)| {
                report_generator::get_family(&problem.get_name()) == *problem_family
            })
            .collect();
        // Collect scores for this row to find best/worst
        let mut row_scores = Vec::new();
        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;
            row_scores.push((optimizer_family.clone(), cell_data.average_ranking));
        }

        // Find best and worst in this row
        let best_family = row_scores
            .iter()
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(f, _)| f.clone())
            .unwrap_or_default();
        let worst_family = row_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(f, _)| f.clone())
            .unwrap_or_default();

        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;

            // Determine cell color
            let color_cmd = if cell_data.average_ranking.is_finite() {
                if optimizer_family.to_string() == best_family.to_string() {
                    BEST_COLOR_LATEX_INLINE
                } else if optimizer_family.to_string() == worst_family.to_string() {
                    WORST_COLOR_LATEX_INLINE
                } else {
                    ""
                }
            } else {
                ""
            };

            let cell_content = format!(
                "& {} \\makecell{{{:.1} / {:.1} \\\\ \\scriptsize{{{}}} \\\\ \\scriptsize{{{}}}}}",
                color_cmd,
                cell_data.average_ranking,
                cell_data.best_rank_average,
                report_generator::escape_latex(&truncate_name(
                    &cell_data.best_variant,
                    MAX_NAME_SIZE
                )),
                report_generator::escape_latex(&truncate_name(
                    &cell_data.worst_variant,
                    MAX_NAME_SIZE
                ))
            );
            content.push_str(&cell_content);
        }
        content.push_str(" \\\\\n");
    }
    content.push_str(
        r#"\end{longtable}
\vspace{0.5em}
\textbf{Legend:} Each cell contains:
\begin{itemize}
\item \textbf{Top line:} Average Ranking / Best Rank Average (lower is better)
\item \textbf{Middle line:} Best performing variant in this optimizer family
\item \textbf{Bottom line:} Worst performing variant in this optimizer family
\end{itemize}
Green cells indicate the best performing optimizer family for that problem family.
Red cells indicate the worst performing optimizer family.
"#,
    );
    Ok(content)
}

pub fn generate_family_vs_family_comparison_table(
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
        return Ok("*No data available for family comparison.*\n\n".to_string());
    }
    // Create the table header
    content.push_str(r#"<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px; text-align: center; font-weight: bold;">Problem Family</th>
"#);
    for optimizer_family in &optimizer_families {
        content.push_str(&format!(
            r#"<th style="border: 1px solid #ddd; padding: 8px; text-align: center; font-weight: bold; writing-mode: vertical-lr; text-orientation: mixed;">{optimizer_family}</th>
"#
        ));
    }
    content.push_str("</tr>\n");
    // For each problem family, calculate statistics
    for problem_family in &problem_families {
        content.push_str(&format!(
            r#"<tr>
<td style="border: 1px solid #ddd; padding: 8px; font-weight: bold; background-color: #f8f9fa;">{problem_family}</td>
"#
        ));
        // Get all problems in this family
        let problems_in_family: Vec<_> = all_results
            .iter()
            .filter(|(problem, _)| {
                report_generator::get_family(&problem.get_name()) == *problem_family
            })
            .collect();
        if problems_in_family.is_empty() {
            continue;
        }

        for optimizer_family in &optimizer_families {
            let cell_data =
                calculate_family_performance_data(&problems_in_family, optimizer_family)?;

            // Collect scores for this row to find best/worst
            let mut row_scores = Vec::new();
            for opt_fam in &optimizer_families {
                let data = calculate_family_performance_data(&problems_in_family, opt_fam)?;
                if data.average_ranking.is_finite() {
                    row_scores.push((opt_fam.clone(), data.average_ranking));
                }
            }

            // Find best and worst in this row
            let best_family = row_scores
                .iter()
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(f, _)| f.to_string());
            let worst_family = row_scores
                .iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(f, _)| f.to_string());

            let cell_style = if cell_data.average_ranking.is_finite() {
                if Some(optimizer_family.to_string()) == best_family {
                    "border: 1px solid #ddd; padding: 6px; text-align: center; background-color: #90EE90; font-size: 10px;"
                } else if Some(optimizer_family.to_string()) == worst_family {
                    "border: 1px solid #ddd; padding: 6px; text-align: center; background-color: #FFB6C1; font-size: 10px;"
                } else {
                    "border: 1px solid #ddd; padding: 6px; text-align: center; font-size: 10px;"
                }
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
- Green cells indicate the best performing optimizer family for that problem family
- Red cells indicate the worst performing optimizer family for that problem family
"#,
    );
    Ok(content)
}

pub(crate) fn calculate_family_performance_data(
    problems_in_family: &[&(&ProblemSpec, BenchmarkResults)],
    optimizer_family: &str,
) -> anyhow::Result<FamilyPerformanceData> {
    if problems_in_family.is_empty() {
        return Ok(FamilyPerformanceData {
            average_ranking: f32::INFINITY,
            best_rank_average: f32::INFINITY,
            best_variant: "N/A".to_string(),
            worst_variant: "N/A".to_string(),
        });
    }

    let mut all_rankings = Vec::new();
    let mut best_ranks_per_problem = Vec::new();
    let mut variant_performance = HashMap::new();
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
            let final_values: Vec<f32> = runs
                .iter()
                .map(|r| r.best_value)
                .filter(|&v| v.is_finite())
                .collect();
            if final_values.is_empty() {
                continue;
            }
            let success_count = if is_no_threshold_mode() {
                0 // In no-threshold mode, we don't use convergence_achieved
            } else {
                runs.iter().filter(|r| r.convergence_achieved).count()
            };
            let success_rate = success_count as f32 / runs.len() as f32;
            let mean_final = final_values.iter().sum::<f32>() / final_values.len() as f32;
            let mean_func_evals = runs
                .iter()
                .map(|r| r.function_evaluations as f32)
                .sum::<f32>()
                / runs.len() as f32;
            let mean_grad_evals = runs
                .iter()
                .map(|r| r.gradient_evaluations as f32)
                .sum::<f32>()
                / runs.len() as f32;
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
                // In no-threshold mode, sort by best value (mean_final)
                a.2.total_cmp(&b.2)
            } else {
                // Sort by success rate first, then by total evaluations
                use std::cmp::Ordering;
                match b.1.total_cmp(&a.1) {
                    Ordering::Equal => {
                        if b.1 > 0.0 {
                            a.3.total_cmp(&b.3)
                        } else {
                            a.2.total_cmp(&b.2)
                        }
                    }
                    ord => ord,
                }
            }
        });
        // Assign rankings and collect data for this optimizer family
        let mut family_ranks_this_problem = Vec::new();
        let mut best_rank_this_problem = f32::INFINITY;
        for (rank, (optimizer, _, mean_final, _)) in perf_data.iter().enumerate() {
            let current_family = get_optimizer_family(optimizer);
            let rank_value = (rank + 1) as f32;
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
        all_rankings.iter().sum::<f32>() / all_rankings.len() as f32
    } else {
        f32::INFINITY
    };
    let best_rank_average = if !best_ranks_per_problem.is_empty() {
        best_ranks_per_problem.iter().sum::<f32>() / best_ranks_per_problem.len() as f32
    } else {
        f32::INFINITY
    };
    // Find best and worst variants
    let mut variant_averages = Vec::new();
    for (variant, ranks_and_values) in variant_performance {
        if !ranks_and_values.is_empty() {
            let avg_rank = ranks_and_values.iter().map(|(rank, _)| *rank).sum::<f32>()
                / ranks_and_values.len() as f32;
            variant_averages.push((variant, avg_rank));
        }
    }
    variant_averages.sort_by(|a, b| a.1.total_cmp(&b.1));
    let best_variant = variant_averages
        .first()
        .map(|(name, _)| shorten_optimizer_name(name, MAX_NAME_SIZE))
        .unwrap_or_else(|| "N/A".to_string());
    let worst_variant = variant_averages
        .last()
        .map(|(name, _)| shorten_optimizer_name(name, MAX_NAME_SIZE))
        .unwrap_or_else(|| "N/A".to_string());
    Ok(FamilyPerformanceData {
        average_ranking,
        best_rank_average,
        best_variant,
        worst_variant,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
    use crate::experiment_runner::test_data;
    use std::fs;

    #[tokio::test]
    async fn test_render_family_vs_family_examples() -> anyhow::Result<()> {
        // Create a target directory for manual checking
        let target_dir = Path::new("target/test_output/family_vs_family_examples");
        fs::create_dir_all(target_dir)?;
        // Create test data
        let test_data = test_data::create_test_data();
        let test_data_refs: Vec<(&ProblemSpec, BenchmarkResults)> = test_data
            .iter()
            .map(|(spec, results)| (spec, results.clone()))
            .collect();
        // Generate LaTeX table
        generate_family_vs_family_latex_table(&test_data_refs, target_dir).await?;
        // Generate HTML table content
        let html_content = generate_family_vs_family_comparison_table(&test_data_refs)?;
        let html_file_path = target_dir.join("family_vs_family_comparison.html".to_string());
        // Ensure parent directory exists before writing HTML file
        if let Some(parent) = html_file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        // Wrap the content in a complete HTML document for better viewing
        let full_html = format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <title>Family vs Family Comparison Test</title>
    <meta charset="UTF-8">
    <style>
        body {{ font-family: Arial, sans-serif; margin: 20px; }}
        h1, h2 {{ color: #333; }}
        table {{ border-collapse: collapse; margin: 20px 0; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: center; }}
        th {{ background-color: #f2f2f2; font-weight: bold; }}
        .best {{ background-color: #90EE90; }}
        .worst {{ background-color: #FFB6C1; }}
    </style>
</head>
<body>
    <h1>Family vs Family Comparison Test Output</h1>
    <p>This is a test rendering of the family vs family comparison table with mock data.</p>
    {}
</body>
</html>"#,
            html_content
        );
        fs::write(&html_file_path, full_html)?;
        // Generate table content only (for inclusion in larger documents)
        let table_content = generate_family_vs_family_table_content(&test_data_refs)?;
        let latex_content_path = target_dir.join("family_vs_family_table_content.tex");
        // Ensure parent directory exists before writing LaTeX content file
        if let Some(parent) = latex_content_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&latex_content_path, table_content)?;
        // Create a README file explaining the test outputs
        let readme_content = format!(
            r#"# Family vs Family Comparison Test Output
This directory contains test renderings of the family vs family comparison tables.
Generated on: {}
## Files:
1. **family_vs_family_matrix.tex** - Complete LaTeX document with the comparison table
2. **family_vs_family_comparison.html** - HTML version for web viewing
3. **family_vs_family_table_content.tex** - LaTeX table content only (for inclusion)
4. **README.md** - This file
## Test Data:
The test uses mock benchmark results for the following problem families:
- **rosenbrock**: rosenbrock_2d, rosenbrock_10d
- **sphere**: sphere_2d, sphere_10d
- **rastrigin**: rastrigin_2d
And the following optimizer families:
- **lbfgs**: lbfgs_default, lbfgs_aggressive
- **adam**: adam_default, adam_adaptive
- **sgd**: sgd_momentum
- **nelder_mead**: nelder_mead_standard
## Manual Verification:
1. Open the HTML file in a web browser to check the visual formatting
2. Compile the LaTeX file to verify the table renders correctly
3. Check that best/worst performers are highlighted appropriately
4. Verify that the table content can be included in other documents
## Expected Behavior:
- Green cells should highlight the best performing optimizer family for each problem family
- Red cells should highlight the worst performing optimizer family
- Each cell should contain average ranking, best rank average, and best/worst variants
- The table should be properly formatted and readable
"#,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        );
        let readme_path = target_dir.join("README.md");
        // Ensure parent directory exists before writing README file
        if let Some(parent) = readme_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&readme_path, readme_content)?;
        println!(
            "✅ Family vs family comparison examples rendered to: {}",
            target_dir.display()
        );
        println!("📁 Files generated:");
        println!("   - family_vs_family_matrix.tex (complete LaTeX document)");
        println!("   - family_vs_family_comparison.html (HTML version)");
        println!("   - family_vs_family_table_content.tex (LaTeX content only)");
        println!("   - README.md (documentation)");
        println!("🔍 Open the HTML file in a browser for manual verification");
        Ok(())
    }
    #[test]
    fn test_truncate_name() {
        assert_eq!(truncate_name("short", 10), "short".to_string());
        assert_eq!(truncate_name("exactlyten", 10), "exactlyten".to_string());
        assert_eq!(
            truncate_name("this_is_longer_than_ten", 10),
            "this_is...".to_string()
        );
        assert_eq!(truncate_name("test", 4), "test".to_string());
        assert_eq!(truncate_name("test", 3), "...".to_string());
        assert_eq!(truncate_name("test", 2), "...".to_string());
        assert_eq!(truncate_name("test", 1), "...".to_string());
        assert_eq!(truncate_name("test", 0), "".to_string());
        assert_eq!(truncate_name("very_long_name", 5), "ve...".to_string());
    }
    #[test]
    fn test_calculate_family_performance_data_empty() {
        let empty_problems = vec![];
        let result = calculate_family_performance_data(&empty_problems, "test_family").unwrap();
        assert!(result.average_ranking.is_infinite());
        assert!(result.best_rank_average.is_infinite());
        assert_eq!(result.best_variant, "N/A".to_string());
        assert_eq!(result.worst_variant, "N/A".to_string());
    }
}
