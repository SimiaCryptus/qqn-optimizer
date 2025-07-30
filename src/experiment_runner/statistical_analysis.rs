#![allow(clippy::type_complexity)]

use super::experiment_runner::get_optimizer_family;
use crate::benchmarks::evaluation::{BenchmarkConfig, BenchmarkResults, ProblemSpec};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
/// Minimum sample size required for statistical tests
const MIN_SAMPLE_SIZE: usize = 2;
/// Significance level for hypothesis testing
const ALPHA: f64 = 0.05;
/// Effect size thresholds for Cohen's d
const COHEN_D_SMALL: f64 = 0.2;
const COHEN_D_MEDIUM: f64 = 0.5;
const COHEN_D_LARGE: f64 = 0.8;

/// Statistical Analysis Module
///
/// This module provides comprehensive statistical analysis capabilities for comparing
/// optimization algorithms, particularly focusing on QQN (Quasi-Quasi-Newton) methods
/// versus traditional optimization approaches.
///
/// # Features
///
/// - **Welch's t-test**: Performs statistical significance testing between optimizer groups
/// - **Effect size calculation**: Computes Cohen's d for practical significance assessment
/// - **Comparison matrices**: Generates visual comparison tables for multiple optimizers
/// - **Dual metric analysis**: Evaluates both objective function values and computational costs
/// - **CSV export**: Saves raw statistical data for further analysis
///
/// # Statistical Methods
///
/// The module employs robust statistical techniques suitable for optimization benchmarking:
///
/// - **Welch's t-test**: Used instead of Student's t-test as it doesn't assume equal variances
/// - **Two-tailed testing**: Tests for significant differences in either direction
/// - **Bonferroni correction**: Should be applied externally for multiple comparisons
/// - **Effect size**: Cohen's d provides practical significance beyond statistical significance
///
/// # Usage Patterns
///
/// ```rust
/// use qqn_optimizer::experiment_runner::StatisticalAnalysis;
///
/// let analysis = StatisticalAnalysis::new();
/// let report = analysis.generate_statistical_analysis(
///     &benchmark_results,
///     &config,
///     "output/",
///     true  // use optimizer families
/// )?;
/// ```
#[derive(Debug, Clone)]
pub struct StatisticalAnalysis;

impl Default for StatisticalAnalysis {
    fn default() -> Self {
        Self::new()
    }
}

