use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use qqn_optimizer::OptimizationProblem;
use super::report_generator::get_family;

/// Handles statistical analysis and significance testing
pub struct StatisticalAnalysis;

impl StatisticalAnalysis {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_statistical_analysis(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        config: &BenchmarkConfig,
        output_dir: &str,
    ) -> anyhow::Result<String> {
        let mut section = String::from(
            r#"
    <div class="section">
        <h2>Statistical Analysis</h2>
        <div class="subsection">
            <h3>Pairwise Comparisons: QQN vs Non-QQN Optimizers by Problem Family</h3>
            <p>Statistical significance tests comparing QQN variants against non-QQN baseline optimizers on final objective values and computational cost, aggregated by problem family for increased statistical power.</p>
"#,
        );

        let mut combined_results = BenchmarkResults::new(config.clone());
        for (_, results) in all_results {
            for result in &results.results {
                combined_results.add_result(result.clone());
            }
        }

        if combined_results.results.len() < 2 {
            section.push_str(
                r#"            <p><em>Insufficient data for statistical analysis.</em></p>
        </div>
    </div>
"#,
            );
            return Ok(section);
        }

        let mut optimizer_results: HashMap<String, Vec<(f64, f64, String)>> = HashMap::new(); // (final_value, cost, problem)
        for result in &combined_results.results {
            let cost = (result.function_evaluations.max(result.gradient_evaluations)) as f64;
            let problem_name = all_results.iter()
                .find(|(_, res)| res.results.iter().any(|r| 
                    r.optimizer_name == result.optimizer_name && 
                    r.run_id == result.run_id &&
                    (r.final_value - result.final_value).abs() < 1e-12
                ))
                .map(|(name, _)| name.name())
                .unwrap_or_else(|| "Unknown");
            
            optimizer_results
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push((result.final_value, cost, problem_name.to_string()));
        }

        optimizer_results.retain(|_, values| values.len() >= 2);
        if optimizer_results.len() < 2 {
            section.push_str(
                r#"            <p><em>Insufficient data for pairwise comparisons (need at least 2 optimizers with 2+ runs each).</em></p>
        </div>
    </div>
"#,
            );
            return Ok(section);
        }

        let mut qqn_optimizers = Vec::new();
        let mut non_qqn_optimizers = Vec::new();
        for optimizer_name in optimizer_results.keys() {
            if optimizer_name.contains("QQN") {
                qqn_optimizers.push(optimizer_name.clone());
            } else {
                non_qqn_optimizers.push(optimizer_name.clone());
            }
        }

        if qqn_optimizers.is_empty() || non_qqn_optimizers.is_empty() {
            section.push_str(
                r#"            <p><em>Need both QQN and non-QQN optimizers for comparison.</em></p>
        </div>
    </div>
"#,
            );
            return Ok(section);
        }

        section.push_str(
            r#"            <table>
<table>
                <tr>
                    <th>Problem Family</th>
                    <th>QQN Optimizer</th>
                    <th>Non-QQN Optimizer</th>
                    <th>Metric</th>
                    <th>Test Statistic</th>
                    <th>p-value</th>
                    <th>Significant</th>
                    <th>Effect Size</th>
                </tr>
"#,
        );

        let mut comparisons_made = 0;
        let mut csv_data = Vec::new();
        csv_data.push("Problem_Family,QQN_Optimizer,NonQQN_Optimizer,Metric,Test_Statistic,P_Value,Significant,Effect_Size".to_string());
        // Group results by problem family for per-family analysis
        let mut family_optimizer_results: HashMap<String, HashMap<String, Vec<(f64, f64)>>> = HashMap::new();
        for (optimizer, results) in &optimizer_results {
            for (final_value, cost, problem) in results {
                let family = get_family(problem);
                family_optimizer_results
                    .entry(family)
                    .or_insert_with(HashMap::new)
                    .entry(optimizer.clone())
                    .or_insert_with(Vec::new)
                    .push((*final_value, *cost));
            }
        }
        for (family_name, family_results) in &family_optimizer_results {
            let mut family_qqn_optimizers = Vec::new();
            let mut family_non_qqn_optimizers = Vec::new();
            for optimizer_name in family_results.keys() {
                if optimizer_name.contains("QQN") {
                    family_qqn_optimizers.push(optimizer_name.clone());
                } else {
                    family_non_qqn_optimizers.push(optimizer_name.clone());
                }
            }
            if family_qqn_optimizers.is_empty() || family_non_qqn_optimizers.is_empty() {
                continue;
            }

            for qqn_opt in &family_qqn_optimizers {
                for non_qqn_opt in &family_non_qqn_optimizers {
                    let qqn_results = &family_results[qqn_opt];
                    let non_qqn_results = &family_results[non_qqn_opt];
                    
                    if qqn_results.len() < 2 || non_qqn_results.len() < 2 {
                        continue;
                    }

                    // Extract final values for statistical testing
                    let qqn_final_values: Vec<f64> = qqn_results.iter().map(|(final_val, _)| *final_val).collect();
                    let non_qqn_final_values: Vec<f64> = non_qqn_results.iter().map(|(final_val, _)| *final_val).collect();
                    
                    match self.welch_t_test(&qqn_final_values, &non_qqn_final_values) {
                    Ok((t_stat, p_value)) => {
                            let effect_size = self.cohens_d(&qqn_final_values, &non_qqn_final_values);
                        let significant = p_value < 0.05;
                        let significance_class = if significant { "best" } else { "" };

                        section.push_str(&format!(
                            r#"                <tr class="{}">
                                <td>{}</td>
                                <td>{}</td>
                                <td>{}</td>
                                <td>Final Objective Value</td>
                                <td>{:.4}</td>
                                <td>{:.4}</td>
                                <td>{}</td>
                                <td>{:.3}</td>
                            </tr>
"#,
                            significance_class,
                                family_name,
                            qqn_opt,
                            non_qqn_opt,
                            t_stat,
                            p_value,
                            if significant { "✓" } else { "✗" },
                            effect_size
                        ));
                            csv_data.push(format!("{},{},{},Final_Objective_Value,{:.6},{:.6},{},{:.6}",
                                family_name, qqn_opt, non_qqn_opt, t_stat, p_value, significant, effect_size));
                        comparisons_made += 1;
                    }
                    Err(e) => {
                        section.push_str(&format!(
                            r#"                <tr>
                                <td>{}</td>
                                <td>{}</td>
                                <td>{}</td>
                                <td colspan="6"><em>Test failed: {}</em></td>
                            </tr>
"#,
                                family_name, qqn_opt, non_qqn_opt, e
                        ));
                    }
                }
                    // Test on computational cost
                    let costs_qqn: Vec<f64> = qqn_results.iter().map(|(_, c)| *c).collect();
                    let costs_non_qqn: Vec<f64> = non_qqn_results.iter().map(|(_, c)| *c).collect();
                    match self.welch_t_test(&costs_qqn, &costs_non_qqn) {
                        Ok((t_stat, p_value)) => {
                            let effect_size = self.cohens_d(&costs_qqn, &costs_non_qqn);
                            let significant = p_value < 0.05;
                            let significance_class = if significant { "best" } else { "" };
                            section.push_str(&format!(
                                r#"                <tr class="{}">
                                    <td>{}</td>
                                    <td>{}</td>
                                    <td>{}</td>
                                    <td>Computational Cost</td>
                                    <td>{:.4}</td>
                                    <td>{:.4}</td>
                                    <td>{}</td>
                                    <td>{:.3}</td>
                                </tr>
"#,
                                significance_class,
                                family_name,
                                qqn_opt,
                                non_qqn_opt,
                                t_stat,
                                p_value,
                                if significant { "✓" } else { "✗" },
                                effect_size
                            ));
                            csv_data.push(format!("{},{},{},Computational_Cost,{:.6},{:.6},{},{:.6}",
                                family_name, qqn_opt, non_qqn_opt, t_stat, p_value, significant, effect_size));
                            comparisons_made += 1;
                        }
                        Err(e) => {
                            section.push_str(&format!(
                                r#"                <tr>
                                    <td>{}</td>
                                    <td>{}</td>
                                    <td>{}</td>
                                    <td>Computational Cost</td>
                                    <td colspan="4"><em>Test failed: {}</em></td>
                                </tr>
"#,
                                family_name, qqn_opt, non_qqn_opt, e
                            ));
                        }
                    }
                }
            }
        }

        if comparisons_made == 0 {
            section.push_str(r#"                <tr>
                    <td colspan="8"><em>No valid QQN vs non-QQN comparisons could be performed.</em></td>
                </tr>
"#);
        }
        // Save CSV data
        if let Err(e) = self.save_statistical_analysis_csv(&csv_data, output_dir) {
            eprintln!("Warning: Failed to save statistical analysis CSV: {}", e);
        }

        section.push_str(r#"            </table>

            <div class="citation">
                <strong>Citation Note:</strong> Statistical tests performed using Welch's t-test comparing final objective values
                and computational cost (max of function/gradient evaluations) between QQN variants and non-QQN optimizers 
                with α = 0.05. Effect sizes calculated using Cohen's d. Analysis performed per problem family to account for 
                family-specific characteristics and increase statistical power by aggregating similar problems.
                <br><strong>Data:</strong> <a href="statistical_analysis_raw_data.csv">Raw statistical analysis data (CSV)</a>
            </div>
        </div>
    </div>
"#);

        Ok(section)
    }
    fn save_statistical_analysis_csv(&self, csv_data: &[String], output_dir: &str) -> anyhow::Result<()> {
        let csv_content = csv_data.join("\n");
        let csv_path = Path::new(output_dir).join("statistical_analysis_raw_data.csv");
        fs::write(csv_path, csv_content)?;
        Ok(())
    }

    fn welch_t_test(&self, sample_a: &[f64], sample_b: &[f64]) -> anyhow::Result<(f64, f64)> {
        if sample_a.len() < 2 || sample_b.len() < 2 {
            return Err(anyhow::anyhow!("Insufficient sample size for t-test"));
        }

        let mean_a = sample_a.iter().sum::<f64>() / sample_a.len() as f64;
        let mean_b = sample_b.iter().sum::<f64>() / sample_b.len() as f64;

        let var_a = sample_a.iter()
            .map(|x| (x - mean_a).powi(2))
            .sum::<f64>() / (sample_a.len() - 1) as f64;
        let var_b = sample_b.iter()
            .map(|x| (x - mean_b).powi(2))
            .sum::<f64>() / (sample_b.len() - 1) as f64;

        if var_a == 0.0 && var_b == 0.0 {
            if mean_a == mean_b {
                return Ok((0.0, 1.0));
            } else {
                return Err(anyhow::anyhow!("Zero variance with different means"));
            }
        }

        let se = (var_a / sample_a.len() as f64 + var_b / sample_b.len() as f64).sqrt();
        if se == 0.0 {
            return Err(anyhow::anyhow!("Zero standard error"));
        }

        let t_stat = (mean_a - mean_b) / se;

        let df = {
            let numerator = (var_a / sample_a.len() as f64 + var_b / sample_b.len() as f64).powi(2);
            let denom_a = (var_a / sample_a.len() as f64).powi(2) / (sample_a.len() - 1) as f64;
            let denom_b = (var_b / sample_b.len() as f64).powi(2) / (sample_b.len() - 1) as f64;
            numerator / (denom_a + denom_b)
        };

        let p_value = self.t_distribution_p_value(t_stat.abs(), df);
        Ok((t_stat, p_value))
    }

    fn t_distribution_p_value(&self, t_abs: f64, df: f64) -> f64 {
        if df > 30.0 {
            let z = t_abs;
            if z > 3.0 {
                0.001
            } else if z > 2.576 {
                0.01
            } else if z > 1.96 {
                0.05
            } else if z > 1.645 {
                0.10
            } else if z > 1.282 {
                0.20
            } else {
                0.50
            }
        } else {
            let critical_005 = if df >= 10.0 { 2.228 } else if df >= 5.0 { 2.571 } else { 3.182 };
            let critical_001 = if df >= 10.0 { 3.169 } else if df >= 5.0 { 4.032 } else { 5.841 };
            if t_abs > critical_001 {
                0.001
            } else if t_abs > critical_005 {
                0.01
            } else if t_abs > 2.0 {
                0.05
            } else if t_abs > 1.5 {
                0.10
            } else {
                0.50
            }
        }
    }

    fn cohens_d(&self, sample_a: &[f64], sample_b: &[f64]) -> f64 {
        if sample_a.len() < 2 || sample_b.len() < 2 {
            return 0.0;
        }

        let mean_a = sample_a.iter().sum::<f64>() / sample_a.len() as f64;
        let mean_b = sample_b.iter().sum::<f64>() / sample_b.len() as f64;

        let var_a = sample_a.iter()
            .map(|x| (x - mean_a).powi(2))
            .sum::<f64>() / (sample_a.len() - 1) as f64;
        let var_b = sample_b.iter()
            .map(|x| (x - mean_b).powi(2))
            .sum::<f64>() / (sample_b.len() - 1) as f64;

        let pooled_sd = ((var_a + var_b) / 2.0).sqrt();
        if pooled_sd == 0.0 {
            return 0.0;
        }

        (mean_a - mean_b) / pooled_sd
    }
}