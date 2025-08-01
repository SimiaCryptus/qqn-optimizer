use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::{report_generator, ReportGenerator};
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
/// Result of a comparison between two optimizers
#[derive(Debug, Default)]
struct ComparisonResult {
    wins: usize,
    losses: usize,
    ties: usize,
}
impl ComparisonResult {
    /// Format the comparison result as a LaTeX cell
    fn to_latex(&self) -> String {
        match (self.wins > self.losses, self.losses > self.wins) {
            (true, false) => format!("\\textcolor{{green!70!black}}{{{wins}W-{losses}L-{ties}T}}", wins=self.wins, losses=self.losses, ties=self.ties),
            (false, true) => format!("\\textcolor{{red!70!black}}{{{wins}W-{losses}L-{ties}T}}", wins=self.wins, losses=self.losses, ties=self.ties),
            _ => format!("{wins}W-{losses}L-{ties}T", wins=self.wins, losses=self.losses, ties=self.ties),
        }
    }
}


/// Generate comparison matrix LaTeX table
pub fn generate_comparison_matrix_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
    slf: &ReportGenerator,
) -> anyhow::Result<()> {
    let (qqn_optimizers, non_qqn_optimizers) = collect_and_sort_optimizers(all_results)?;
    
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
    let col_spec = format!("l{}", "c".repeat(qqn_optimizers.len()));

    latex_content.push_str(&format!(
        r#"\begin{{table}}[htbp]
\centering
\caption{{QQN vs Non-QQN Optimizer Comparison Matrix}}
\label{{tab:comparison_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{{col_spec}}}
\toprule
\textbf{{Non-QQN Optimizer}} {}\\
\midrule
"#,
        qqn_optimizers
            .iter()
            .map(|opt| format!("& \\textbf{{{}}}", report_generator::escape_latex(opt)))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    let comparison_rows = generate_optimizer_comparison_rows(all_results, &qqn_optimizers, &non_qqn_optimizers, slf)?;
    latex_content.push_str(&comparison_rows);
    
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

/// Generate comparison matrix table content (without document wrapper)
pub fn generate_comparison_matrix_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    slf: &ReportGenerator,
) -> anyhow::Result<String> {
    let (qqn_optimizers, non_qqn_optimizers) = collect_and_sort_optimizers(all_results)?;

    let mut content = format!(
        r#"\begin{{table}}[H]
\centering
\caption{{QQN vs Non-QQN Optimizer Comparison Matrix}}
\label{{tab:comparison_matrix}}
\adjustbox{{width=\textwidth,center}}{{
\begin{{tabular}}{{l{}}}
\toprule
\textbf{{Non-QQN Optimizer}} {}\\
\midrule
"#,
        "c".repeat(qqn_optimizers.len()),
        qqn_optimizers
            .iter()
            .map(|opt| format!("& \\textbf{{{}}}", report_generator::escape_latex(opt)))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let comparison_rows = generate_optimizer_comparison_rows(all_results, &qqn_optimizers, &non_qqn_optimizers, slf)?;
    content.push_str(&comparison_rows);
    
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
pub fn generate_family_comparison_matrix_table_content(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    slf: &ReportGenerator,
) -> anyhow::Result<String> {
    let (qqn_families, non_qqn_families) = collect_and_sort_families(all_results)?;
    
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
            .map(|fam| format!("& \\textbf{{{}}}", report_generator::escape_latex(fam)))
            .collect::<Vec<_>>()
            .join(" ")
    );
    let comparison_rows = generate_family_comparison_rows(all_results, &qqn_families, &non_qqn_families, slf)?;
    content.push_str(&comparison_rows);
    
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

/// Generate family comparison matrix LaTeX table
pub fn generate_family_comparison_matrix_latex_table(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    latex_dir: &Path,
    slf: &ReportGenerator,
) -> anyhow::Result<()> {
    let (qqn_families, non_qqn_families) = collect_and_sort_families(all_results)?;
    
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
            .map(|fam| format!("& \\textbf{{{}}}", report_generator::escape_latex(fam)))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    let comparison_rows = generate_family_comparison_rows(all_results, &qqn_families, &non_qqn_families, slf)?;
    latex_content.push_str(&comparison_rows);
    
    latex_content.push_str(
        r#"\bottomrule
/// Helper function to collect and sort optimizers into QQN and non-QQN groups
fn collect_and_sort_optimizers(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<(Vec<String>, Vec<String>)> {
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
        anyhow::bail!("Cannot generate comparison matrix: need both QQN and non-QQN optimizers");
    }
    qqn_optimizers.sort();
    non_qqn_optimizers.sort();
    Ok((qqn_optimizers, non_qqn_optimizers))
}
/// Helper function to collect and sort optimizer families
fn collect_and_sort_families(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
) -> anyhow::Result<(Vec<String>, Vec<String>)> {
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
        anyhow::bail!("Cannot generate family comparison matrix: need both QQN and non-QQN families");
    }
    qqn_families.sort();
    non_qqn_families.sort();
    Ok((qqn_families, non_qqn_families))
}
/// Helper function to perform statistical comparison between two sets of results
fn compare_results(
    results1: &[&SingleResult],
    results2: &[&SingleResult],
    slf: &ReportGenerator,
) -> ComparisonResult {
    let mut comparison = ComparisonResult::default();
    if results1.len() < 2 || results2.len() < 2 {
        return comparison;
    }
    let values1: Vec<f64> = results1
        .iter()
        .map(|r| r.final_value)
        .filter(|&v| v.is_finite())
        .collect();
    let values2: Vec<f64> = results2
        .iter()
        .map(|r| r.final_value)
        .filter(|&v| v.is_finite())
        .collect();
    if values1.is_empty() || values2.is_empty() {
        return comparison;
    }
    let mean1 = values1.iter().sum::<f64>() / values1.len() as f64;
    let mean2 = values2.iter().sum::<f64>() / values2.len() as f64;
    match slf.statistical_analysis.welch_t_test_public(&values1, &values2) {
        Ok((_, p_value)) => {
            if p_value < 0.05 {
                if mean1 < mean2 {
                    comparison.wins += 1;
                } else {
                    comparison.losses += 1;
                }
            } else {
                comparison.ties += 1;
            }
        }
        Err(_) => {
            comparison.ties += 1;
        }
    }
    comparison
}
/// Generate comparison rows for optimizer comparison matrix
fn generate_optimizer_comparison_rows(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    qqn_optimizers: &[String],
    non_qqn_optimizers: &[String],
    slf: &ReportGenerator,
) -> anyhow::Result<String> {
    let mut problem_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> = HashMap::new();
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        for result in &results.results {
            problem_results
                .entry(problem_name.to_string())
                .or_default()
                .entry(result.optimizer_name.clone())
                .or_default()
                .push(result);
        }
    }
    let mut rows = String::new();
    for non_qqn_opt in non_qqn_optimizers {
        rows.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(non_qqn_opt)
        ));
        for qqn_opt in qqn_optimizers {
            let mut total_comparison = ComparisonResult::default();
            for optimizers in problem_results.values() {
                if let (Some(qqn_results), Some(non_qqn_results)) =
                    (optimizers.get(qqn_opt), optimizers.get(non_qqn_opt))
                {
                    let comparison = compare_results(qqn_results, non_qqn_results, slf);
                    total_comparison.wins += comparison.wins;
                    total_comparison.losses += comparison.losses;
                    total_comparison.ties += comparison.ties;
                }
            }
            rows.push_str(&format!("& {} ", total_comparison.to_latex()));
        }
        rows.push_str("\\\\\n");
    }
    Ok(rows)
}
/// Generate comparison rows for family comparison matrix
fn generate_family_comparison_rows(
    all_results: &[(&ProblemSpec, BenchmarkResults)],
    qqn_families: &[String],
    non_qqn_families: &[String],
    slf: &ReportGenerator,
) -> anyhow::Result<String> {
    let mut problem_family_results: HashMap<String, HashMap<String, Vec<&SingleResult>>> = HashMap::new();
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        for result in &results.results {
            let family = get_optimizer_family(&result.optimizer_name);
            problem_family_results
                .entry(problem_name.to_string())
                .or_default()
                .entry(family)
                .or_default()
                .push(result);
        }
    }
    let mut rows = String::new();
    for qqn_fam in qqn_families {
        rows.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(qqn_fam)
        ));
        for non_qqn_fam in non_qqn_families {
            let mut total_comparison = ComparisonResult::default();
            for families in problem_family_results.values() {
                if let (Some(qqn_results), Some(non_qqn_results)) =
                    (families.get(qqn_fam), families.get(non_qqn_fam))
                {
                    let comparison = compare_results(qqn_results, non_qqn_results, slf);
                    total_comparison.wins += comparison.wins;
                    total_comparison.losses += comparison.losses;
                    total_comparison.ties += comparison.ties;
                }
            }
            rows.push_str(&format!("& {} ", total_comparison.to_latex()));
        }
        rows.push_str("\\\\\n");
    }
    Ok(rows)
}
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