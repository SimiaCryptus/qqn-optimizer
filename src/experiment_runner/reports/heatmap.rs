use crate::benchmarks::evaluation::{BenchmarkResults, ProblemSpec};
use crate::experiment_runner::report_generator;
use anyhow::Context;
use std::fs;
use std::path::Path;

/// Generate success rate heatmap table content (without document wrapper)
pub fn generate_success_rate_heatmap_table_content(
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
                report_generator::escape_latex(opt)
            ))
            .collect::<Vec<_>>()
            .join(" ")
    );
    // Same calculation logic as the standalone table
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(&problem_name)
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

/// Generate success rate heatmap LaTeX table
pub fn generate_success_rate_heatmap_latex_table(
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
            .map(|opt| format!("& \\rotatebox{{90}}{{\\textbf{{{}}}}}", report_generator::escape_latex(opt)))
            .collect::<Vec<_>>()
            .join(" ")
    ));
    // Calculate success rates for each problem-optimizer combination
    for (problem, results) in all_results {
        let problem_name = problem.get_name();
        latex_content.push_str(&format!(
            "\\textbf{{{}}} ",
            report_generator::escape_latex(&problem_name)
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
