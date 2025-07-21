use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults};
use std::collections::HashMap;

/// Handles statistical analysis and significance testing
pub struct StatisticalAnalysis;

impl StatisticalAnalysis {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_statistical_analysis(
        &self,
        all_results: &[(String, BenchmarkResults)],
        config: &BenchmarkConfig,
    ) -> anyhow::Result<String> {
        let mut section = String::from(
            r#"
    <div class="section">
        <h2>Statistical Analysis</h2>
        <div class="subsection">
            <h3>Pairwise Comparisons: QQN vs Non-QQN Optimizers</h3>
            <p>Statistical significance tests comparing QQN variants against non-QQN baseline optimizers on final objective values.</p>
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

        let mut optimizer_results: HashMap<String, Vec<f64>> = HashMap::new();
        for result in &combined_results.results {
            optimizer_results
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push(result.final_value);
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
                <tr>
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

        for qqn_opt in &qqn_optimizers {
            for non_qqn_opt in &non_qqn_optimizers {
                let values_qqn = &optimizer_results[qqn_opt];
                let values_non_qqn = &optimizer_results[non_qqn_opt];

                match self.welch_t_test(values_qqn, values_non_qqn) {
                    Ok((t_stat, p_value)) => {
                        let effect_size = self.cohens_d(values_qqn, values_non_qqn);
                        let significant = p_value < 0.05;
                        let significance_class = if significant { "best" } else { "" };

                        section.push_str(&format!(
                            r#"                <tr class="{}">
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
                            qqn_opt,
                            non_qqn_opt,
                            t_stat,
                            p_value,
                            if significant { "✓" } else { "✗" },
                            effect_size
                        ));
                        comparisons_made += 1;
                    }
                    Err(e) => {
                        section.push_str(&format!(
                            r#"                <tr>
                                <td>{}</td>
                                <td>{}</td>
                                <td colspan="5"><em>Test failed: {}</em></td>
                            </tr>
"#,
                            qqn_opt, non_qqn_opt, e
                        ));
                    }
                }
            }
        }

        if comparisons_made == 0 {
            section.push_str(r#"                <tr>
                    <td colspan="7"><em>No valid QQN vs non-QQN comparisons could be performed.</em></td>
                </tr>
"#);
        }

        section.push_str(r#"            </table>

            <div class="citation">
                <strong>Citation Note:</strong> Statistical tests performed using Welch's t-test comparing final objective values
                between QQN variants and non-QQN optimizers with α = 0.05. Effect sizes calculated using Cohen's d.
            </div>
        </div>
    </div>
"#);

        Ok(section)
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