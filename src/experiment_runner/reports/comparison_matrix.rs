use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec, SingleResult};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::{report_generator, ReportGenerator};
use anyhow::Context;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const P_THRESHOLD: f64 = 0.05;

/// Generate comparison matrix LaTeX table
pub fn generate_comparison_matrix_latex_table(
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
    // Group results by problem for comparison
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
    for non_qqn_opt in &non_qqn_optimizers {
        latex_content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(non_qqn_opt)
        ));
        for qqn_opt in &qqn_optimizers {
            let mut wins = 0;
            let mut losses = 0;
            let mut ties = 0;
            for optimizers in problem_results.values() {
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
                                if p_value < P_THRESHOLD {
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
                format!("\\textcolor{{green!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else if losses > wins {
                format!("\\textcolor{{red!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else {
                format!("{wins}W-{losses}L-{ties}T")
            };
            latex_content.push_str(&format!("& {cell_content} "));
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
    // Ensure parent directory exists
    if let Some(parent) = latex_dir.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    // Ensure directory exists
    fs::create_dir_all(latex_dir)
        .with_context(|| format!("Failed to create directory: {}", latex_dir.display()))?;
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
    // Same comparison logic as before...
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
    for non_qqn_opt in &non_qqn_optimizers {
        content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(non_qqn_opt)
        ));
        for qqn_opt in &qqn_optimizers {
            let mut wins = 0;
            let mut losses = 0;
            let mut ties = 0;
            for optimizers in problem_results.values() {
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
                                if p_value < P_THRESHOLD {
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
                format!("\\textcolor{{green!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else if losses > wins {
                format!("\\textcolor{{red!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else {
                format!("{wins}W-{losses}L-{ties}T")
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
            .map(|fam| format!("& \\textbf{{{}}}", report_generator::escape_latex(fam)))
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
                .or_default()
                .entry(family)
                .or_default()
                .push(result);
        }
    }
    for qqn_fam in &qqn_families {
        content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(qqn_fam)
        ));
        for non_qqn_fam in &non_qqn_families {
            let mut wins = 0;
            let mut losses = 0;
            let mut ties = 0;
            for families in problem_family_results.values() {
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
                                if p_value < P_THRESHOLD {
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
                format!("\\textcolor{{green!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else if losses > wins {
                format!("\\textcolor{{red!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else {
                format!("{wins}W-{losses}L-{ties}T")
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
    // Collect all optimizer families
    let mut all_families = std::collections::HashSet::new();
    for (_, results) in all_results {
        for result in &results.results {
            let family = get_optimizer_family(&result.optimizer_name);
            all_families.insert(family);
        }
    }
    let mut qqn_families: Vec<String> = Vec::new();
    let mut non_qqn_families: Vec<String> = Vec::new();
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
            .to_vec()
            .iter()
            .map(|fam| format!("& \\textbf{{{}}}", report_generator::escape_latex(fam)).to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .to_string()
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
                .or_default()
                .entry(family)
                .or_default()
                .push(result);
        }
    }
    for qqn_fam in &qqn_families {
        latex_content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(qqn_fam)
        ));
        for non_qqn_fam in &non_qqn_families {
            let mut wins = 0;
            let mut losses = 0;
            let mut ties = 0;
            for families in problem_family_results.values() {
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
                                if p_value < P_THRESHOLD {
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
                format!("\\textcolor{{green!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else if losses > wins {
                format!("\\textcolor{{red!70!black}}{{{wins}W-{losses}L-{ties}T}}")
            } else {
                format!("{wins}W-{losses}L-{ties}T")
            };
            latex_content.push_str(&format!("& {cell_content} "));
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
    // Ensure parent directory exists
    if let Some(parent) = latex_dir.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
    }

    // Ensure directory exists
    fs::create_dir_all(latex_dir)
        .with_context(|| format!("Failed to create directory: {}", latex_dir.display()))?;
    let latex_path = latex_dir.join("family_comparison_matrix.tex");
    fs::write(&latex_path, latex_content)
        .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
    println!(
        "Generated family comparison matrix LaTeX table: {}",
        latex_path.display()
    );
    Ok(())
}