impl StatisticalAnalysis {
    /// Creates a new StatisticalAnalysis instance
    ///
    /// # Returns
    ///
    /// A new `StatisticalAnalysis` struct ready for use
    pub fn new() -> Self {
        Self
    }
    /// Generates comprehensive statistical analysis comparing optimization algorithms
    ///
    /// This is the main entry point for statistical analysis. It performs pairwise
    /// comparisons between QQN and non-QQN optimizers across all benchmark problems,
    /// testing both objective function performance and computational efficiency.
    ///
    /// # Arguments
    ///
    /// * `all_results` - Slice of tuples containing problem specifications and their benchmark results
    /// * `_config` - Benchmark configuration (currently unused but reserved for future features)
    /// * `output_dir` - Directory path where CSV files and other outputs will be saved
    /// * `use_optimizer_families` - If true, groups similar optimizers (e.g., all BFGS variants)
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - HTML-formatted statistical analysis report
    /// * `Err(anyhow::Error)` - If file I/O fails or insufficient data is provided
    ///
    /// # Statistical Analysis Process
    ///
    /// 1. **Data Validation**: Ensures sufficient sample sizes (≥2 per group)
    /// 2. **Optimizer Categorization**: Separates QQN from non-QQN methods
    /// 3. **Problem Grouping**: Groups results by problem or problem family
    /// 4. **Pairwise Testing**: Performs Welch's t-tests for each QQN vs non-QQN pair
    /// 5. **Effect Size Calculation**: Computes Cohen's d for practical significance
    /// 6. **Report Generation**: Creates HTML comparison matrices and CSV exports
    ///
    /// # Output Files
    ///
    /// - `statistical_analysis_raw_data.csv`: Raw statistical test results
    ///
    /// # Example Output Interpretation
    ///
    /// ```text
    /// Problem: Rosenbrock, QQN vs BFGS
    /// - Final Objective: QQN wins (p=0.023, d=0.85)
    /// - Computational Cost: BFGS wins (p=0.041, d=0.72)
    /// ```
    ///
    /// This indicates QQN found better solutions but required more function evaluations.
    pub fn generate_statistical_analysis(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        _config: &BenchmarkConfig,
        output_dir: &str,
        use_optimizer_families: bool,
    ) -> Result<String> {
        let mut section = String::new();

        // Data validation: Ensure sufficient results for meaningful analysis
        let total_results: usize = all_results.iter().map(|(_, r)| r.results.len()).sum();
        if total_results < MIN_SAMPLE_SIZE {
            section.push_str(
                "Insufficient data for statistical analysis (minimum 2 results required).\n",
            );
            return Ok(section);
        }

        // Organize results by optimizer, storing (final_value, cost, problem_name)
        // Cost is defined as max(function_evaluations, gradient_evaluations)

        let mut optimizer_results: HashMap<String, Vec<(f64, f64, String)>> = HashMap::new(); // (final_value, cost, problem)

        for (problem, results) in all_results {
            let problem_name = problem.get_name();
            for result in &results.results {
                let cost = result.function_evaluations.max(result.gradient_evaluations) as f64;
                let optimizer_key = if use_optimizer_families {
                    get_optimizer_family(&result.optimizer_name)
                } else {
                    result.optimizer_name.clone()
                };

                optimizer_results.entry(optimizer_key).or_default().push((
                    result.final_value,
                    cost,
                    problem_name.to_string(),
                ));
            }
        }

        // Remove optimizers with insufficient data (< 2 samples)

        optimizer_results.retain(|_, values| values.len() >= MIN_SAMPLE_SIZE);

        if optimizer_results.len() < 2 {
            section.push_str("Insufficient optimizer variety for comparison (minimum 2 different optimizers required).\n");
            return Ok(section);
        }

        // Categorize optimizers into QQN and non-QQN groups
        // QQN optimizers are identified by containing "QQN" in their name

        let mut qqn_optimizers = Vec::new();
        let mut non_qqn_optimizers = Vec::new();

        for optimizer_name in optimizer_results.keys() {
            if optimizer_name == "QQN" || optimizer_name.contains("QQN") {
                qqn_optimizers.push(optimizer_name.clone());
            } else {
                non_qqn_optimizers.push(optimizer_name.clone());
            }
        }

        // Require at least one optimizer from each category for comparison

        if qqn_optimizers.is_empty() || non_qqn_optimizers.is_empty() {
            section.push_str("Statistical comparison requires both QQN and non-QQN optimizers.\n");
            return Ok(section);
        }

        // Initialize CSV output with headers
        // CSV contains detailed statistical test results for external analysis

        let mut csv_data = Vec::new();
        csv_data.push("Problem,QQN_Optimizer,NonQQN_Optimizer,Metric,Winner,Test_Statistic,P_Value,Significant,Effect_Size".to_string());

        // Track wins for summary matrix generation
        // Key: (qqn_optimizer, non_qqn_optimizer), Value: (qqn_wins, non_qqn_wins)
        let mut win_matrix: HashMap<(String, String), (i32, i32)> = HashMap::new(); // (qqn_wins, non_qqn_wins)

        // Group results by problem for focused comparisons
        // This allows testing optimizer performance on specific problem types
        let mut grouped_optimizer_results: HashMap<String, HashMap<String, Vec<(f64, f64)>>> =
            HashMap::new();
        for (optimizer, results) in &optimizer_results {
            for (final_value, cost, problem) in results {
                let optimizer_key = if use_optimizer_families {
                    get_optimizer_family(optimizer)
                } else {
                    optimizer.clone()
                };
                grouped_optimizer_results
                    // Note: Could group by problem family instead of individual problems
                    .entry(problem.to_string())
                    .or_default()
                    .entry(optimizer_key)
                    .or_default()
                    .push((*final_value, *cost));
            }
        }
        // Perform statistical analysis for each problem family
        for (family_name, family_results) in &grouped_optimizer_results {
            // Re-categorize optimizers within this problem family
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
            // Perform all pairwise comparisons between QQN and non-QQN optimizers

            for qqn_opt in &family_qqn_optimizers {
                for non_qqn_opt in &family_non_qqn_optimizers {
                    let qqn_results = &family_results[qqn_opt];
                    let non_qqn_results = &family_results[non_qqn_opt];
                    // Skip comparisons with insufficient sample sizes

                    if qqn_results.len() < 2 || non_qqn_results.len() < 2 {
                        continue;
                    }

                    // Test 1: Compare final objective function values
                    // Lower values indicate better performance for minimization problems
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
                            let significant = p_value < ALPHA;

                            // Determine winner: lower mean indicates better performance
                            let qqn_mean: f64 = qqn_final_values.iter().sum::<f64>()
                                / qqn_final_values.len() as f64;
                            let non_qqn_mean: f64 = non_qqn_final_values.iter().sum::<f64>()
                                / non_qqn_final_values.len() as f64;

                            let winner = if significant {
                                if qqn_mean < non_qqn_mean {
                                    win_matrix
                                        .entry((qqn_opt.clone(), non_qqn_opt.clone()))
                                        .or_insert((0, 0))
                                        .0 += 1;
                                    qqn_opt.clone()
                                } else {
                                    win_matrix
                                        .entry((qqn_opt.clone(), non_qqn_opt.clone()))
                                        .or_insert((0, 0))
                                        .1 += 1;
                                    non_qqn_opt.clone()
                                }
                            } else {
                                "-".to_string()
                            };

                            csv_data.push(format!(
                                "{family_name},{qqn_opt},{non_qqn_opt},Final_Objective_Value,{winner},{t_stat:.6},{p_value:.6},{significant},{effect_size:.3}"
                            ));
                        }
                        Err(e) => {
                            log::warn!(
                                "Statistical test failed for {qqn_opt} vs {non_qqn_opt} on {family_name}: {e}"
                            );
                        }
                    }

                    // Test 2: Compare computational costs
                    // Lower costs indicate better efficiency
                    let costs_qqn: Vec<f64> = qqn_results.iter().map(|(_, c)| *c).collect();
                    let costs_non_qqn: Vec<f64> = non_qqn_results.iter().map(|(_, c)| *c).collect();
                    match self.welch_t_test(&costs_qqn, &costs_non_qqn) {
                        Ok((t_stat, p_value)) => {
                            let effect_size = self.cohens_d(&costs_qqn, &costs_non_qqn);
                            let significant = p_value < ALPHA;

                            // Determine efficiency winner: lower mean cost is better
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
                                "{family_name},{qqn_opt},{non_qqn_opt},Computational_Cost,{winner_name},{t_stat:.6},{p_value:.6},{significant},{effect_size:.3}"
                            ));
                        }
                        Err(e) => {
                            log::warn!(
                                "Cost comparison test failed for {qqn_opt} vs {non_qqn_opt} on {family_name}: {e}"
                            );
                        }
                    }
                }
            }
        }

        // Generate visual comparison matrix if we have comparison data

        if !win_matrix.is_empty() {
            section.push_str(
                &self.generate_comparison_matrix(
                    &grouped_optimizer_results,
                    use_optimizer_families,
                )?,
            );
        }

        // Export raw statistical data for external analysis tools
        self.save_statistical_analysis_csv(&csv_data, output_dir)
            .context("Failed to save statistical analysis CSV")?;

        // Generate LaTeX tables for academic reporting
        self.generate_pairwise_statistical_matrix(&optimizer_results, output_dir)
            .context("Failed to generate pairwise statistical matrix")?;
        self.generate_problem_difficulty_ranking(all_results, output_dir)
            .context("Failed to generate problem difficulty ranking")?;

        Ok(section)
    }

    /// Public interface for Welch's t-test
    ///
    /// Exposes the internal t-test implementation for use by other modules,
    /// particularly for LaTeX table generation and custom analysis workflows.
    ///
    /// # Arguments
    ///
    /// * `sample_a` - First sample data
    /// * `sample_b` - Second sample data
    ///
    /// # Returns
    ///
    /// * `Ok((t_statistic, p_value))` - Test results
    /// * `Err(anyhow::Error)` - If samples are too small or have computational issues
    pub fn welch_t_test_public(&self, sample_a: &[f64], sample_b: &[f64]) -> Result<(f64, f64)> {
        self.welch_t_test(sample_a, sample_b)
    }
    /// Public interface for Cohen's d effect size calculation
    ///
    /// # Arguments
    ///
    /// * `sample_a` - First sample data
    /// * `sample_b` - Second sample data
    ///
    /// # Returns
    ///
    /// Cohen's d effect size (always positive)
    pub fn cohens_d_public(&self, sample_a: &[f64], sample_b: &[f64]) -> f64 {
        self.cohens_d(sample_a, sample_b)
    }
    /// Saves statistical analysis results to CSV file
    ///
    /// Creates a CSV file containing all statistical test results for external
    /// analysis, visualization, or reporting. The CSV includes test statistics,
    /// p-values, effect sizes, and significance indicators.
    ///
    /// # Arguments
    ///
    /// * `csv_data` - Vector of CSV-formatted strings (including header)
    /// * `output_dir` - Directory where the CSV file will be saved
    ///
    /// # Output File
    ///
    /// Creates `statistical_analysis_raw_data.csv` with columns:
    /// - Problem: Problem or problem family name
    /// - QQN_Optimizer: Name of the QQN optimizer variant
    /// - NonQQN_Optimizer: Name of the comparison optimizer
    /// - Metric: Either "Final_Objective_Value" or "Computational_Cost"
    /// - Winner: Which optimizer performed better (or "-" if not significant)
    /// - Test_Statistic: Welch's t-test statistic
    /// - P_Value: Statistical significance level
    /// - Significant: Boolean indicating p < 0.05
    /// - Effect_Size: Cohen's d effect size measure
    fn save_statistical_analysis_csv(&self, csv_data: &[String], output_dir: &str) -> Result<()> {
        let csv_content = csv_data.join("\n");
        let csv_path = Path::new(output_dir).join("statistical_analysis_raw_data.csv");
        fs::write(&csv_path, csv_content)
            .with_context(|| format!("Failed to write CSV to {csv_path:?}"))?;
        Ok(())
    }
    /// Performs Welch's t-test for comparing two independent samples
    ///
    /// Welch's t-test is preferred over Student's t-test because it doesn't assume
    /// equal variances between groups, making it more robust for optimization
    /// benchmarking where different algorithms may have very different variance
    /// characteristics.
    ///
    /// # Mathematical Background
    ///
    /// The test statistic is calculated as:
    /// ```text
    /// t = (mean_a - mean_b) / sqrt(var_a/n_a + var_b/n_b)
    /// ```
    ///
    /// Degrees of freedom use the Welch-Satterthwaite equation:
    /// ```text
    /// df = (var_a/n_a + var_b/n_b)² / ((var_a/n_a)²/(n_a-1) + (var_b/n_b)²/(n_b-1))
    /// ```
    ///
    /// # Arguments
    ///
    /// * `sample_a` - First sample (e.g., QQN optimizer results)
    /// * `sample_b` - Second sample (e.g., BFGS optimizer results)
    ///
    /// # Returns
    ///
    /// * `Ok((t_statistic, p_value))` - Test results for two-tailed test
    /// * `Err(anyhow::Error)` - If insufficient data or computational issues
    ///
    /// # Special Cases
    ///
    /// - Zero variance in both samples with equal means: returns (0.0, 1.0)
    /// - Zero variance with different means: returns error
    /// - Zero standard error: returns error
    ///
    /// # Example
    ///
    /// ```rust
    /// let qqn_results = vec![1.2, 1.5, 1.1, 1.3];
    /// let bfgs_results = vec![2.1, 2.3, 1.9, 2.2];
    /// let (t_stat, p_val) = analysis.welch_t_test(&qqn_results, &bfgs_results)?;
    /// println!("t = {:.3}, p = {:.3}", t_stat, p_val);
    /// ```
    fn welch_t_test(&self, sample_a: &[f64], sample_b: &[f64]) -> Result<(f64, f64)> {
        if sample_a.len() < MIN_SAMPLE_SIZE || sample_b.len() < MIN_SAMPLE_SIZE {
            return Err(anyhow::anyhow!("Insufficient sample size for t-test"));
        }

        let mean_a = sample_a.iter().sum::<f64>() / sample_a.len() as f64;
        let mean_b = sample_b.iter().sum::<f64>() / sample_b.len() as f64;

        let var_a = sample_a.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>()
            / (sample_a.len() - 1) as f64;
        let var_b = sample_b.iter().map(|x| (x - mean_b).powi(2)).sum::<f64>()
            / (sample_b.len() - 1) as f64;

        // Handle special case: both samples have zero variance

        if var_a == 0.0 && var_b == 0.0 {
            if mean_a == mean_b {
                return Ok((0.0, 1.0));
            } else {
                return Err(anyhow::anyhow!("Zero variance with different means"));
            }
        }

        // Calculate standard error of the difference in means

        let se = (var_a / sample_a.len() as f64 + var_b / sample_b.len() as f64).sqrt();
        if se == 0.0 {
            return Err(anyhow::anyhow!("Zero standard error"));
        }

        // Compute t-statistic

        let t_stat = (mean_a - mean_b) / se;

        // Calculate degrees of freedom using Welch-Satterthwaite equation

        let df = {
            let numerator = (var_a / sample_a.len() as f64 + var_b / sample_b.len() as f64).powi(2);
            let denom_a = (var_a / sample_a.len() as f64).powi(2) / (sample_a.len() - 1) as f64;
            let denom_b = (var_b / sample_b.len() as f64).powi(2) / (sample_b.len() - 1) as f64;
            numerator / (denom_a + denom_b)
        };

        // Convert t-statistic to p-value for two-tailed test

        let p_value = self.t_distribution_p_value(t_stat.abs(), df);
        Ok((t_stat, p_value))
    }
    /// Approximates p-value for t-distribution
    ///
    /// Provides approximate p-values for two-tailed t-tests using lookup tables
    /// and interpolation. This implementation prioritizes speed over precision,
    /// which is acceptable for benchmark analysis where exact p-values are less
    /// critical than identifying clear statistical differences.
    ///
    /// # Arguments
    ///
    /// * `t_abs` - Absolute value of the t-statistic
    /// * `df` - Degrees of freedom
    ///
    /// # Returns
    ///
    /// Approximate two-tailed p-value
    ///
    /// # Approximation Method
    ///
    /// - For df > 30: Uses normal approximation
    /// - For df ≤ 30: Uses t-distribution critical values
    /// - Returns conservative estimates at standard significance levels
    ///
    /// # Accuracy
    ///
    /// The approximation is most accurate for common significance levels:
    /// - p < 0.001, p < 0.01, p < 0.05, p < 0.10
    /// - Less precise for intermediate p-values, but sufficient for hypothesis testing
    fn t_distribution_p_value(&self, t_abs: f64, df: f64) -> f64 {
        // Special case: if t = 0, p-value should be 1.0
        if t_abs == 0.0 {
            return 1.0;
        }

        // For large df, t-distribution approaches normal distribution
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
            // Use t-distribution critical values for smaller sample sizes
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
    /// Calculates Cohen's d effect size
    ///
    /// Cohen's d measures the standardized difference between two means,
    /// providing an indication of practical significance beyond statistical
    /// significance. This is crucial in optimization benchmarking where
    /// small but statistically significant differences may not be practically
    /// meaningful.
    ///
    /// # Formula
    ///
    /// ```text
    /// d = |mean_a - mean_b| / pooled_standard_deviation
    /// ```
    ///
    /// Where pooled SD = sqrt((var_a + var_b) / 2)
    ///
    /// # Interpretation Guidelines
    ///
    /// - **Small effect**: d ≈ 0.2
    /// - **Medium effect**: d ≈ 0.5  
    /// - **Large effect**: d ≈ 0.8
    /// - **Very large effect**: d > 1.0
    ///
    /// # Arguments
    ///
    /// * `sample_a` - First sample data
    /// * `sample_b` - Second sample data
    ///
    /// # Returns
    ///
    /// Cohen's d effect size (always positive)
    ///
    /// # Example Interpretation
    ///
    /// ```text
    /// d = 0.3: QQN is 0.3 standard deviations better (small effect)
    /// d = 0.8: QQN is 0.8 standard deviations better (large effect)
    /// d = 1.2: QQN is 1.2 standard deviations better (very large effect)
    /// ```
    fn cohens_d(&self, sample_a: &[f64], sample_b: &[f64]) -> f64 {
        if sample_a.len() < MIN_SAMPLE_SIZE || sample_b.len() < MIN_SAMPLE_SIZE {
            return 0.0;
        }

        let mean_a = sample_a.iter().sum::<f64>() / sample_a.len() as f64;
        let mean_b = sample_b.iter().sum::<f64>() / sample_b.len() as f64;

        let var_a = sample_a.iter().map(|x| (x - mean_a).powi(2)).sum::<f64>()
            / (sample_a.len() - 1) as f64;
        let var_b = sample_b.iter().map(|x| (x - mean_b).powi(2)).sum::<f64>()
            / (sample_b.len() - 1) as f64;

        // Calculate pooled standard deviation

        let pooled_sd = ((var_a + var_b) / 2.0).sqrt();
        if pooled_sd == 0.0 {
            return 0.0;
        }

        // Return absolute effect size

        (mean_a - mean_b).abs() / pooled_sd
    }
    /// Interprets Cohen's d effect size
    ///
    /// # Arguments
    ///
    /// * `d` - Cohen's d value
    ///
    /// # Returns
    ///
    /// String description of effect size magnitude
    pub fn interpret_cohens_d(&self, d: f64) -> &'static str {
        if d < COHEN_D_SMALL {
            "negligible"
        } else if d < COHEN_D_MEDIUM {
            "small"
        } else if d < COHEN_D_LARGE {
            "medium"
        } else {
            "large"
        }
    }
    /// Generates HTML comparison matrix for optimizer performance
    ///
    /// Creates a comprehensive visual comparison showing how QQN optimizers
    /// perform against traditional optimizers across all benchmark problems.
    /// The matrix uses color coding to quickly identify performance patterns.
    ///
    /// # Arguments
    ///
    /// * `grouped_results` - Results organized by problem and optimizer
    /// * `use_optimizer_families` - Whether to group similar optimizers
    ///
    /// # Returns
    ///
    /// HTML-formatted comparison matrix string
    ///
    /// # Matrix Structure
    ///
    /// - **Rows**: QQN optimizer variants
    /// - **Columns**: Non-QQN optimizers  
    /// - **Cells**: Problem-specific comparisons
    ///
    /// # Color Coding
    ///
    /// - **Green**: QQN optimizer won (statistically significant)
    /// - **Red**: Non-QQN optimizer won (statistically significant)
    /// - **Gray**: No statistically significant difference
    ///
    /// # Cell Contents
    ///
    /// Each cell shows:
    /// - Problem name
    /// - Objective value comparison (obj: delta)
    /// - Computational cost comparison (cost: delta)
    ///
    /// Where delta = QQN_mean - NonQQN_mean (negative favors QQN)
    ///
    /// # Example Cell
    ///
    /// ```html
    /// <div class='problem-comparison'>
    ///   <strong>Rosenbrock</strong>
    ///   <br><span style='color: #28a745;'>obj: -0.023</span>
    ///   <br><span style='color: #dc3545;'>cost: 15.2</span>
    /// </div>
    /// ```
    ///
    /// This indicates QQN found better solutions (green, negative delta)
    /// but used more function evaluations (red, positive delta).
    fn generate_comparison_matrix(
        &self,
        grouped_results: &HashMap<String, HashMap<String, Vec<(f64, f64)>>>,
        use_optimizer_families: bool,
    ) -> Result<String> {
        let mut matrix_section = String::from(
            r#"

# Optimizer Comparison Matrix

Matrix showing all comparisons. Green indicates QQN won (statistically significant), red indicates non-QQN optimizer won (statistically significant), gray indicates no significant difference.

"#,
        );

        if use_optimizer_families {
            matrix_section.push_str("**Note:** Comparisons are based on optimizer families (e.g., all QQN variants grouped together).\n\n");
        }

        // Collect and categorize all optimizers present in the results
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
        // Convert to sorted vectors for consistent ordering
        let mut qqn_optimizers: Vec<_> = all_qqn_optimizers.into_iter().collect();
        let mut non_qqn_optimizers: Vec<_> = all_non_qqn_optimizers.into_iter().collect();
        qqn_optimizers.sort();
        non_qqn_optimizers.sort();

        if qqn_optimizers.is_empty() || non_qqn_optimizers.is_empty() {
            return Ok("".to_string());
        }

        // Generate HTML table structure
        matrix_section.push_str(
            r#"
<table class="comparison-matrix">
  <tr><th></th>
"#,
        );

        // Create header row with non-QQN optimizer names
        for non_qqn in &non_qqn_optimizers {
            matrix_section.push_str(&format!("    <th>{non_qqn}</th>\n"));
        }
        matrix_section.push_str("  </tr>\n");

        // Generate data rows (one per QQN optimizer)
        for qqn_opt in &qqn_optimizers {
            matrix_section.push_str(&format!("  <tr>\n    <th>{qqn_opt}</th>\n"));
            // Generate cells for each QQN vs non-QQN comparison
            for non_qqn_opt in &non_qqn_optimizers {
                matrix_section.push_str("    <td>");

                let mut cell_content = Vec::new();
                // Sort problems for consistent display order
                let mut problem_names: Vec<_> = grouped_results.keys().cloned().collect();
                problem_names.sort();
                // Process each problem for this optimizer pair

                for problem_name in &problem_names {
                    if let Some(family_results) = grouped_results.get(problem_name) {
                        if let (Some(qqn_results), Some(non_qqn_results)) =
                            (family_results.get(qqn_opt), family_results.get(non_qqn_opt))
                        {
                            if qqn_results.len() >= 2 && non_qqn_results.len() >= 2 {
                                let mut problem_section = format!(
                                    "<div class='problem-comparison'><strong>{problem_name}</strong>"
                                );

                                // Analyze objective function performance
                                let qqn_final_values: Vec<f64> =
                                    qqn_results.iter().map(|(v, _)| *v).collect();
                                let non_qqn_final_values: Vec<f64> =
                                    non_qqn_results.iter().map(|(v, _)| *v).collect();

                                let qqn_mean_obj = qqn_final_values.iter().sum::<f64>()
                                    / qqn_final_values.len() as f64;
                                let non_qqn_mean_obj = non_qqn_final_values.iter().sum::<f64>()
                                    / non_qqn_final_values.len() as f64;
                                let delta_obj = qqn_mean_obj - non_qqn_mean_obj;
                                // Determine color based on statistical significance and winner

                                let (obj_color, _obj_significant) = if let Ok((_, p_value)) =
                                    self.welch_t_test(&qqn_final_values, &non_qqn_final_values)
                                {
                                    if p_value < ALPHA {
                                        let qqn_won = qqn_mean_obj < non_qqn_mean_obj;
                                        (if qqn_won { "#28a745" } else { "#dc3545" }, true)
                                    } else {
                                        ("#6c757d", false)
                                    }
                                } else {
                                    ("#6c757d", false)
                                };

                                // Format delta value with appropriate precision

                                let delta_obj_str = if delta_obj.abs() >= 1000.0 {
                                    format!("{delta_obj:.1e}")
                                } else if delta_obj.abs() >= 1.0 {
                                    format!("{delta_obj:.2}")
                                } else {
                                    format!("{delta_obj:.3}")
                                };

                                problem_section.push_str(&format!(
                                    "<br><span style='color: {obj_color}; font-size: 0.85em;'>obj: {delta_obj_str}</span>"
                                ));

                                // Analyze computational efficiency
                                let qqn_costs: Vec<f64> =
                                    qqn_results.iter().map(|(_, c)| *c).collect();
                                let non_qqn_costs: Vec<f64> =
                                    non_qqn_results.iter().map(|(_, c)| *c).collect();

                                let qqn_mean_cost =
                                    qqn_costs.iter().sum::<f64>() / qqn_costs.len() as f64;
                                let non_qqn_mean_cost =
                                    non_qqn_costs.iter().sum::<f64>() / non_qqn_costs.len() as f64;
                                let delta_cost = qqn_mean_cost - non_qqn_mean_cost;
                                // Determine color for cost comparison

                                let (cost_color, _cost_significant) = if let Ok((_, p_value)) =
                                    self.welch_t_test(&qqn_costs, &non_qqn_costs)
                                {
                                    if p_value < ALPHA {
                                        let qqn_won = qqn_mean_cost < non_qqn_mean_cost;
                                        (if qqn_won { "#28a745" } else { "#dc3545" }, true)
                                    } else {
                                        ("#6c757d", false)
                                    }
                                } else {
                                    ("#6c757d", false)
                                };

                                // Format cost delta with appropriate precision

                                let delta_cost_str = if delta_cost.abs() >= 1000.0 {
                                    format!("{delta_cost:.1e}")
                                } else if delta_cost.abs() >= 1.0 {
                                    format!("{delta_cost:.2}")
                                } else {
                                    format!("{delta_cost:.3}")
                                };

                                problem_section.push_str(&format!(
                                    "<br><span style='color: {cost_color}; font-size: 0.85em;'>cost: {delta_cost_str}</span>"
                                ));

                                problem_section.push_str("</div>");
                                cell_content.push(problem_section);
                            }
                        }
                    }
                }
                // Fill cell with content or placeholder
                if !cell_content.is_empty() {
                    matrix_section.push_str(&cell_content.join(""));
                } else {
                    matrix_section.push('—');
                }
                matrix_section.push_str("</td>\n");
            }
            matrix_section.push_str("                </tr>\n");
        }
        matrix_section.push_str(
            r#"            </table>
"#,
        );
        // Add explanatory legend
        matrix_section.push_str(r#"
        
<p style="margin-top: 10px; font-size: 0.9em;">
    <strong>Legend:</strong> Each cell shows all problem comparisons.
    <span style='color: #28a745;'>Green</span> = QQN variant won,
    <span style='color: #dc3545;'>Red</span> = Non-QQN optimizer won.
    <span style='color: #6c757d;'>Gray</span> = No statistically significant difference.
    (obj) = objective value comparison, (cost) = computational cost comparison.
    deltas show the signed difference (QQN mean - Non-QQN mean): negative values favor QQN for minimization problems.
</p>
            
"#);
        Ok(matrix_section)
    }
    /// Generates LaTeX pairwise statistical comparison matrix
    ///
    /// Creates a symmetric matrix showing statistical comparisons between all optimizer pairs.
    /// Upper triangle contains p-values from Welch's t-test, lower triangle contains Cohen's d
    /// effect sizes. Color coding indicates significance levels for easy interpretation.
    ///
    /// # Arguments
    ///
    /// * `optimizer_results` - Results organized by optimizer name
    /// * `output_dir` - Directory where LaTeX file will be saved
    /// * `use_optimizer_families` - Whether to group similar optimizers
    ///
    /// # Output File
    ///
    /// Creates `latex/statistical_pairwise_matrix.tex` with:
    /// - Symmetric matrix layout
    /// - Upper triangle: p-values (colored by significance)
    /// - Lower triangle: Cohen's d effect sizes
    /// - Diagonal: optimizer names
    ///
    /// # Color Coding
    ///
    /// - Red: p < 0.001 (highly significant)
    /// - Orange: p < 0.01 (very significant)  
    /// - Yellow: p < 0.05 (significant)
    /// - White: p ≥ 0.05 (not significant)
    fn generate_pairwise_statistical_matrix(
        &self,
        optimizer_results: &HashMap<String, Vec<(f64, f64, String)>>,
        output_dir: &str,
    ) -> Result<()> {
        // Collect optimizer names and sort for consistent ordering
        let mut optimizers: Vec<String> = optimizer_results.keys().cloned().collect();
        optimizers.sort();

        if optimizers.len() < 2 {
            return Ok(());
        }

        let latex_dir = Path::new(output_dir).join("latex");
        fs::create_dir_all(&latex_dir)
            .with_context(|| format!("Failed to create LaTeX directory: {latex_dir:?}"))?;

        let mut latex_content = String::new();

        // LaTeX table header with dynamic column count
        let col_spec = "l".repeat(optimizers.len() + 1);
        latex_content.push_str(&format!("\\begin{{tabular}}{{{col_spec}}}\n\\toprule\n"));
        // Header row
        latex_content.push_str("Optimizer");
        for opt in &optimizers {
            latex_content.push_str(&format!(" & {}", self.escape_latex(opt)));
        }
        latex_content.push_str(" \\\\\n\\midrule\n");
        // Generate matrix rows
        for (i, opt_i) in optimizers.iter().enumerate() {
            latex_content.push_str(&self.escape_latex(opt_i));
            for (j, opt_j) in optimizers.iter().enumerate() {
                if i == j {
                    // Diagonal: optimizer name (abbreviated)
                    let abbrev = if opt_i.len() > 8 {
                        format!("{}...", &opt_i[..5])
                    } else {
                        opt_i.clone()
                    };
                    latex_content
                        .push_str(&format!(" & \\textbf{{{}}}", self.escape_latex(&abbrev)));
                } else if i < j {
                    // Upper triangle: p-values
                    let p_value = self.calculate_pairwise_p_value(
                        &optimizer_results[opt_i],
                        &optimizer_results[opt_j],
                    );
                    let (color, p_str) = if p_value < 0.001 {
                        ("red!30", format!("{p_value:.3e}"))
                    } else if p_value < 0.01 {
                        ("orange!30", format!("{p_value:.3e}"))
                    } else if p_value < 0.05 {
                        ("yellow!30", format!("{p_value:.3e}"))
                    } else {
                        ("white", format!("{p_value:.3e}"))
                    };
                    latex_content.push_str(&format!(" & \\cellcolor{{{color}}}{p_str}"));
                } else {
                    // Lower triangle: Cohen's d effect sizes
                    let effect_size = self.calculate_pairwise_cohens_d(
                        &optimizer_results[opt_i],
                        &optimizer_results[opt_j],
                    );
                    latex_content.push_str(&format!(" & {effect_size:.2e}"));
                }
            }
            latex_content.push_str(" \\\\\n");
        }
        latex_content.push_str("\\bottomrule\n\\end{tabular}\n");
        // Add caption and explanation
        latex_content.push_str("\n\\vspace{0.5cm}\n");
        latex_content.push_str("\\textbf{Matrix Interpretation:}\n");
        latex_content.push_str("\\begin{itemize}\n");
        latex_content.push_str("\\item Upper triangle: p-values from Welch's t-test\n");
        latex_content.push_str("\\item Lower triangle: Cohen's d effect sizes\n");
        latex_content.push_str("\\item Color coding: \\colorbox{red!30}{p < 0.001}, \\colorbox{orange!30}{p < 0.01}, \\colorbox{yellow!30}{p < 0.05}\n");
        latex_content.push_str("\\end{itemize}\n");

        let latex_path = latex_dir.join("statistical_pairwise_matrix.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX file: {latex_path:?}"))?;

        Ok(())
    }
    /// Generates LaTeX problem difficulty ranking table
    ///
    /// Creates a comprehensive ranking of benchmark problems based on optimizer
    /// performance across all methods. Helps identify universally challenging
    /// problems and those where specific optimizers excel.
    ///
    /// # Arguments
    ///
    /// * `all_results` - All benchmark results across problems and optimizers
    /// * `output_dir` - Directory where LaTeX file will be saved
    ///
    /// # Output File
    ///
    /// Creates `latex/problem_difficulty_ranking.tex` with columns:
    /// - Problem: Problem name
    /// - Success Rate: Mean success rate across all optimizers
    /// - Best Optimizer: Optimizer with highest success rate
    /// - Worst Optimizer: Optimizer with lowest success rate  
    /// - Difficulty Score: Normalized difficulty metric (0-100)
    ///
    /// # Difficulty Calculation
    ///
    /// Difficulty score combines multiple factors:
    /// - Mean objective function value achieved
    /// - Success rate (convergence within tolerance)
    /// - Variance in performance across optimizers
    /// - Computational cost requirements
    fn generate_problem_difficulty_ranking(
        &self,
        all_results: &[(&ProblemSpec, BenchmarkResults)],
        output_dir: &str,
    ) -> Result<()> {
        let latex_dir = Path::new(output_dir).join("latex");
        fs::create_dir_all(&latex_dir)
            .with_context(|| format!("Failed to create LaTeX directory: {latex_dir:?}"))?;

        // Analyze each problem's difficulty metrics
        let mut problem_stats = Vec::new();

        for (problem_spec, results) in all_results {
            let problem_name = problem_spec.get_name();
            if results.results.is_empty() {
                continue;
            }

            // Calculate success rates by optimizer
            let mut optimizer_success: HashMap<String, (i32, i32)> = HashMap::new(); // (successes, total)
            let mut all_final_values = Vec::new();
            let mut all_costs = Vec::new();

            for result in &results.results {
                // Define success threshold based on problem type
                let success_threshold = if problem_name.contains("Rosenbrock") {
                    1e-4 // Rosenbrock is harder to solve to high precision
                } else {
                    1e-6
                };
                let success = result.final_value < success_threshold;

                let entry = optimizer_success
                    .entry(result.optimizer_name.clone())
                    .or_insert((0, 0));
                if success {
                    entry.0 += 1;
                }
                entry.1 += 1;

                all_final_values.push(result.final_value);
                all_costs.push(result.function_evaluations.max(result.gradient_evaluations) as f64);
            }
            // Calculate overall success rate
            let total_successes: i32 = optimizer_success.values().map(|(s, _)| *s).sum();
            let total_attempts: i32 = optimizer_success.values().map(|(_, t)| *t).sum();
            let success_rate = if total_attempts > 0 {
                total_successes as f64 / total_attempts as f64
            } else {
                0.0
            };
            // Find best and worst optimizers
            let mut best_optimizer = String::new();
            let mut worst_optimizer = String::new();
            let mut best_rate = -1.0;
            let mut worst_rate = 2.0;
            for (optimizer, (successes, total)) in &optimizer_success {
                let rate = *successes as f64 / *total as f64;
                if rate > best_rate {
                    best_rate = rate;
                    best_optimizer = optimizer.clone();
                }
                if rate < worst_rate {
                    worst_rate = rate;
                    worst_optimizer = optimizer.clone();
                }
            }
            // Calculate difficulty score (0-100, higher = more difficult)
            let mean_final_value =
                all_final_values.iter().sum::<f64>() / all_final_values.len() as f64;
            let mean_cost = all_costs.iter().sum::<f64>() / all_costs.len() as f64;
            let value_variance = all_final_values
                .iter()
                .map(|v| (v - mean_final_value).powi(2))
                .sum::<f64>()
                / all_final_values.len() as f64;
            // Composite difficulty score
            let difficulty_score = (
                (1.0 - success_rate) * 40.0 +  // Success rate component (0-40)
                (mean_final_value.log10().max(-10.0) + 10.0) * 3.0 +  // Objective value component (0-30)
                (mean_cost / 1000.0).min(1.0) * 20.0 +  // Cost component (0-20)
                (value_variance.sqrt() / mean_final_value.abs()).min(1.0) * 10.0
                // Variance component (0-10)
            )
            .clamp(0.0, 100.0);

            problem_stats.push((
                problem_name.to_string(),
                success_rate,
                best_optimizer,
                worst_optimizer,
                difficulty_score,
            ));
        }
        // Sort by difficulty score (descending - most difficult first)
        problem_stats.sort_by(|a, b| b.4.partial_cmp(&a.4).unwrap_or(std::cmp::Ordering::Equal));
        // Generate LaTeX table
        let mut latex_content = String::new();
        latex_content.push_str("\\begin{longtable}{lrrllr}\n");
        latex_content.push_str("\\toprule\n");
        latex_content.push_str(
            "Problem & Success Rate & Best Optimizer & Worst Optimizer & Difficulty Score \\\\\n",
        );
        latex_content.push_str("\\midrule\n");
        latex_content.push_str("\\endfirsthead\n");
        latex_content.push_str("\\toprule\n");
        latex_content.push_str(
            "Problem & Success Rate & Best Optimizer & Worst Optimizer & Difficulty Score \\\\\n",
        );
        latex_content.push_str("\\midrule\n");
        latex_content.push_str("\\endhead\n");
        for (problem, success_rate, best_opt, worst_opt, difficulty) in &problem_stats {
            // Color code difficulty score
            let difficulty_color = if *difficulty > 80.0 {
                "red!30"
            } else if *difficulty > 60.0 {
                "orange!30"
            } else if *difficulty > 40.0 {
                "yellow!30"
            } else {
                "green!30"
            };
            latex_content.push_str(&format!(
                "{} & {:.1e} & {} & {} & \\cellcolor{{{}}} {:.1} \\\\\n",
                self.escape_latex(problem),
                success_rate,
                self.escape_latex(best_opt),
                self.escape_latex(worst_opt),
                difficulty_color,
                difficulty
            ));
        }
        latex_content.push_str("\\bottomrule\n");
        latex_content.push_str("\\end{longtable}\n");
        // Add explanation
        latex_content.push_str("\n\\vspace{0.5cm}\n");
        latex_content.push_str("\\textbf{Difficulty Score Interpretation:}\n");
        latex_content.push_str("\\begin{itemize}\n");
        latex_content.push_str("\\item \\colorbox{green!30}{0-40}: Easy problems\n");
        latex_content.push_str("\\item \\colorbox{yellow!30}{40-60}: Moderate problems\n");
        latex_content.push_str("\\item \\colorbox{orange!30}{60-80}: Hard problems\n");
        latex_content.push_str("\\item \\colorbox{red!30}{80-100}: Very hard problems\n");
        latex_content.push_str("\\end{itemize}\n");

        let latex_path = latex_dir.join("problem_difficulty_ranking.tex");
        fs::write(&latex_path, latex_content)
            .with_context(|| format!("Failed to write LaTeX file: {latex_path:?}"))?;

        Ok(())
    }

    /// Helper function to calculate p-value between two optimizer result sets
    fn calculate_pairwise_p_value(
        &self,
        results_a: &[(f64, f64, String)],
        results_b: &[(f64, f64, String)],
    ) -> f64 {
        let values_a: Vec<f64> = results_a.iter().map(|(v, _, _)| *v).collect();
        let values_b: Vec<f64> = results_b.iter().map(|(v, _, _)| *v).collect();

        if values_a.len() < MIN_SAMPLE_SIZE || values_b.len() < MIN_SAMPLE_SIZE {
            return 1.0; // No significant difference if insufficient data
        }

        match self.welch_t_test(&values_a, &values_b) {
            Ok((_, p_value)) => p_value,
            Err(_) => 1.0,
        }
    }

    /// Helper function to calculate Cohen's d between two optimizer result sets
    fn calculate_pairwise_cohens_d(
        &self,
        results_a: &[(f64, f64, String)],
        results_b: &[(f64, f64, String)],
    ) -> f64 {
        let values_a: Vec<f64> = results_a.iter().map(|(v, _, _)| *v).collect();
        let values_b: Vec<f64> = results_b.iter().map(|(v, _, _)| *v).collect();

        if values_a.len() < MIN_SAMPLE_SIZE || values_b.len() < MIN_SAMPLE_SIZE {
            return 0.0;
        }

        self.cohens_d(&values_a, &values_b)
    }

    /// Helper function to escape LaTeX special characters
    fn escape_latex(&self, text: &str) -> String {
        text.chars()
            .map(|c| match c {
                '\\' => "\\textbackslash{}".to_string(),
                '&' => "\\&".to_string(),
                '%' => "\\%".to_string(),
                '$' => "\\$".to_string(),
                '#' => "\\#".to_string(),
                '^' => "\\textasciicircum{}".to_string(),
                '_' => "\\_".to_string(),
                '{' => "\\{".to_string(),
                '}' => "\\}".to_string(),
                '~' => "\\textasciitilde{}".to_string(),
                _ => c.to_string(),
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_welch_t_test() {
        let analysis = StatisticalAnalysis::new();
        // Test with equal samples
        let sample_a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let sample_b = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let (t_stat, p_value) = analysis.welch_t_test(&sample_a, &sample_b).unwrap();
        assert_eq!(t_stat, 0.0);
        assert_eq!(p_value, 1.0);
        // Test with different samples
        let sample_a = vec![1.0, 2.0, 3.0];
        let sample_b = vec![4.0, 5.0, 6.0];
        let (t_stat, p_value) = analysis.welch_t_test(&sample_a, &sample_b).unwrap();
        assert!(t_stat < 0.0); // sample_a has lower mean
        assert!(p_value < 0.05); // Should be significant
    }
    #[test]
    fn test_cohens_d() {
        let analysis = StatisticalAnalysis::new();
        // Test with no effect
        let sample_a = vec![1.0, 2.0, 3.0];
        let sample_b = vec![1.0, 2.0, 3.0];
        let d = analysis.cohens_d(&sample_a, &sample_b);
        assert_eq!(d, 0.0);
        // Test with large effect
        let sample_a = vec![1.0, 2.0, 3.0];
        let sample_b = vec![10.0, 11.0, 12.0];
        let d = analysis.cohens_d(&sample_a, &sample_b);
        assert!(d > COHEN_D_LARGE);
    }
    #[test]
    fn test_interpret_cohens_d() {
        let analysis = StatisticalAnalysis::new();
        assert_eq!(analysis.interpret_cohens_d(0.1), "negligible");
        assert_eq!(analysis.interpret_cohens_d(0.3), "small");
        assert_eq!(analysis.interpret_cohens_d(0.6), "medium");
        assert_eq!(analysis.interpret_cohens_d(1.0), "large");
    }
    #[test]
    fn test_escape_latex() {
        let analysis = StatisticalAnalysis::new();
        let input = "Test & % $ # ^ _ { } ~ \\";
        let expected = "Test \\& \\% \\$ \\# \\textasciicircum{} \\_ \\{ \\} \\textasciitilde{} \\textbackslash{}";
        assert_eq!(analysis.escape_latex(input), expected);
    }
}
