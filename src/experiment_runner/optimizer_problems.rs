use std::cmp::Ordering;
use crate::benchmarks::evaluation::{
    is_no_threshold_mode, BenchmarkResults, ProblemSpec, SingleResult,
};
use crate::experiment_runner::report_generator;
use crate::experiment_runner::reports::convergence_analysis::generate_convergence_analysis;
use crate::experiment_runner::reports::performance_analysis::generate_performance_analysis;
use crate::experiment_runner::reports::{failure_analysis, parameter_evolution, run_by_run};
use crate::{experiment_runner, OptimizationProblem};
use anyhow::Context;
use log::warn;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate problem table content (without document wrapper)
pub fn generate_problem_table_content(
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
        experiment_runner::escape_latex(&problem_name),
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
        let function_evals: Vec<f64> = runs.iter().map(|r| r.function_evaluations as f64).collect();
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
        let mean_function_evals = function_evals.iter().sum::<f64>() / function_evals.len() as f64;
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

    if is_no_threshold_mode() {
        // In no-threshold mode, sort by best value
        perf_data.sort_by(|a, b| {
            a.3.total_cmp(&b.3)
        });
    } else {
        // Sort by success rate first, then by total evaluations
        perf_data.sort_by(|a, b| {
            use std::cmp::Ordering;
            match b.7.total_cmp(&a.7) {
                Ordering::Equal => if b.7 > 0.0 { (a.5 + a.6).total_cmp(&(b.5 + b.6)) } else {
                    a.3.total_cmp(&b.3)
                },
                other => other,
            }
        });
    }

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
            format!("\\textbf{{{}}}", experiment_runner::escape_latex(optimizer))
        } else {
            experiment_runner::escape_latex(optimizer)
        };
        content.push_str(&format!(
            "{optimizer_style} & {mean_final:.2e} & {std_final:.2e} & {best_final:.2e} & {worst_final:.2e} & {mean_func_evals:.1} & {success_rate:.1} & {mean_time:.3} \\\\\n"
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

/// Generate a detailed report for a specific optimizer on a specific problem
pub async fn generate_optimizer_problem_report(
    output_dir: &str,
    problem: &dyn OptimizationProblem,
    optimizer_name: &str,
    runs: &[&SingleResult],
) -> anyhow::Result<()> {
    let problem_name = problem.name();
    let problem_filename = problem_name.replace(" ", "_");
    let optimizer_filename = optimizer_name.replace(" ", "_");
    let filename = format!("detailed_{problem_filename}_{optimizer_filename}.md");
    let filepath = Path::new(output_dir).join(&filename);
    let mut content =
        report_generator::generate_detailed_report_header(problem, optimizer_name, runs);
    content.push_str(&run_by_run::generate_run_by_run_analysis(runs)?);
    content.push_str(&generate_convergence_analysis(runs)?);
    content.push_str(&parameter_evolution::generate_parameter_evolution_analysis(
        runs,
    )?);
    content.push_str(&generate_performance_analysis(runs)?);
    content.push_str(&failure_analysis::generate_failure_analysis(runs)?);
    content.push_str(&report_generator::generate_detailed_report_footer(
        problem_name,
        optimizer_name,
    ));
    fs::write(&filepath, content)
        .with_context(|| format!("Failed to write detailed report to: {}", filepath.display()))?;
    Ok(())
}

pub fn generate_problem_section(
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
        dimension.unwrap_or(0),
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
                result.optimizer_name, problem_name, result.final_value, result.final_gradient_norm
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
                "> {optimizer} claimed convergence with {evaluations} function evaluations (final_value: {final_value:.2e})  \n"
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

        let function_evals: Vec<f64> = runs.iter().map(|r| r.function_evaluations as f64).collect();
        let gradient_evals: Vec<f64> = runs.iter().map(|r| r.gradient_evaluations as f64).collect();
        let success_count = if is_no_threshold_mode() {
            0
        } else {
            runs.iter().filter(|r| r.convergence_achieved).count()
        };
        let execution_times: Vec<f64> = runs
            .iter()
            .map(|r| r.execution_time.as_secs_f64())
            .collect();

        let mean_final = final_values.iter().sum::<f64>() / final_values.len() as f64;
        if !mean_final.is_finite() {
            warn!(
                "Mean final value for optimizer '{optimizer}' is not finite (mean: {mean_final})"
            );
            continue;
        }
        // Separate statistics for successful and unsuccessful runs
        let successful_runs: Vec<_> = runs.iter().filter(|r| r.convergence_achieved).collect();
        let unsuccessful_runs: Vec<_> = runs.iter().filter(|r| !r.convergence_achieved).collect();
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
        let mean_function_evals = function_evals.iter().sum::<f64>() / function_evals.len() as f64;
        let mean_gradient_evals = gradient_evals.iter().sum::<f64>() / gradient_evals.len() as f64;
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

    if is_no_threshold_mode() {
        // In no-threshold mode, sort by best value
        perf_data.sort_by(|a, b| {
            a.3.total_cmp(&b.3)
        });
    } else {
        // Sort by success rate first, then by total evaluations
        perf_data.sort_by(|a, b| {
            use std::cmp::Ordering;
            match b.7.total_cmp(&a.7) {
                Ordering::Equal => if b.7 > 0.0 { (a.5 + a.6).total_cmp(&(b.5 + b.6)) } else {
                    a.3.total_cmp(&b.3)
                },
                other => other,
            }
        });
    }

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
            format!("reports/detailed_{problem_filename}_{optimizer_filename}.md");
        let optimizer_link = format!(
            r#"<a href="{detailed_report_filename}" title="Click for detailed analysis">{optimizer}</a>"#
        );

        // Create formatted strings for success/fail values
        let success_str = if mean_final_success.is_nan() || !mean_final_success.is_finite() {
            "-".to_string()
        } else {
            format!("{mean_final_success:.2e}")
        };
        let fail_str = if mean_final_fail.is_nan() || !mean_final_fail.is_finite() {
            "-".to_string()
        } else {
            format!("{mean_final_fail:.2e}")
        };
        let final_value_str = if mean_final.is_finite() {
            format!("{mean_final:.2e} / {success_str} / {fail_str}")
        } else {
            format!("- / {success_str} / {fail_str}")
        };
        // Create formatted strings for function evaluations
        let func_success_str =
            if mean_func_evals_success.is_nan() || !mean_func_evals_success.is_finite() {
                "-".to_string()
            } else {
                format!("{mean_func_evals_success:.1}")
            };
        let func_fail_str = if mean_func_evals_fail.is_nan() || !mean_func_evals_fail.is_finite() {
            "-".to_string()
        } else {
            format!("{mean_func_evals_fail:.1}")
        };
        let func_evals_str = format!("{mean_func_evals:.1} / {func_success_str} / {func_fail_str}");
        // Create formatted strings for gradient evaluations
        let grad_success_str =
            if mean_grad_evals_success.is_nan() || !mean_grad_evals_success.is_finite() {
                "-".to_string()
            } else {
                format!("{mean_grad_evals_success:.1}")
            };
        let grad_fail_str = if mean_grad_evals_fail.is_nan() || !mean_grad_evals_fail.is_finite() {
            "-".to_string()
        } else {
            format!("{mean_grad_evals_fail:.1}")
        };
        let grad_evals_str = format!("{mean_grad_evals:.1} / {grad_success_str} / {grad_fail_str}");

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
    let convergence_plot = format!("plots/{problem_filename}/convergence.png");
    let log_convergence_plot = format!("plots/{problem_filename}/log_convergence.png");
    // Check if convergence plot exists
    let string = format!("{plots_dir}/{problem_filename}/convergence.png");
    if Path::new(&string).exists() {
        section.push_str(&format!(
            r#"<img src="{convergence_plot}" alt="Convergence plot for {problem_name}" style="max-width: 48%; height: auto; margin: 1%;">
"#
        ));
    }
    let string = format!("{plots_dir}/{problem_filename}/log_convergence.png");
    if Path::new(&string).exists() {
        section.push_str(&format!(
            r#"<img src="{log_convergence_plot}" alt="Log convergence plot for {problem_name}" style="max-width: 48%; height: auto; margin: 1%;">
"#
        ));
    }
    section.push_str(&format!(
        r#"
**Figure:** Convergence plots for {problem_name} showing objective value vs iterations.
Left: Linear scale, Right: Log scale for better visualization of convergence behavior.

**Data:** [Linear scale data (CSV)](data/{problem_filename}_data.csv) | [Log scale data (CSV)](data/{problem_filename}_log_data.csv)

"#
    ));
    Ok(section)
}

pub fn generate_problem_specific_csvs(
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
        let csv_path = problems_dir.join(format!("{problem_name}_results.csv"));
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
    let report_path = reports_dir.join(format!("problem_analysis_{problem_filename}.md"));

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
        experiment_runner::get_family(&problem_name),
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

![Convergence Plot](../plots/{}.png)
![Log Convergence Plot](../plots/{}_log.png)

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
        let mean_func_evals = runs
            .iter()
            .map(|r| r.function_evaluations as f64)
            .sum::<f64>()
            / runs.len() as f64;
        let mean_time = runs
            .iter()
            .map(|r| r.execution_time.as_secs_f64())
            .sum::<f64>()
            / runs.len() as f64;

        perf_data.push((
            optimizer.clone(),
            mean_final,
            success_rate,
            mean_func_evals,
            mean_time,
        ));
    }

    if is_no_threshold_mode() {
        // In no-threshold mode, sort by mean final value
        perf_data.sort_by(|a, b| a.1.total_cmp(&b.1));
    } else {
        // Sort by success rate first, then by mean final value
        perf_data.sort_by(|a, b| match b.2.total_cmp(&a.2) {
            std::cmp::Ordering::Equal => a.1.total_cmp(&b.1),
            other => other,
        });
    }


    for (i, (optimizer, mean_final, success_rate, mean_func_evals, mean_time)) in
        perf_data.iter().enumerate()
    {
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

/// Generate problem-specific LaTeX table
pub fn generate_problem_latex_table(
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
        experiment_runner::escape_latex(&problem_name),
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
        let function_evals: Vec<f64> = runs.iter().map(|r| r.function_evaluations as f64).collect();
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
        let mean_function_evals = function_evals.iter().sum::<f64>() / function_evals.len() as f64;
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

    if is_no_threshold_mode() {
        // In no-threshold mode, sort by best value
        perf_data.sort_by(|a, b| {
            a.3.total_cmp(&b.3)
        });
    } else {
        // Sort by success rate first, then by total evaluations
        perf_data.sort_by(|a, b| {
            use std::cmp::Ordering;
            match b.7.total_cmp(&a.7) {
                Ordering::Equal => if b.7 > 0.0 { (a.5 + a.6).total_cmp(&(b.5 + b.6)) } else {
                    a.3.total_cmp(&b.3)
                },
                other => other,
            }
        });
    }

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
            format!("\\textbf{{{}}}", experiment_runner::escape_latex(optimizer))
        } else {
            experiment_runner::escape_latex(optimizer)
        };
        latex_content.push_str(&format!(
            "{optimizer_style} & {mean_final:.2e} & {std_final:.2e} & {best_final:.2e} & {worst_final:.2e} & {mean_func_evals:.1} & {success_rate:.1} & {mean_time:.3} \\\\\n"
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
    let latex_path = latex_dir.join(format!("{problem_filename}_performance.tex"));
    fs::write(&latex_path, latex_content)
        .with_context(|| format!("Failed to write LaTeX table to: {}", latex_path.display()))?;
    Ok(())
}
