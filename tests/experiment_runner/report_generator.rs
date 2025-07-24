use super::StatisticalAnalysis;
use anyhow::Context;
use log::warn;
use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults, SingleResult};
use qqn_optimizer::OptimizationProblem;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

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
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        _problems: &Vec<Arc<dyn OptimizationProblem>>,
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)
            .with_context(|| format!("Failed to create output directory: {}", self.output_dir))?;
        println!("Generating report in directory: {}", self.output_dir);
        // Generate detailed optimizer-problem reports first
        self.generate_detailed_reports(all_results).await?;

        let mut html_content = self.generate_header();
        // html_content.push_str(&self.generate_executive_summary(all_results));

        for (problem, results) in all_results {
            html_content.push_str(&self.generate_problem_section(problem, results)?);
        }

        if !all_results.is_empty() && all_results.iter().any(|(_, r)| !r.results.is_empty()) {
            html_content.push_str(&self.statistical_analysis.generate_statistical_analysis(
                all_results,
                &self.config,
                &self.output_dir,
            )?);
            //html_content.push_str(&self.generate_performance_profiles(all_results, _problems)?);
            //html_content.push_str(&self.generate_model_test_matrices(all_results)?);
        }

        html_content.push_str(&self.generate_conclusions(all_results));
        html_content.push_str(&self.generate_html_footer());

        let md_path = Path::new(&self.output_dir).join("benchmark_report.md");
        println!("Saving Markdown report to: {}", md_path.display());
        fs::write(&md_path, html_content.clone()).with_context(|| {
            format!("Failed to write Markdown report to: {}", md_path.display())
        })?;

        self.generate_csv_exports(all_results)?;

        Ok(())
    }

    fn generate_header(&self) -> String {
        format!(
            r#"# Quadratic Quasi-Newton (QQN) Optimizer: Experimental Validation

*Comprehensive Benchmark Results for Academic Publication*

**Generated on:** {}

"#,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    fn generate_problem_section(
        &self,
        problem: &Arc<dyn OptimizationProblem>,
        results: &BenchmarkResults,
    ) -> anyhow::Result<String> {
        let problem_name = problem.name();
        let optimal_value = problem.optimal_value().unwrap_or(f64::NEG_INFINITY);
        let dimension = problem.dimension();

        let mut section = format!(
            r#"## Problem: {}

**Family:** {}  
**Dimension:** {}  
**Success Threshold:** {:0.3e}  
**Total Runs:** {}

### Performance Summary
"#,
            problem_name,
            get_family(problem_name),
            dimension,
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
        }

        if !suspicious_results.is_empty() {
            section.push_str(
                r#"> ⚠️ **Suspicious/False Convergence Results Detected:**
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
                .map(|r| r.final_value)
                .filter(|&v| v.is_finite())
                .collect();
            if final_values.is_empty() {
                continue; // Skip optimizers with no valid results
            }

            let function_evals: Vec<f64> =
                runs.iter().map(|r| r.function_evaluations as f64).collect();
            let gradient_evals: Vec<f64> =
                runs.iter().map(|r| r.gradient_evaluations as f64).collect();
            let success_count = runs.iter().filter(|r| r.convergence_achieved).count();
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
            match a.7.partial_cmp(&b.7) {
                Some(ord) => {
                    let result = ord.reverse();
                    if result != Ordering::Equal {
                        return result;
                    }
                }
                None => match (a.7.is_nan(), b.7.is_nan()) {
                    (true, true) => {}
                    (true, false) => return Ordering::Greater,
                    (false, true) => return Ordering::Less,
                    (false, false) => unreachable!(),
                },
            }

            let is_failed = a.7.is_nan() || a.7 == 0.0;
            if is_failed {
                a.1.total_cmp(&b.1)
            } else {
                a.1.total_cmp(&b.1)
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
            let optimizer_link = format!("<a href=\"{}\">{}</a>", detailed_report_filename, optimizer);
            
            // Format the separated statistics
            let final_value_str = format!(
                "{:.2e} / {:.2e} / {:.2e}",
                mean_final,
                if mean_final_success.is_finite() {
                    *mean_final_success
                } else {
                    *mean_final
                },
                if mean_final_fail.is_finite() {
                    *mean_final_fail
                } else {
                    *mean_final
                }
            );
            let func_evals_str = format!(
                "{:.1} / {:.1} / {:.1}",
                mean_func_evals,
                if mean_func_evals_success.is_finite() {
                    *mean_func_evals_success
                } else {
                    *mean_func_evals
                },
                if mean_func_evals_fail.is_finite() {
                    *mean_func_evals_fail
                } else {
                    *mean_func_evals
                }
            );
            let grad_evals_str = format!(
                "{:.1} / {:.1} / {:.1}",
                mean_grad_evals,
                if mean_grad_evals_success.is_finite() {
                    *mean_grad_evals_success
                } else {
                    *mean_grad_evals
                },
                if mean_grad_evals_fail.is_finite() {
                    *mean_grad_evals_fail
                } else {
                    *mean_grad_evals
                }
            );

            section.push_str(&format!(
                r#"<tr style="{}">
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
        let convergence_path = Path::new(&self.output_dir).join(&convergence_plot);
        if convergence_path.exists() {
            section.push_str(&format!(
                r#"<img src="{}" alt="Convergence plot for {}" style="max-width: 48%; height: auto; margin: 1%;">
"#,
                convergence_plot, problem_name
            ));
        }
        // Check if log convergence plot exists
        let log_convergence_path = Path::new(&self.output_dir).join(&log_convergence_plot);
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

    fn generate_conclusions(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> String {
        let mut optimizer_scores = HashMap::new();
        let mut ml_optimizer_scores = HashMap::new();
        let mut math_optimizer_scores = HashMap::new();
        let mut optimizer_efficiency = HashMap::new();

        for (_, results) in all_results {
            for result in &results.results {
                let score = optimizer_scores
                    .entry(result.optimizer_name.clone())
                    .or_insert(0.0);
                if result.convergence_achieved {
                    *score += 1.0;
                }
                if result.final_value < 1e-6 {
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

        for (problem, results) in all_results {
            let problem_name = problem.name();
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
                if result.final_value < 1e-6 {
                    *score += 0.5;
                }
            }
        }

        let best_optimizer = optimizer_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        let most_efficient = optimizer_efficiency
            .iter()
            .map(|(name, (successes, total_time))| {
                let efficiency = *successes as f64 / total_time;
                (name.clone(), efficiency)
            })
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(name, _)| name)
            .unwrap_or_else(|| "Unknown".to_string());
        format!(
            r#"
            
## Conclusions

### Key Findings

- The **{}** optimizer demonstrated the best overall performance across the test suite.
- The **{}** optimizer showed the best efficiency (success rate per unit time).

"#,
            best_optimizer, most_efficient,
        )
    }

    fn generate_html_footer(&self) -> String {
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
            self.config.num_runs,
            self.config.min_improvement_percent,
            self.config.max_iterations,
            self.config.maximum_function_calls,
            self.config.time_limit.clone(),
            env!("CARGO_PKG_VERSION"),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    fn generate_csv_exports(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)
            .with_context(|| format!("Failed to create output directory: {}", self.output_dir))?;
        println!("Exporting CSV files to: {}", self.output_dir);

        // Enhanced CSV with more fields
        let mut csv_content = String::from("Problem,ProblemFamily,Dimension,Optimizer,Run,FinalValue,FinalGradientNorm,Iterations,FunctionEvals,GradientEvals,Time,Converged,ConvergenceReason\n");

        for (problem, results) in all_results {
            let problem_name = problem.name();
            let problem_family = get_family(problem_name);
            let dimension = problem.dimension();

            for result in &results.results {
                csv_content.push_str(&format!(
                    "{},{},{},{},{},{:.6e},{:.6e},{},{},{},{:.3},{},{:?}\n",
                    problem_name,
                    problem_family,
                    dimension,
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

        let csv_path = Path::new(&self.output_dir).join("detailed_results.csv");
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

                let problem_name = problem.name();
                let problem_family = get_family(problem_name);
                let dimension = problem.dimension();

                summary_csv.push_str(&format!(
                    "{},{},{},{},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.6e},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.1},{:.3},{:.3},{}\n",
                    problem_name,
                    problem_family,
                    dimension,
                    optimizer,
                    mean_final,
                    if mean_final_success.is_finite() { mean_final_success } else { mean_final },
                    if mean_final_fail.is_finite() { mean_final_fail } else { mean_final },
                    std_final,
                    best_final,
                    worst_final,
                    mean_iterations,
                    mean_function_evals,
                    if mean_func_evals_success.is_finite() { mean_func_evals_success } else { mean_function_evals },
                    if mean_func_evals_fail.is_finite() { mean_func_evals_fail } else { mean_function_evals },
                    mean_gradient_evals,
                    if mean_grad_evals_success.is_finite() { mean_grad_evals_success } else { mean_gradient_evals },
                    if mean_grad_evals_fail.is_finite() { mean_grad_evals_fail } else { mean_gradient_evals },
                    mean_time,
                    success_rate,
                    runs.len()
                ));
            }
        }

        let summary_path = Path::new(&self.output_dir).join("summary_statistics.csv");
        println!("Writing summary statistics to: {}", summary_path.display());
        fs::write(&summary_path, summary_csv).with_context(|| {
            format!("Failed to write summary CSV to: {}", summary_path.display())
        })?;

        // Generate problem-specific CSV files for easier analysis
        self.generate_problem_specific_csvs(all_results)?;

        Ok(())
    }
    fn generate_problem_specific_csvs(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        let problems_dir = Path::new(&self.output_dir).join("problems");
        fs::create_dir_all(&problems_dir).with_context(|| {
            format!(
                "Failed to create problems directory: {}",
                problems_dir.display()
            )
        })?;
        for (problem, results) in all_results {
            let problem_name = problem.name().replace(" ", "_");
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

    /// Generate detailed reports for each optimizer-problem combination
    async fn generate_detailed_reports(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        for (problem, results) in all_results {
            let mut optimizer_results = std::collections::HashMap::new();
            // Group results by optimizer
            for result in &results.results {
                let optimizer_results_vec = optimizer_results
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                optimizer_results_vec.push(result);
            }
            // Generate detailed report for each optimizer on this problem
            for (optimizer_name, optimizer_runs) in optimizer_results {
                self.generate_optimizer_problem_report(
                    problem.as_ref(),
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
        &self,
        problem: &dyn OptimizationProblem,
        optimizer_name: &str,
        runs: &[&SingleResult],
    ) -> anyhow::Result<()> {
        let problem_name = problem.name();
        let problem_filename = problem_name.replace(" ", "_");
        let optimizer_filename = optimizer_name.replace(" ", "_");
        let filename = format!("detailed_{}_{}.md", problem_filename, optimizer_filename);
        let filepath = Path::new(&self.output_dir).join(&filename);
        let mut content = self.generate_detailed_report_header(problem, optimizer_name, runs);
        content.push_str(&self.generate_run_by_run_analysis(runs)?);
        content.push_str(&self.generate_convergence_analysis(runs)?);
        content.push_str(&self.generate_parameter_evolution_analysis(runs)?);
        content.push_str(&self.generate_performance_analysis(runs)?);
        content.push_str(&self.generate_failure_analysis(runs)?);
        content.push_str(&self.generate_detailed_report_footer(problem_name, optimizer_name));
        fs::write(&filepath, content).with_context(|| {
            format!("Failed to write detailed report to: {}", filepath.display())
        })?;
        Ok(())
    }
    fn generate_detailed_report_header(
        &self,
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
- **Best Final Value:** {:.6e}
- **Worst Final Value:** {:.6e}
- **Mean Final Value:** {:.6e}
- **Success Rate:** {:.1}%
---
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
    fn generate_run_by_run_analysis(&self, runs: &[&SingleResult]) -> anyhow::Result<String> {
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
                .replace("NumericalError", "Num Err");
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
                    "✓"
                } else {
                    "✗"
                },
                convergence_reason
            ));
        }
        content.push_str("</table>\n\n");
        Ok(content)
    }
    fn generate_convergence_analysis(&self, runs: &[&SingleResult]) -> anyhow::Result<String> {
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
            content.push_str(&format!(
                r#"### Successful Runs ({} out of {})
- **Average Iterations to Convergence:** {:.1}
- **Average Function Evaluations:** {:.1}
- **Average Time to Convergence:** {:.3}s
- **Fastest Convergence:** {} iterations ({:.3}s)
- **Slowest Convergence:** {} iterations ({:.3}s)
"#,
                successful_runs.len(),
                runs.len(),
                mean_iterations,
                mean_func_evals,
                mean_time,
                iterations.iter().min().unwrap_or(&0),
                times
                    .iter()
                    .min_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0.0),
                iterations.iter().max().unwrap_or(&0),
                times
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(&0.0)
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
    fn generate_parameter_evolution_analysis(
        &self,
        runs: &[&SingleResult],
    ) -> anyhow::Result<String> {
        let mut content = String::from("## Parameter Evolution Analysis\n\n");
        // Find the run with the best final value for detailed analysis
        let best_run = runs
            .iter()
            .filter(|r| r.final_value.is_finite())
            .min_by(|a, b| a.final_value.partial_cmp(&b.final_value).unwrap());
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
                content.push_str("#### Parameter Evolution (Selected Iterations)\n\n");
                content.push_str("<table style=\"border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;\">\n");
                content.push_str("<tr style=\"background-color: #f2f2f2;\">\n");
                content.push_str(
                    "<th style=\"border: 1px solid #ddd; padding: 4px;\">Iteration</th>\n",
                );
                content.push_str(
                    "<th style=\"border: 1px solid #ddd; padding: 4px;\">Function Value</th>\n",
                );
                content.push_str(
                    "<th style=\"border: 1px solid #ddd; padding: 4px;\">Gradient Norm</th>\n",
                );
                content.push_str(
                    "<th style=\"border: 1px solid #ddd; padding: 4px;\">Step Size</th>\n",
                );
                content.push_str("<th style=\"border: 1px solid #ddd; padding: 4px;\">Parameters (first 5)</th>\n");
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
                        "<tr><td style=\"border: 1px solid #ddd; padding: 4px;\">{}</td><td style=\"border: 1px solid #ddd; padding: 4px;\">{:.3e}</td><td style=\"border: 1px solid #ddd; padding: 4px;\">{:.3e}</td><td style=\"border: 1px solid #ddd; padding: 4px;\">{:.3e}</td><td style=\"border: 1px solid #ddd; padding: 4px;\">[{}]</td></tr>\n",
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
    fn generate_performance_analysis(&self, runs: &[&SingleResult]) -> anyhow::Result<String> {
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
    fn generate_failure_analysis(&self, runs: &[&SingleResult]) -> anyhow::Result<String> {
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
            match run.convergence_reason {
                qqn_optimizer::benchmarks::evaluation::ConvergenceReason::TimeLimit => timeout_failures += 1,
                qqn_optimizer::benchmarks::evaluation::ConvergenceReason::NumericalError => numerical_failures += 1,
                qqn_optimizer::benchmarks::evaluation::ConvergenceReason::MaxIterations => max_iter_failures += 1,
                qqn_optimizer::benchmarks::evaluation::ConvergenceReason::MaxFunctionEvaluations => {
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
    fn generate_detailed_report_footer(&self, problem_name: &str, optimizer_name: &str) -> String {
        format!(
            r#"---
## Data Files
- [Raw CSV Data](problems/{}_results.csv)
- [Convergence Plot](convergence_{}.png)
- [Log Scale Convergence Plot](convergence_{}_log.png)
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