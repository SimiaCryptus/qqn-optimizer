use qqn_optimizer::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults};
use qqn_optimizer::OptimizationProblem;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::Arc;

/// Handles statistical analysis and significance testing
pub struct StatisticalAnalysis;

impl StatisticalAnalysis {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_statistical_analysis(
        &self,
        all_results: &[(&Arc<dyn OptimizationProblem>, BenchmarkResults)],
        _config: &BenchmarkConfig,
        output_dir: &str,
    ) -> anyhow::Result<String> {
        let mut section = String::from(r#""#, );

        // Check if we have enough data
        let total_results: usize = all_results.iter().map(|(_, r)| r.results.len()).sum();
        if total_results < 2 {
            return Ok(section);
        }

        let mut optimizer_results: HashMap<String, Vec<(f64, f64, String)>> = HashMap::new(); // (final_value, cost, problem)
        for (problem, results) in all_results {
            let problem_name = problem.name();
            for result in &results.results {
                let cost = (result.function_evaluations.max(result.gradient_evaluations)) as f64;

                optimizer_results
                    .entry(result.optimizer_name.clone())
                    .or_insert_with(Vec::new)
                    .push((result.final_value, cost, problem_name.to_string()));
            }
        }

        optimizer_results.retain(|_, values| values.len() >= 2);
        if optimizer_results.len() < 2 {
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
            return Ok(section);
        }
        
        let mut csv_data = Vec::new();
        csv_data.push("Problem,QQN_Optimizer,NonQQN_Optimizer,Metric,Winner,Test_Statistic,P_Value,Significant,Effect_Size".to_string());

        // Track wins for summary matrix
        let mut win_matrix: HashMap<(String, String), (i32, i32)> = HashMap::new(); // (qqn_wins, non_qqn_wins)

        // Group results by problem name (can be changed to family if needed)
        let mut grouped_optimizer_results: HashMap<String, HashMap<String, Vec<(f64, f64)>>> =
            HashMap::new();
        for (optimizer, results) in &optimizer_results {
            for (final_value, cost, problem) in results {
                grouped_optimizer_results
                    //.entry(get_family(problem))
                    .entry(problem.to_string())
                    .or_insert_with(HashMap::new)
                    .entry(optimizer.clone())
                    .or_insert_with(Vec::new)
                    .push((*final_value, *cost));
            }
        }
        for (family_name, family_results) in &grouped_optimizer_results {
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
                    let qqn_final_values: Vec<f64> = qqn_results
                        .iter()
                        .map(|(final_val, _)| *final_val)
                        .collect();
                    let non_qqn_final_values: Vec<f64> = non_qqn_results
                        .iter()
                        .map(|(final_val, _)| *final_val)
                        .collect();

                    match self.welch_t_test(&qqn_final_values, &non_qqn_final_values) {
                        Ok((t_stat, p_value)) => {
                            let effect_size =
                                self.cohens_d(&qqn_final_values, &non_qqn_final_values);
                            let significant = p_value < 0.05;

                            // Determine winner (lower is better for minimization)
                            let qqn_mean: f64 = qqn_final_values.iter().sum::<f64>()
                                / qqn_final_values.len() as f64;
                            let non_qqn_mean: f64 = non_qqn_final_values.iter().sum::<f64>()
                                / non_qqn_final_values.len() as f64;
                            let _winner = if significant {
                                if qqn_mean < non_qqn_mean {
                                    win_matrix
                                        .entry((qqn_opt.clone(), non_qqn_opt.clone()))
                                        .or_insert((0, 0))
                                        .0 += 1;
                                    format!("<span style='color: #28a745; font-weight: bold;'>{}</span>", qqn_opt)
                                } else {
                                    win_matrix
                                        .entry((qqn_opt.clone(), non_qqn_opt.clone()))
                                        .or_insert((0, 0))
                                        .1 += 1;
                                    format!("<span style='color: #dc3545; font-weight: bold;'>{}</span>", non_qqn_opt)
                                }
                            } else {
                                "—".to_string()
                            };

                            let winner_name = if significant {
                                if qqn_mean < non_qqn_mean {
                                    qqn_opt
                                } else {
                                    non_qqn_opt
                                }
                            } else {
                                "-"
                            };
                            csv_data.push(format!(
                                "{},{},{},Final_Objective_Value,{},{:.6},{:.6},{},{:.6}",
                                family_name,
                                qqn_opt,
                                non_qqn_opt,
                                winner_name,
                                t_stat,
                                p_value,
                                significant,
                                effect_size
                            ));
                        }
                        Err(e) => {
                            section.push_str(&format!(
                                r#"                <tr>
                                <td>{}</td>
                                <td>{}</td>
                                <td>{}</td>
                                <td colspan="7"><em>Test failed: {}</em></td>
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

                            // Determine winner (lower is better for cost)
                            let qqn_mean_cost: f64 =
                                costs_qqn.iter().sum::<f64>() / costs_qqn.len() as f64;
                            let non_qqn_mean_cost: f64 =
                                costs_non_qqn.iter().sum::<f64>() / costs_non_qqn.len() as f64;
                            let winner_name = if significant {
                                if qqn_mean_cost < non_qqn_mean_cost {
                                    qqn_opt
                                } else {
                                    non_qqn_opt
                                }
                            } else {
                                "-"
                            };
                            csv_data.push(format!(
                                "{},{},{},Computational_Cost,{},{:.6},{:.6},{},{:.6}",
                                family_name,
                                qqn_opt,
                                non_qqn_opt,
                                winner_name,
                                t_stat,
                                p_value,
                                significant,
                                effect_size
                            ));
                        }
                        Err(_e) => {
                        }
                    }
                }
            }
        }

        if !win_matrix.is_empty() {
            section.push_str(&self.generate_comparison_matrix(&grouped_optimizer_results)?);
        }

        // Save CSV data
        if let Err(e) = self.save_statistical_analysis_csv(&csv_data, output_dir) {
            eprintln!("Warning: Failed to save statistical analysis CSV: {}", e);
        }

        section.push_str(r#"
<style>
    .significant-row { background-color: #f0f8ff; }
    .significant-row td { font-weight: 500; }
</style>
        "#);

        Ok(section)
    }
    /// Expose welch_t_test for use in LaTeX table generation
    pub fn welch_t_test_public(&self, sample_a: &[f64], sample_b: &[f64]) -> anyhow::Result<(f64, f64)> {
        self.welch_t_test(sample_a, sample_b)
    }

    fn save_statistical_analysis_csv(
        &self,
        csv_data: &[String],
        output_dir: &str,
    ) -> anyhow::Result<()> {
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

        let var_a = sample_a.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>()
            / (sample_a.len() - 1) as f64;
        let var_b = sample_b.iter().map(|x| (x - mean_b).powi(2)).sum::<f64>()
            / (sample_b.len() - 1) as f64;

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
            let critical_005 = if df >= 10.0 {
                2.228
            } else if df >= 5.0 {
                2.571
            } else {
                3.182
            };
            let critical_001 = if df >= 10.0 {
                3.169
            } else if df >= 5.0 {
                4.032
            } else {
                5.841
            };
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

        let var_a = sample_a.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>()
            / (sample_a.len() - 1) as f64;
        let var_b = sample_b.iter().map(|x| (x - mean_b).powi(2)).sum::<f64>()
            / (sample_b.len() - 1) as f64;

        let pooled_sd = ((var_a + var_b) / 2.0).sqrt();
        if pooled_sd == 0.0 {
            return 0.0;
        }

        (mean_a - mean_b) / pooled_sd
    }
    fn generate_comparison_matrix(
        &self,
        grouped_results: &HashMap<String, HashMap<String, Vec<(f64, f64)>>>,
    ) -> anyhow::Result<String> {
        let mut matrix_section = String::from(
            r#"

# QQN vs Non-QQN Comparison Matrix

Matrix showing all comparisons. Green indicates QQN variant won (statistically significant), red indicates non-QQN optimizer won (statistically significant), gray indicates no significant difference.

"#,
        );
        // Collect all QQN and non-QQN optimizers
        let mut all_qqn_optimizers = std::collections::HashSet::new();
        let mut all_non_qqn_optimizers = std::collections::HashSet::new();
        for family_results in grouped_results.values() {
            for optimizer in family_results.keys() {
                if optimizer.contains("QQN") {
                    all_qqn_optimizers.insert(optimizer.clone());
                } else {
                    all_non_qqn_optimizers.insert(optimizer.clone());
                }
            }
        }
        let mut qqn_optimizers: Vec<_> = all_qqn_optimizers.into_iter().collect();
        let mut non_qqn_optimizers: Vec<_> = all_non_qqn_optimizers.into_iter().collect();
        qqn_optimizers.sort();
        non_qqn_optimizers.sort();
        if qqn_optimizers.is_empty() || non_qqn_optimizers.is_empty() {
            return Ok("".to_string());
        }
        
        // Generate matrix table
        matrix_section.push_str(
            r#"
<table class="comparison-matrix">
  <tr>
"#,
        );
        // Header row with non-QQN optimizers
        for non_qqn in &non_qqn_optimizers {
            matrix_section.push_str(&format!("    <th>{}</th>\n", non_qqn));
        }
        matrix_section.push_str("  </tr>\n");
        
        // Data rows
        for qqn_opt in &qqn_optimizers {
            matrix_section.push_str(&format!(
                "  <tr>\n    <th>{}</th>\n",
                qqn_opt
            ));
            for non_qqn_opt in &non_qqn_optimizers {
                matrix_section.push_str("    <td>");
                
                let mut cell_content = Vec::new();
                let mut problem_names: Vec<_> = grouped_results.keys().cloned().collect();
                problem_names.sort();
                
                for problem_name in &problem_names {
                    if let Some(family_results) = grouped_results.get(problem_name) {
                        if let (Some(qqn_results), Some(non_qqn_results)) =
                            (family_results.get(qqn_opt), family_results.get(non_qqn_opt))
                        {
                            if qqn_results.len() >= 2 && non_qqn_results.len() >= 2 {
                                let mut problem_section = format!("<div class='problem-comparison'><strong>{}</strong>", problem_name);
                                
                                // Test final values
                                let qqn_final_values: Vec<f64> =
                                    qqn_results.iter().map(|(v, _)| *v).collect();
                                let non_qqn_final_values: Vec<f64> =
                                    non_qqn_results.iter().map(|(v, _)| *v).collect();
                                
                                let qqn_mean_obj = qqn_final_values.iter().sum::<f64>()
                                    / qqn_final_values.len() as f64;
                                let non_qqn_mean_obj = non_qqn_final_values.iter().sum::<f64>()
                                    / non_qqn_final_values.len() as f64;
                                let delta_obj = qqn_mean_obj - non_qqn_mean_obj;
                                
                                let (obj_color, _obj_significant) = if let Ok((_, p_value)) =
                                    self.welch_t_test(&qqn_final_values, &non_qqn_final_values)
                                {
                                    if p_value < 0.05 {
                                        let qqn_won = qqn_mean_obj < non_qqn_mean_obj;
                                        (if qqn_won { "#28a745" } else { "#dc3545" }, true)
                                    } else {
                                        ("#6c757d", false)
                                    }
                                } else {
                                    ("#6c757d", false)
                                };
                                
                                let delta_obj_str = if delta_obj.abs() >= 1000.0 {
                                    format!("{:.1e}", delta_obj)
                                } else if delta_obj.abs() >= 1.0 {
                                    format!("{:.2}", delta_obj)
                                } else {
                                    format!("{:.3}", delta_obj)
                                };
                                
                                problem_section.push_str(&format!(
                                    "<br><span style='color: {}; font-size: 0.85em;'>obj: Δ{}</span>",
                                    obj_color, delta_obj_str
                                ));
                                
                                // Test computational cost
                                let qqn_costs: Vec<f64> = qqn_results.iter().map(|(_, c)| *c).collect();
                                let non_qqn_costs: Vec<f64> =
                                    non_qqn_results.iter().map(|(_, c)| *c).collect();
                                
                                let qqn_mean_cost =
                                    qqn_costs.iter().sum::<f64>() / qqn_costs.len() as f64;
                                let non_qqn_mean_cost = non_qqn_costs.iter().sum::<f64>()
                                    / non_qqn_costs.len() as f64;
                                let delta_cost = qqn_mean_cost - non_qqn_mean_cost;
                                
                                let (cost_color, _cost_significant) = if let Ok((_, p_value)) = 
                                    self.welch_t_test(&qqn_costs, &non_qqn_costs)
                                {
                                    if p_value < 0.05 {
                                        let qqn_won = qqn_mean_cost < non_qqn_mean_cost;
                                        (if qqn_won { "#28a745" } else { "#dc3545" }, true)
                                    } else {
                                        ("#6c757d", false)
                                    }
                                } else {
                                    ("#6c757d", false)
                                };
                                
                                let delta_cost_str = if delta_cost.abs() >= 1000.0 {
                                    format!("{:.1e}", delta_cost)
                                } else if delta_cost.abs() >= 1.0 {
                                    format!("{:.2}", delta_cost)
                                } else {
                                    format!("{:.3}", delta_cost)
                                };
                                
                                problem_section.push_str(&format!(
                                    "<br><span style='color: {}; font-size: 0.85em;'>cost: Δ{}</span>",
                                    cost_color, delta_cost_str
                                ));
                                
                                problem_section.push_str("</div>");
                                cell_content.push(problem_section);
                            }
                        }
                    }
                }
                if !cell_content.is_empty() {
                    matrix_section.push_str(&cell_content.join(""));
                } else {
                    matrix_section.push_str("—");
                }
                matrix_section.push_str("</td>\n");
            }
            matrix_section.push_str("                </tr>\n");
        }
        matrix_section.push_str(r#"            </table>
"#);
        matrix_section.push_str(r#"
        
<p style="margin-top: 10px; font-size: 0.9em;">
    <strong>Legend:</strong> Each cell shows all problem comparisons.
    <span style='color: #28a745;'>Green</span> = QQN variant won,
    <span style='color: #dc3545;'>Red</span> = Non-QQN optimizer won.
    <span style='color: #6c757d;'>Gray</span> = No statistically significant difference.
    (obj) = objective value comparison, (cost) = computational cost comparison.
    Δ shows the signed difference (QQN mean - Non-QQN mean): negative values favor QQN for minimization problems.
</p>
            
"#);
        Ok(matrix_section)
    }
}