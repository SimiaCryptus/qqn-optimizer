use std::path::Path;
use std::fs;
use anyhow::Context;
use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::experiment_runner::get_optimizer_family;
use crate::experiment_runner::report_generator;

/// Generate efficiency matrix LaTeX table
pub fn generate_efficiency_matrix_latex_table(
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
            .map(|fam| format!("& \\textbf{{{}}}", report_generator::escape_latex(fam)))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    // Calculate efficiency data for each optimizer family across problem families
    for optimizer_family in &optimizer_families {
        latex_content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(optimizer_family)
        ));
        for problem_family in &problem_families {
            // Get all successful runs for this optimizer family on this problem family
            let mut successful_evaluations = Vec::new();
            for (problem, results) in all_results {
                if report_generator::get_family(&problem.get_name()) == *problem_family {
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