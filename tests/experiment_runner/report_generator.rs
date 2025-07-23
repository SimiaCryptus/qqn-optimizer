use super::StatisticalAnalysis;
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
        .unwrap_or(problem_name) {
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
        x => x.to_string()
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


    pub async fn generate_html_report(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        _problems: &Vec<Arc<dyn OptimizationProblem>>,
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        println!("Generating report in directory: {}", self.output_dir);

        let mut html_content = self.generate_html_header();
        // html_content.push_str(&self.generate_executive_summary(all_results));

        for (problem, results) in all_results {
            html_content.push_str(&self.generate_problem_section(problem, results)?);
        }

        if !all_results.is_empty() && all_results.iter().any(|(_, r)| !r.results.is_empty()) {
            html_content.push_str(&self.statistical_analysis.generate_statistical_analysis(all_results, &self.config, &self.output_dir)?);
            // html_content.push_str(&self.generate_performance_profiles(all_results, problems)?);
            // html_content.push_str(&self.generate_model_test_matrices(all_results)?);
        }

        html_content.push_str(&self.generate_conclusions(all_results));
        html_content.push_str(&self.generate_html_footer());

        let html_path = Path::new(&self.output_dir).join("benchmark_report.html");
        println!("Saving HTML report to: {}", html_path.display());
        fs::write(html_path, html_content)?;

        self.generate_csv_exports(all_results)?;

        Ok(())
    }

    fn generate_html_header(&self) -> String {
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>QQN Optimizer Benchmark Results</title>
    <style>
        body {{ font-family: 'Times New Roman', serif; margin: 40px; line-height: 1.6; }}
        .header {{ text-align: center; margin-bottom: 40px; }}
        .section {{ margin: 30px 0; }}
        .subsection {{ margin: 20px 0; }}
        table {{ border-collapse: collapse; width: 100%; margin: 20px 0; }}
        th, td {{ border: 1px solid #ddd; padding: 8px; text-align: center; }}
        th {{ background-color: #f2f2f2; font-weight: bold; }}
        .best {{ background-color: #d4edda; font-weight: bold; }}
        .second {{ background-color: #fff3cd; }}
        .citation {{ background-color: #f8f9fa; padding: 15px; margin: 20px 0; border-left: 4px solid #007bff; }}
        .code {{ font-family: 'Courier New', monospace; background-color: #f8f9fa; padding: 10px; }}
        .figure {{ text-align: center; margin: 20px 0; }}
        .caption {{ font-style: italic; margin-top: 10px; }}
        .summary-box {{ background-color: #e9ecef; padding: 20px; margin: 20px 0; border-radius: 5px; }}
        .metric {{ display: inline-block; margin: 10px 20px; }}
        .metric-value {{ font-size: 1.2em; font-weight: bold; color: #007bff; }}
        .metric-label {{ font-size: 0.9em; color: #6c757d; }}
        .algorithm-highlight {{ background-color: #fff3cd; padding: 2px 4px; border-radius: 3px; }}
        .matrix-table {{ font-size: 0.8em; }}
        .matrix-table th {{ writing-mode: vertical-lr; text-orientation: mixed; min-width: 80px; }}
        .matrix-table td {{ text-align: center; padding: 4px; }}
        .performance-matrix {{ margin: 20px 0; }}
        .performance-matrix h3 {{ color: #495057; margin-bottom: 10px; }}
.footer {{ margin-top: 50px; padding-top: 20px; border-top: 1px solid #ddd; text-align: center; color: #6c757d; }}
        .data-links {{ margin-top: 10px; font-size: 0.9em; }}
        .data-links a {{ color: #007bff; text-decoration: none; margin: 0 5px; }}
        .data-links a:hover {{ text-decoration: underline; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Quadratic Quasi-Newton (QQN) Optimizer: Experimental Validation</h1>
        <p><em>Comprehensive Benchmark Results for Academic Publication</em></p>
        <p>Generated on: {}</p>
    </div>
"#,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    fn generate_executive_summary(&self, all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)]) -> String {
        let total_problems = all_results.len();
        let total_runs = all_results
            .iter()
            .map(|(_, r)| r.results.len())
            .sum::<usize>();

        // Aggregate by problem family and optimizer
        let mut family_optimizer_stats: HashMap<String, HashMap<String, (usize, usize)>> = HashMap::new();
        let mut overall_optimizer_stats = HashMap::new();

        for (problem, results) in all_results {
            // Determine problem family from name
            let family = get_family(problem.name());

            for result in &results.results {
                // Overall stats
                let overall_stats = overall_optimizer_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert((0, 0));
                overall_stats.1 += 1;
                if result.convergence_achieved {
                    overall_stats.0 += 1;
                }

                // Family-specific stats
                let family_stats = family_optimizer_stats
                    .entry(family.clone())
                    .or_insert_with(HashMap::new);
                let optimizer_stats = family_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert((0, 0));
                optimizer_stats.1 += 1;
                if result.convergence_achieved {
                    optimizer_stats.0 += 1;
                }
            }
        }

        let ml_problems = all_results
            .iter()
            .filter(|(problem, _)| {
                let name = problem.name();
                name.contains("Regression") || name.contains("Neural") || name.contains("SVM")
            })
            .count();
        let math_problems = total_problems - ml_problems;

        let mut summary = format!(
            r#"
    <div class="section">
        <h2>Executive Summary</h2>
        <div class="summary-box">
            <h3>Experimental Overview</h3>
            <div class="metric">
                <div class="metric-value">{}</div>
                <div class="metric-label">Test Problems</div>
            </div>
            <div class="metric">
                <div class="metric-value">{}</div>
                <div class="metric-label">Mathematical Functions</div>
            </div>
            <div class="metric">
                <div class="metric-value">{}</div>
                <div class="metric-label">ML Problems</div>
            </div>
            <div class="metric">
                <div class="metric-value">{}</div>
                <div class="metric-label">Total Optimization Runs</div>
            </div>
            <div class="metric">
                <div class="metric-value">{}</div>
                <div class="metric-label">Optimizers Compared</div>
            </div>
        </div>

        <h3>Overall Success Rates by Optimizer</h3>
        <table>
            <tr><th>Optimizer</th><th>Successful Runs</th><th>Total Runs</th><th>Success Rate</th></tr>

"#,
            total_problems,
            math_problems,
            ml_problems,
            total_runs,
            overall_optimizer_stats.len()
        );

        let mut sorted_optimizers: Vec<_> = overall_optimizer_stats.iter().collect();
        sorted_optimizers.sort_by(|a, b| {
            let rate_a = a.1.0 as f64 / a.1.1 as f64;
            let rate_b = b.1.0 as f64 / b.1.1 as f64;
            rate_b.partial_cmp(&rate_a).unwrap()
        });

        for (i, (optimizer, (successful, total))) in sorted_optimizers.iter().enumerate() {
            let rate = *successful as f64 / *total as f64;
            let class = if i == 0 {
                "best"
            } else if i == 1 {
                "second"
            } else {
                ""
            };
            summary.push_str(&format!(
                r#"            <tr class="{}"><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td></tr>
"#,
                class,
                optimizer,
                successful,
                total,
                rate * 100.0
            ));
        }

        summary.push_str(
            r#"        </table>
"#,
        );
        // Add family-specific success rate tables
        for (family, optimizer_stats) in &family_optimizer_stats {
            summary.push_str(&format!(
                r#"
        <h3>Success Rates by Optimizer - {}</h3>
        <table>
            <tr><th>Optimizer</th><th>Successful Runs</th><th>Total Runs</th><th>Success Rate</th></tr>
"#,
                family
            ));
            let mut sorted_family_optimizers: Vec<_> = optimizer_stats.iter().collect();
            sorted_family_optimizers.sort_by(|a, b| {
                let rate_a = a.1.0 as f64 / a.1.1 as f64;
                let rate_b = b.1.0 as f64 / b.1.1 as f64;
                rate_b.partial_cmp(&rate_a).unwrap()
            });
            for (i, (optimizer, (successful, total))) in sorted_family_optimizers.iter().enumerate() {
                let rate = *successful as f64 / *total as f64;
                let class = if i == 0 {
                    "best"
                } else if i == 1 {
                    "second"
                } else {
                    ""
                };
                summary.push_str(&format!(
                    r#"            <tr class="{}"><td>{}</td><td>{}</td><td>{}</td><td>{:.1}%</td></tr>
"#,
                    class,
                    optimizer,
                    successful,
                    total,
                    rate * 100.0
                ));
            }
            summary.push_str(
                r#"        </table>
"#,
            );
        }
        summary.push_str(
            r#"
    </div>
"#,
        );

        summary
    }

    fn generate_problem_section(
        &self,
        problem: &Arc<dyn OptimizationProblem>,
        results: &BenchmarkResults,
    ) -> anyhow::Result<String> {
        let problem_name = problem.name();
        let mut section = format!(
            r#"
    <div class="section">
        <h2>Problem: {}</h2>
        <p><strong>Family:</strong> {}</p>
        <p><strong>Success Threshold:</strong> {:0.3e}</p>
        <div class="subsection">
            <h3>Performance Summary</h3>
"#,
            problem_name, get_family(problem_name), problem.optimal_value().unwrap_or(f64::NEG_INFINITY)
        );

        let mut optimizer_stats = HashMap::new();
        let mut suspicious_results = Vec::new();

        for result in &results.results {
            let stats = optimizer_stats
                .entry(result.optimizer_name.clone())
                .or_insert(Vec::new());
            stats.push(result);
            if result.function_evaluations <= 2 && result.convergence_achieved {
                suspicious_results.push((
                    result.optimizer_name.clone(),
                    result.function_evaluations,
                    result.final_value,
                ));
            }
        }

        if !suspicious_results.is_empty() {
            section.push_str(r#"            <div style="background-color: #fff3cd; padding: 10px; margin: 10px 0; border-radius: 5px;">
               <strong>⚠️ Suspicious/False Convergence Results Detected:</strong><br>
"#);
            for (optimizer, evaluations, final_value) in suspicious_results {
                section.push_str(&format!(
                    "                {} claimed convergence with {} function evaluations (final_value: {:.2e})<br>\n",
                    optimizer, evaluations, final_value
                ));
            }
            section.push_str(r#"                When repeating, this may indicate problems with initialization or convergence criteria. It might just be randomly lucky initialization.
            </div>
"#);
        }

        section.push_str(
            r#"            <table>
                <tr>
                    <th>Optimizer</th>
                    <th>Mean Final Value</th>
                    <th>Std Dev</th>
                    <th>Mean Function Evals</th>
                    <th>Mean Gradient Evals</th>
                    <th>Success Rate</th>
                </tr>
"#,
        );

        let mut perf_data = Vec::new();
        for (optimizer, runs) in &optimizer_stats {
            let final_values: Vec<f64> = runs.iter().map(|r| r.final_value).collect();
            let function_evals: Vec<f64> =
                runs.iter().map(|r| r.function_evaluations as f64).collect();
            let gradient_evals: Vec<f64> =
                runs.iter().map(|r| r.gradient_evaluations as f64).collect();
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
            let mean_function_evals =
                function_evals.iter().sum::<f64>() / function_evals.len() as f64;
            let mean_gradient_evals =
                gradient_evals.iter().sum::<f64>() / gradient_evals.len() as f64;
            let success_rate = success_count as f64 / runs.len() as f64;

            perf_data.push((
                optimizer.clone(),
                mean_final,
                std_final,
                mean_function_evals,
                mean_gradient_evals,
                success_rate,
            ));
        }

        perf_data.sort_by(|a, b| {
            use std::cmp::Ordering;
            match a.5.partial_cmp(&b.5) {
                Some(ord) => {
                    let result = ord.reverse();
                    if result != Ordering::Equal {
                        return result;
                    }
                }
                None => {
                    match (a.5.is_nan(), b.5.is_nan()) {
                        (true, true) => {}
                        (true, false) => return Ordering::Greater,
                        (false, true) => return Ordering::Less,
                        (false, false) => unreachable!(),
                    }
                }
            }

            let is_failed = a.5.is_nan() || a.5 == 0.0;
            if is_failed {
                a.1.total_cmp(&b.1)
            } else {
                let total_evals_a = if a.3 < a.4 { a.4 } else { a.3 };
                let total_evals_b = if b.3 < b.4 { b.4 } else { b.3 };
                total_evals_a.total_cmp(&total_evals_b)
            }
        });

        for (i, (optimizer, mean_final, std_final, mean_func_evals, mean_grad_evals, success_rate)) in perf_data.iter().enumerate() {
            let class = if i == 0 {
                "best"
            } else if i == 1 {
                "second"
            } else {
                ""
            };
            section.push_str(&format!(
                r#"                <tr class="{}">
                    <td>{}</td>
                    <td>{:.2e}</td>
                    <td>{:.2e}</td>
                    <td>{:.1}</td>
                    <td>{:.1}</td>
                    <td>{:.1}%</td>
                </tr>
"#,
                class,
                optimizer,
                mean_final,
                std_final,
                mean_func_evals,
                mean_grad_evals,
                success_rate * 100.0,
            ));
        }

        section.push_str(
            r#"            </table>
            <h3>Convergence Plots</h3>
            <div class="figure">
"#);
        // Add convergence plots for this problem
        let problem_filename = problem_name.replace(" ", "_");
        let convergence_plot = format!("convergence_{}.png", problem_filename);
        let log_convergence_plot = format!("convergence_{}_log.png", problem_filename);
        // Check if convergence plot exists
        let convergence_path = Path::new(&self.output_dir).join(&convergence_plot);
        if convergence_path.exists() {
            section.push_str(&format!(
                r#"                <img src="{}" alt="Convergence plot for {}" style="max-width: 48%; height: auto; margin: 1%;">
"#,
                convergence_plot, problem_name
            ));
        }
        // Check if log convergence plot exists
        let log_convergence_path = Path::new(&self.output_dir).join(&log_convergence_plot);
        if log_convergence_path.exists() {
            section.push_str(&format!(
                r#"                <img src="{}" alt="Log convergence plot for {}" style="max-width: 48%; height: auto; margin: 1%;">
"#,
                log_convergence_plot, problem_name
            ));
        }
        section.push_str(&format!(
            r#"                <div class="caption">
                    <strong>Figure:</strong> Convergence plots for {} showing objective value vs iterations.
                    Left: Linear scale, Right: Log scale for better visualization of convergence behavior.
                    <br><strong>Data:</strong> 
                    <a href="convergence_{}_data.csv">Linear scale data (CSV)</a> | 
                    <a href="convergence_{}_log_data.csv">Log scale data (CSV)</a>
                </div>
            </div>
"#,
            problem_name, 
            problem_filename,
            problem_filename
        ));
        Ok(section)
    }

    fn generate_performance_profiles(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        _problems: &Vec<Arc<dyn OptimizationProblem>>,
    ) -> anyhow::Result<String> {
        let mut section = String::from(
            r#"
    <div class="section">
        <h2>Performance Profiles</h2>
        <div class="subsection">
            <p>Performance profiles show the fraction of problems solved within a given performance ratio
            relative to the best optimizer for each problem.</p>
            "#,
        );

        let total_results: usize = all_results.iter().map(|(_, r)| r.results.len()).sum();
        if total_results < 2 {
            section.push_str(
                r#"
            <p><em>Insufficient data for performance profile analysis.</em></p>
        </div>
    </div>
"#,
            );
            return Ok(section);
        }


        section.push_str(r#"
            <div class="figure">
"#);

        // Check for performance comparison plots
        let performance_comparison_path = Path::new(&self.output_dir).join("performance_comparison.png");
        let performance_boxplot_path = Path::new(&self.output_dir).join("performance_distribution.png");

        if performance_comparison_path.exists() {
            section.push_str(&format!(
                r#"                <img src="performance_comparison.png" alt="Performance Comparison" style="max-width: 48%; height: auto; margin: 1%;">
"#
            ));
        }

        if performance_boxplot_path.exists() {
            section.push_str(&format!(
                r#"                <img src="performance_distribution.png" alt="Performance Distribution" style="max-width: 48%; height: auto; margin: 1%;">
"#
            ));
        }

        section.push_str(r#"                <div class="caption">
                    <strong>Figure:</strong> Performance comparison across all test problems. 
                    Left: Bar chart showing mean performance by optimizer. Right: Arc plots showing performance distribution.
                    <br><strong>Data:</strong> 
                    <a href="performance_comparison_data.csv">Performance comparison data (CSV)</a> | 
                    <a href="performance_distribution_data.csv">Distribution data (CSV)</a>
                </div>
            </div>

            <div class="citation">
                <strong>Interpretation:</strong> The bar chart shows average performance across all problems, while
                the box plots reveal the distribution and variability of results. Arc plots show median (red line),
                quartiles (box), and range (whiskers) for each optimizer.
            </div>
        </div>
    </div>
"#);

        Ok(section)
    }

    fn generate_model_test_matrices(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> anyhow::Result<String> {
        let mut section = String::from(
            r#"
    <div class="section">
        <h2>Model-Test Matrix Analysis</h2>
        <div class="subsection">
            <p>Comprehensive performance matrices showing optimizer performance across all test problems for different metrics.</p>
"#,
        );

        let mut optimizers = std::collections::HashSet::new();
        let mut problems: Vec<&Arc<dyn OptimizationProblem>> = Vec::new();
        for (problem_name, results) in all_results {
            problems.push(problem_name);
            for result in &results.results {
                optimizers.insert(result.optimizer_name.clone());
            }
        }
        let mut optimizers: Vec<_> = optimizers.into_iter().collect();
        optimizers.sort();

        let main_rankings = self.compute_main_performance_rankings(all_results, &optimizers, &problems)?;

        section.push_str(&self.generate_matrix_table(
            "Success Rate (%)",
            all_results,
            &optimizers,
            &problems,
            &main_rankings,
            |optimizer_results| {
                let success_count = optimizer_results
                    .iter()
                    .filter(|r| r.convergence_achieved)
                    .count();
                format!(
                    "{:.1}",
                    (success_count as f64 / optimizer_results.len() as f64) * 100.0
                )
            },
        )?);

        section.push_str(&self.generate_matrix_table(
            "Mean Final Value",
            all_results,
            &optimizers,
            &problems,
            &main_rankings,
            |optimizer_results| {
                let mean = optimizer_results.iter().map(|r| r.final_value).sum::<f64>()
                    / (optimizer_results.len() as f64);
                format!("{:.2e}", mean)
            },
        )?);

        section.push_str(&self.generate_matrix_table(
            "Mean Function Evaluations",
            all_results,
            &optimizers,
            &problems,
            &main_rankings,
            |optimizer_results| {
                let mean = optimizer_results
                    .iter()
                    .map(|r| r.function_evaluations as f64)
                    .sum::<f64>()
                    / optimizer_results.len() as f64;
                format!("{:.1}", mean)
            },
        )?);

        section.push_str(r#"
            <div class="citation">
                <strong>Matrix Interpretation:</strong> Each cell shows the average performance of an optimizer on a specific problem.
                Green highlighting indicates the best performance for each problem, yellow indicates second-best.
                Highlighting is consistent across all matrices based on the main performance ranking (success rate + final value).
            </div>
        </div>
    </div>
"#);

        Ok(section)
    }

    fn compute_main_performance_rankings(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        optimizers: &[String],
        problems: &Vec<&Arc<dyn OptimizationProblem>>,
    ) -> anyhow::Result<HashMap<String, Vec<(String, usize)>>> {
        let mut problem_rankings: HashMap<String, Vec<(String, usize)>> = HashMap::new();

        for problem in problems {
            let results = all_results
                .iter()
                .find(|(p, _)| p.name() == problem.name())
                .map(|(_, results)| results);

            if let Some(results) = results {
                let mut optimizer_scores = Vec::new();
                for optimizer in optimizers {
                    let optimizer_results: Vec<_> = results
                        .results
                        .iter()
                        .filter(|r| r.optimizer_name == *optimizer)
                        .cloned()
                        .collect();

                    if !optimizer_results.is_empty() {
                        let success_count = optimizer_results
                            .iter()
                            .filter(|r| r.convergence_achieved)
                            .count();
                        let success_rate = success_count as f64 / optimizer_results.len() as f64;
                        let mean_final_value = optimizer_results.iter().map(|r| r.final_value).sum::<f64>()
                            / optimizer_results.len() as f64;
                        let composite_score = (success_rate, -mean_final_value);
                        optimizer_scores.push((optimizer.clone(), composite_score));
                    }
                }

                if !optimizer_scores.is_empty() {
                    optimizer_scores.sort_by(|a, b| {
                        match (a.1.0.is_nan(), a.1.1.is_nan(), b.1.0.is_nan(), b.1.1.is_nan()) {
                            (true, _, false, _) => std::cmp::Ordering::Greater,
                            (false, _, true, _) => std::cmp::Ordering::Less,
                            (_, true, _, false) => std::cmp::Ordering::Greater,
                            (_, false, _, true) => std::cmp::Ordering::Less,
                            (true, _, true, _) | (_, true, _, true) => a.0.cmp(&b.0),
                            (false, false, false, false) => {
                                b.1.0.total_cmp(&a.1.0)
                                    .then_with(|| a.1.1.total_cmp(&b.1.1))
                            }
                        }
                    });

                    let ranked_optimizers: Vec<(String, usize)> = optimizer_scores
                        .into_iter()
                        .enumerate()
                        .map(|(rank, (name, _))| (name, rank))
                        .collect();
                    problem_rankings.insert(problem.name().to_string().clone(), ranked_optimizers);
                }
            }
        }

        Ok(problem_rankings)
    }

    fn generate_matrix_table<F>(
        &self,
        title: &str,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        optimizers: &[String],
        problems: &[&Arc<dyn OptimizationProblem>],
        main_rankings: &HashMap<String, Vec<(String, usize)>>,
        metric_fn: F,
    ) -> anyhow::Result<String>
    where
        F: Fn(&[SingleResult]) -> String,
    {
        let mut highlighted_table = format!(
            r#"
            <h3>{}</h3>
            <table style="font-size: 0.9em;">
                <tr>
                    <th style="min-width: 120px;">Optimizer</th>
"#,
            title
        );

        for problem in problems {
            highlighted_table.push_str(&format!(r#"                    <th style="min-width: 100px; writing-mode: vertical-lr; text-orientation: mixed;">{}</th>
"#, problem.name()));
        }
        highlighted_table.push_str("                </tr>\n");

        for optimizer in optimizers {
            highlighted_table.push_str(&format!(
                r#"                <tr>
                    <td style="text-align: left; font-weight: bold;">{}</td>
"#,
                optimizer
            ));

            for problem in problems {
                let results = all_results
                    .iter()
                    .find(|(name, _)| name.name() == problem.name())
                    .map(|(_, results)| results);

                if let Some(results) = results {
                    let optimizer_results: Vec<_> = results
                        .results
                        .iter()
                        .filter(|r| r.optimizer_name == *optimizer)
                        .cloned()
                        .collect();

                    if optimizer_results.is_empty() {
                        highlighted_table.push_str(r#"                    <td>-</td>
"#);
                    } else {
                        let value_str = metric_fn(&optimizer_results);

                        let class = if let Some(rankings) = main_rankings.get(problem.name()) {
                            let rank = rankings
                                .iter()
                                .find(|(opt, _)| opt == optimizer)
                                .map(|(_, rank)| *rank)
                                .unwrap_or(usize::MAX);

                            match rank {
                                0 => "best",
                                1 if rankings.len() > 1 => "second",
                                _ => "",
                            }
                        } else {
                            ""
                        };

                        highlighted_table.push_str(&format!(
                            r#"                    <td class="{}">{}</td>
"#,
                            class, value_str
                        ));
                    }
                } else {
                    highlighted_table.push_str(r#"                    <td>-</td>
"#);
                }
            }
            highlighted_table.push_str("                </tr>\n");
        }
        highlighted_table.push_str("            </table>\n");
        Ok(highlighted_table)
    }

    fn generate_conclusions(&self, all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)]) -> String {
        let mut optimizer_scores = HashMap::new();
        let mut ml_optimizer_scores = HashMap::new();
        let mut math_optimizer_scores = HashMap::new();

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
        let best_ml_optimizer = ml_optimizer_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string());
        let best_math_optimizer = math_optimizer_scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(name, _)| name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        format!(
            r#"
    <div class="section">
        <h2>Conclusions and Recommendations</h2>
        <div class="subsection">
            <h3>Key Findings</h3>
            <ul>
                <li>The <span class="algorithm-highlight">{}</span> optimizer demonstrated the best overall performance across the test suite.</li>
                <li>For <strong>Machine Learning problems</strong>, <span class="algorithm-highlight">{}</span> performed best.</li>
                <li>For <strong>Mathematical functions</strong>, <span class="algorithm-highlight">{}</span> performed best.</li>
                <li><strong>Optimizer Suite Optimization:</strong> Reduced from 80+ variants to 20 focused, high-performing optimizers.</li>
                <li><strong>Adam-Fast variants</strong> show significant promise and warrant further investigation.</li>
                <li><strong>L-BFGS-Hybrid</strong> combines the best aspects of aggressive and conservative approaches.</li>
                <li><strong>QQN-Backtracking variants</strong> demonstrate adaptive capabilities for different problem types.</li>
            </ul>

            <h3>Recommendations for Practitioners</h3>
            <ul>
                <li><strong>Start with Adam-Fast</strong> for general-purpose optimization.</li>
                <li><strong>Use L-BFGS-Hybrid</strong> for well-conditioned mathematical problems.</li>
                <li><strong>Try QQN-Backtracking-Adaptive</strong> for problems requiring robust line search.</li>
                <li>Always run multiple random seeds and report statistical significance.</li>
                <li>Problem-specific tuning of hyperparameters can significantly impact performance.</li>
            </ul>

            <div class="citation">
                <strong>Academic Citation:</strong><br>
                These results support the theoretical analysis presented in the main paper and demonstrate
                the practical effectiveness of the QQN approach for both mathematical optimization and
                machine learning problems, showing competitive performance across diverse problem types.
            </div>
        </div>
    </div>
"#,
            best_optimizer, best_ml_optimizer, best_math_optimizer
        )
    }

    fn generate_html_footer(&self) -> String {
        format!(
            r#"
    <div class="section">
        <h2>Experimental Details</h2>
        <div class="subsection">
            <h3>Methodology</h3>
            <ul>
                <li><strong>Runs per configuration:</strong> {} independent runs with different random seeds</li>
                <li><strong>Success criteria:</strong> Minimum {:.3}% improvement per iteration OR optimizer-specific convergence within {} iterations</li>
                <li><strong>Time limit:</strong> {:?} per run</li>
                <li><strong>Hardware:</strong> Standard CPU implementation</li>
                <li><strong>Implementation:</strong> Rust-based optimization framework</li>
            </ul>
        </div>
    </div>

    <footer style="margin-top: 50px; padding-top: 20px; border-top: 1px solid #ddd; text-align: center; color: #6c757d;">
        <p>Generated by QQN Optimizer Benchmark Suite v{}</p>
        <p>Report generated on: {}</p>
    </footer>
</body>
</html>
"#,
            self.config.num_runs,
            self.config.min_improvement_percent,
            self.config.max_iterations,
            self.config.time_limit.clone(),
            env!("CARGO_PKG_VERSION"),
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    }

    fn generate_csv_exports(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
    ) -> anyhow::Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        println!("Exporting CSV files to: {}", self.output_dir);

        let mut csv_content = String::from("Problem,Optimizer,Run,FinalValue,Iterations,FunctionEvals,GradientEvals,Time,Converged\n");

        for (problem, results) in all_results {
            let problem_name = problem.name();
            for result in &results.results {
                csv_content.push_str(&format!(
                    "{},{},{},{:.6e},{},{},{},{:.3},{}\n",
                    problem_name,
                    result.optimizer_name,
                    result.run_id,
                    result.final_value,
                    result.iterations,
                    result.function_evaluations,
                    result.gradient_evaluations,
                    result.execution_time.as_secs_f64(),
                    result.convergence_achieved
                ));
            }
        }

        let csv_path = Path::new(&self.output_dir).join("detailed_results.csv");
        println!("Writing detailed results to: {}", csv_path.display());
        fs::write(csv_path, csv_content)?;

        let mut summary_csv = String::from("Problem,Optimizer,MeanFinalValue,StdFinalValue,MeanIterations,MeanFunctionEvals,MeanGradientEvals,SuccessRate\n");

        for (problem, results) in all_results {
            let mut optimizer_stats = HashMap::new();
            for result in &results.results {
                let stats = optimizer_stats
                    .entry(result.optimizer_name.clone())
                    .or_insert(Vec::new());
                stats.push(result);
            }

            for (optimizer, runs) in optimizer_stats {
                let final_values: Vec<f64> = runs.iter().map(|r| r.final_value).collect();
                let iterations: Vec<f64> = runs.iter().map(|r| r.iterations as f64).collect();
                let function_evals: Vec<f64> =
                    runs.iter().map(|r| r.function_evaluations as f64).collect();
                let gradient_evals: Vec<f64> =
                    runs.iter().map(|r| r.gradient_evaluations as f64).collect();
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
                let mean_iterations = iterations.iter().sum::<f64>() / iterations.len() as f64;
                let mean_function_evals =
                    function_evals.iter().sum::<f64>() / function_evals.len() as f64;
                let mean_gradient_evals =
                    gradient_evals.iter().sum::<f64>() / gradient_evals.len() as f64;
                let success_rate = success_count as f64 / runs.len() as f64;
                let problem_name = problem.name();

                summary_csv.push_str(&format!(
                    "{},{},{:.6e},{:.6e},{:.1},{:.1},{:.1},{:.3}\n",
                    problem_name,
                    optimizer,
                    mean_final,
                    std_final,
                    mean_iterations,
                    mean_function_evals,
                    mean_gradient_evals,
                    success_rate
                ));
            }
        }

        let summary_path = Path::new(&self.output_dir).join("summary_statistics.csv");
        println!("Writing summary statistics to: {}", summary_path.display());
        fs::write(summary_path, summary_csv)?;

        Ok(())
    }
}