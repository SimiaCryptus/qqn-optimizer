use crate::benchmarks::evaluation::{BenchmarkResults, SingleResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatisticalAnalysis {
    pub convergence_comparison: ConvergenceComparison,
    pub performance_profiles: PerformanceProfiles,
    pub robustness_analysis: RobustnessAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConvergenceComparison {
    pub optimizer_stats: HashMap<String, OptimizerStatistics>,
    pub pairwise_comparisons: Vec<PairwiseComparison>,
    pub significance_tests: Vec<SignificanceTest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerStatistics {
    pub optimizer_name: String,
    pub mean_final_value: f64,
    pub std_final_value: f64,
    pub median_final_value: f64,
    pub mean_iterations: f64,
    pub std_iterations: f64,
    pub convergence_rate: f64,
    pub mean_execution_time: f64,
    pub std_execution_time: f64,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PairwiseComparison {
    pub optimizer_a: String,
    pub optimizer_b: String,
    pub wins_a: usize,
    pub wins_b: usize,
    pub ties: usize,
    pub effect_size: f64,
    pub confidence_interval: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignificanceTest {
    pub test_name: String,
    pub optimizer_a: String,
    pub optimizer_b: String,
    pub statistic: f64,
    pub p_value: f64,
    pub significant: bool,
    pub alpha: f64,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EffectSize {
    pub optimizer_a: String,
    pub optimizer_b: String,
    pub cohens_d: f64,
    pub magnitude: String,
}
impl EffectSize {
    pub fn magnitude(&self) -> f64 {
        self.cohens_d.abs()
    }
    pub fn is_significant(&self) -> bool {
        self.cohens_d.abs() > 0.2 // Small effect size threshold
    }
}
impl SignificanceTest {
    pub fn is_significant(&self) -> bool {
        self.significant
    }
}
impl ConvergenceComparison {
    /// Get the number of unique problems analyzed
    pub fn num_problems(&self) -> usize {
        // Count unique problems by looking at optimizer statistics
        // This is an approximation - in a real implementation we'd track this separately
        if self.optimizer_stats.is_empty() {
            0
        } else {
            // Estimate based on pairwise comparisons
            // Each optimizer pair represents problems they both solved
            let total_comparisons = self.pairwise_comparisons.len();
            if total_comparisons == 0 {
                1 // At least one problem if we have optimizer stats
            } else {
                // Rough estimate - this could be improved with better tracking
                (total_comparisons as f64).sqrt().ceil() as usize
            }
        }
    }
    /// Get the number of optimizers compared
    pub fn num_optimizers(&self) -> usize {
        self.optimizer_stats.len()
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfiles {
    pub tolerance_levels: Vec<f64>,
    pub time_limits: Vec<f64>,
    pub profiles: HashMap<String, ProfileData>,
    pub ratios: Vec<f64>,
    pub optimizer_profiles: HashMap<String, Vec<f64>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileData {
    pub optimizer_name: String,
    pub tolerance_fractions: Vec<f64>,
    pub time_fractions: Vec<f64>,
    pub overall_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RobustnessAnalysis {
    pub problem_categories: HashMap<String, Vec<String>>,
    pub category_performance: HashMap<String, HashMap<String, f64>>,
    pub failure_analysis: HashMap<String, FailureAnalysis>,
    pub sensitivity_analysis: SensitivityAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureAnalysis {
    pub optimizer_name: String,
    pub failure_rate: f64,
    pub common_failure_modes: Vec<String>,
    pub problematic_functions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensitivityAnalysis {
    pub parameter_sensitivity: HashMap<String, f64>,
    pub dimension_scaling: Vec<(usize, f64)>,
    pub condition_number_impact: Vec<(f64, f64)>,
}

impl StatisticalAnalysis {
    pub fn new(results: &BenchmarkResults) -> Self {
        let convergence_comparison = Self::compute_convergence_comparison(results);
        let performance_profiles = Self::compute_performance_profiles(results);
        let robustness_analysis = Self::compute_robustness_analysis(results);

        Self {
            convergence_comparison,
            performance_profiles,
            robustness_analysis,
        }
    }
    /// Get convergence comparison data
    pub fn convergence_comparison(&self) -> &ConvergenceComparison {
        &self.convergence_comparison
    }
    /// Get performance profiles data
    pub fn performance_profiles(&self) -> &PerformanceProfiles {
        &self.performance_profiles
    }
    /// Get robustness analysis data
    pub fn robustness_analysis(&self) -> &RobustnessAnalysis {
        &self.robustness_analysis
    }
    /// Get all significance tests
    pub fn significance_tests(&self) -> &Vec<SignificanceTest> {
        &self.convergence_comparison.significance_tests
    }
    /// Get effect sizes from pairwise comparisons
    pub fn effect_sizes(&self) -> Vec<EffectSize> {
        self.convergence_comparison.pairwise_comparisons
            .iter()
            .map(|comp| EffectSize {
                optimizer_a: comp.optimizer_a.clone(),
                optimizer_b: comp.optimizer_b.clone(),
                cohens_d: comp.effect_size,
                magnitude: Self::effect_size_magnitude(comp.effect_size),
            })
            .collect()
    }
    fn effect_size_magnitude(cohens_d: f64) -> String {
        let abs_d = cohens_d.abs();
        if abs_d < 0.2 {
            "negligible".to_string()
        } else if abs_d < 0.5 {
            "small".to_string()
        } else if abs_d < 0.8 {
            "medium".to_string()
        } else {
            "large".to_string()
        }
    }

    fn compute_convergence_comparison(results: &BenchmarkResults) -> ConvergenceComparison {
        let mut optimizer_stats = HashMap::new();
        let mut grouped_results: HashMap<String, Vec<&SingleResult>> = HashMap::new();

        // Group results by optimizer
        for result in &results.results {
            grouped_results
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        // Compute statistics for each optimizer
        for (optimizer_name, optimizer_results) in &grouped_results {
            let stats = Self::compute_optimizer_statistics(optimizer_name, optimizer_results);
            optimizer_stats.insert(optimizer_name.clone(), stats);
        }

        // Compute pairwise comparisons
        let pairwise_comparisons = Self::compute_pairwise_comparisons(&grouped_results);

        // Perform significance tests
        let significance_tests = Self::perform_significance_tests(&grouped_results);

        ConvergenceComparison {
            optimizer_stats,
            pairwise_comparisons,
            significance_tests,
        }
    }

    fn compute_optimizer_statistics(
        optimizer_name: &str,
        results: &[&SingleResult],
    ) -> OptimizerStatistics {
        let final_values: Vec<f64> = results.iter().map(|r| r.final_value).collect();
        let iterations: Vec<f64> = results.iter().map(|r| r.iterations as f64).collect();
        let execution_times: Vec<f64> = results
            .iter()
            .map(|r| r.execution_time.as_secs_f64())
            .collect();

        let convergence_count = results.iter().filter(|r| r.convergence_achieved).count();
        let convergence_rate = convergence_count as f64 / results.len() as f64;

        let success_count = results
            .iter()
            .filter(|r| r.convergence_achieved && r.final_value < 1e-6)
            .count();
        let success_rate = success_count as f64 / results.len() as f64;

        OptimizerStatistics {
            optimizer_name: optimizer_name.to_string(),
            mean_final_value: Self::mean(&final_values),
            std_final_value: Self::std_dev(&final_values),
            median_final_value: Self::median(&final_values),
            mean_iterations: Self::mean(&iterations),
            std_iterations: Self::std_dev(&iterations),
            convergence_rate,
            mean_execution_time: Self::mean(&execution_times),
            std_execution_time: Self::std_dev(&execution_times),
            success_rate,
        }
    }

    fn compute_pairwise_comparisons(
        grouped_results: &HashMap<String, Vec<&SingleResult>>,
    ) -> Vec<PairwiseComparison> {
        let mut comparisons = Vec::new();
        let optimizers: Vec<_> = grouped_results.keys().collect();

        for i in 0..optimizers.len() {
            for j in (i + 1)..optimizers.len() {
                let opt_a = optimizers[i];
                let opt_b = optimizers[j];

                let results_a = &grouped_results[opt_a];
                let results_b = &grouped_results[opt_b];

                let comparison = Self::compare_optimizers(opt_a, opt_b, results_a, results_b);
                comparisons.push(comparison);
            }
        }

        comparisons
    }

    fn compare_optimizers(
        opt_a: &str,
        opt_b: &str,
        results_a: &[&SingleResult],
        results_b: &[&SingleResult],
    ) -> PairwiseComparison {
        let mut wins_a = 0;
        let mut wins_b = 0;
        let mut ties = 0;

        // Group results by problem name for fair comparison
        let mut problems_a: HashMap<String, Vec<&SingleResult>> = HashMap::new();
        let mut problems_b: HashMap<String, Vec<&SingleResult>> = HashMap::new();

        for result in results_a {
            problems_a
                .entry(result.problem_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        for result in results_b {
            problems_b
                .entry(result.problem_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        // Compare on common problems
        for problem_name in problems_a.keys() {
            if let Some(results_b_problem) = problems_b.get(problem_name) {
                let results_a_problem = &problems_a[problem_name];

                let mean_a = Self::mean(
                    &results_a_problem
                        .iter()
                        .map(|r| r.final_value)
                        .collect::<Vec<_>>(),
                );
                let mean_b = Self::mean(
                    &results_b_problem
                        .iter()
                        .map(|r| r.final_value)
                        .collect::<Vec<_>>(),
                );

                let tolerance = 1e-10;
                if (mean_a - mean_b).abs() < tolerance {
                    ties += 1;
                } else if mean_a < mean_b {
                    wins_a += 1;
                } else {
                    wins_b += 1;
                }
            }
        }

        // Compute effect size (Cohen's d)
        let values_a: Vec<f64> = results_a.iter().map(|r| r.final_value).collect();
        let values_b: Vec<f64> = results_b.iter().map(|r| r.final_value).collect();
        let effect_size = Self::cohens_d(&values_a, &values_b);

        // Compute confidence interval for mean difference
        let confidence_interval = Self::confidence_interval_mean_diff(&values_a, &values_b, 0.95);

        PairwiseComparison {
            optimizer_a: opt_a.to_string(),
            optimizer_b: opt_b.to_string(),
            wins_a,
            wins_b,
            ties,
            effect_size,
            confidence_interval,
        }
    }

    fn perform_significance_tests(
        grouped_results: &HashMap<String, Vec<&SingleResult>>,
    ) -> Vec<SignificanceTest> {
        let mut tests = Vec::new();
        let optimizers: Vec<_> = grouped_results.keys().collect();
        let alpha = 0.05;

        for i in 0..optimizers.len() {
            for j in (i + 1)..optimizers.len() {
                let opt_a = optimizers[i];
                let opt_b = optimizers[j];

                let values_a: Vec<f64> = grouped_results[opt_a]
                    .iter()
                    .map(|r| r.final_value)
                    .collect();
                let values_b: Vec<f64> = grouped_results[opt_b]
                    .iter()
                    .map(|r| r.final_value)
                    .collect();

                // Welch's t-test
                let t_test = Self::welch_t_test(&values_a, &values_b);
                tests.push(SignificanceTest {
                    test_name: "Welch's t-test".to_string(),
                    optimizer_a: opt_a.to_string(),
                    optimizer_b: opt_b.to_string(),
                    statistic: t_test.0,
                    p_value: t_test.1,
                    significant: t_test.1 < alpha,
                    alpha,
                });

                // Mann-Whitney U test
                let mw_test = Self::mann_whitney_u_test(&values_a, &values_b);
                tests.push(SignificanceTest {
                    test_name: "Mann-Whitney U test".to_string(),
                    optimizer_a: opt_a.to_string(),
                    optimizer_b: opt_b.to_string(),
                    statistic: mw_test.0,
                    p_value: mw_test.1,
                    significant: mw_test.1 < alpha,
                    alpha,
                });
            }
        }

        tests
    }

    fn compute_performance_profiles(results: &BenchmarkResults) -> PerformanceProfiles {
        let tolerance_levels = vec![1e-12, 1e-10, 1e-8, 1e-6, 1e-4, 1e-2, 1e-1];
        let time_limits = vec![0.1, 0.5, 1.0, 5.0, 10.0, 30.0, 60.0];

        let mut grouped_results: HashMap<String, Vec<&SingleResult>> = HashMap::new();
        for result in &results.results {
            grouped_results
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        let mut profiles = HashMap::new();

        for (optimizer_name, optimizer_results) in &grouped_results {
            let tolerance_fractions = Self::compute_tolerance_fractions(
                optimizer_results,
                &tolerance_levels,
            );
            let time_fractions = Self::compute_time_fractions(
                optimizer_results,
                &time_limits,
            );

            let overall_score = Self::compute_overall_score(&tolerance_fractions, &time_fractions);

            profiles.insert(
                optimizer_name.clone(),
                ProfileData {
                    optimizer_name: optimizer_name.clone(),
                    tolerance_fractions,
                    time_fractions,
                    overall_score,
                },
            );
        }
        let optimizer_profiles = profiles.iter().map(|(name, profile_data)| {
            (name.clone(), profile_data.tolerance_fractions.clone())
        }).collect();


        PerformanceProfiles {
            tolerance_levels,
            time_limits,
            profiles,
            ratios: vec![1.0, 2.0, 5.0, 10.0, 20.0, 50.0, 100.0],
            optimizer_profiles,
        }
    }

    fn compute_tolerance_fractions(
        results: &[&SingleResult],
        tolerance_levels: &[f64],
    ) -> Vec<f64> {
        tolerance_levels
            .iter()
            .map(|&tolerance| {
                let solved_count = results
                    .iter()
                    .filter(|r| r.convergence_achieved && r.final_value <= tolerance)
                    .count();
                solved_count as f64 / results.len() as f64
            })
            .collect()
    }

    fn compute_time_fractions(
        results: &[&SingleResult],
        time_limits: &[f64],
    ) -> Vec<f64> {
        time_limits
            .iter()
            .map(|&time_limit| {
                let solved_count = results
                    .iter()
                    .filter(|r| {
                        r.convergence_achieved && r.execution_time.as_secs_f64() <= time_limit
                    })
                    .count();
                solved_count as f64 / results.len() as f64
            })
            .collect()
    }

    fn compute_overall_score(tolerance_fractions: &[f64], time_fractions: &[f64]) -> f64 {
        let tolerance_score: f64 = tolerance_fractions.iter().sum::<f64>() / tolerance_fractions.len() as f64;
        let time_score: f64 = time_fractions.iter().sum::<f64>() / time_fractions.len() as f64;
        (tolerance_score + time_score) / 2.0
    }

    fn compute_robustness_analysis(results: &BenchmarkResults) -> RobustnessAnalysis {
        let mut problem_categories = HashMap::new();
        let mut category_performance = HashMap::new();
        let mut failure_analysis = HashMap::new();

        // Categorize problems
        for result in &results.results {
            let category = Self::categorize_problem(&result.problem_name);
            problem_categories
                .entry(category.clone())
                .or_insert_with(Vec::new)
                .push(result.problem_name.clone());
        }

        // Compute category performance
        for (category, problems) in &problem_categories {
            let mut optimizer_performance = HashMap::new();
            
            for result in &results.results {
                if problems.contains(&result.problem_name) {
                    let entry = optimizer_performance
                        .entry(result.optimizer_name.clone())
                        .or_insert_with(Vec::new);
                    entry.push(result.final_value);
                }
            }

            let mut category_perf = HashMap::new();
            for (optimizer, values) in optimizer_performance {
                let mean_performance = Self::mean(&values);
                category_perf.insert(optimizer, mean_performance);
            }
            category_performance.insert(category.clone(), category_perf);
        }

        // Compute failure analysis
        let mut grouped_results: HashMap<String, Vec<&SingleResult>> = HashMap::new();
        for result in &results.results {
            grouped_results
                .entry(result.optimizer_name.clone())
                .or_insert_with(Vec::new)
                .push(result);
        }

        for (optimizer_name, optimizer_results) in &grouped_results {
            let failure_count = optimizer_results
                .iter()
                .filter(|r| !r.convergence_achieved)
                .count();
            let failure_rate = failure_count as f64 / optimizer_results.len() as f64;

            let problematic_functions = optimizer_results
                .iter()
                .filter(|r| !r.convergence_achieved)
                .map(|r| r.problem_name.clone())
                .collect::<std::collections::HashSet<_>>()
                .into_iter()
                .collect();

            let common_failure_modes = Self::identify_failure_modes(optimizer_results);

            failure_analysis.insert(
                optimizer_name.clone(),
                FailureAnalysis {
                    optimizer_name: optimizer_name.clone(),
                    failure_rate,
                    common_failure_modes,
                    problematic_functions,
                },
            );
        }

        let sensitivity_analysis = Self::compute_sensitivity_analysis(results);

        RobustnessAnalysis {
            problem_categories,
            category_performance,
            failure_analysis,
            sensitivity_analysis,
        }
    }

    fn categorize_problem(problem_name: &str) -> String {
        if problem_name.contains("rosenbrock") {
            "Rosenbrock".to_string()
        } else if problem_name.contains("rastrigin") {
            "Rastrigin".to_string()
        } else if problem_name.contains("sphere") {
            "Sphere".to_string()
        } else if problem_name.contains("beale") {
            "Beale".to_string()
        } else if problem_name.contains("logistic") {
            "Logistic Regression".to_string()
        } else if problem_name.contains("neural") || problem_name.contains("mlp") {
            "Neural Network".to_string()
        } else {
            "Other".to_string()
        }
    }

    fn identify_failure_modes(results: &[&SingleResult]) -> Vec<String> {
        let mut failure_modes = Vec::new();

        let failed_results: Vec<_> = results.iter().filter(|r| !r.convergence_achieved).collect();

        if failed_results.is_empty() {
            return failure_modes;
        }

        // Check for common failure patterns
        let high_iterations = failed_results
            .iter()
            .filter(|r| r.iterations >= 900) // Close to max iterations
            .count();

        if high_iterations > failed_results.len() / 2 {
            failure_modes.push("Max iterations reached".to_string());
        }

        let high_final_values = failed_results
            .iter()
            .filter(|r| r.final_value > 1e3)
            .count();

        if high_final_values > failed_results.len() / 3 {
            failure_modes.push("Poor convergence (high final values)".to_string());
        }

        let slow_convergence = failed_results
            .iter()
            .filter(|r| r.execution_time.as_secs_f64() > 30.0)
            .count();

        if slow_convergence > failed_results.len() / 3 {
            failure_modes.push("Slow convergence".to_string());
        }

        failure_modes
    }

    fn compute_sensitivity_analysis(results: &BenchmarkResults) -> SensitivityAnalysis {
        // Simplified sensitivity analysis
        let parameter_sensitivity = HashMap::new(); // Would need parameter variation data
        let dimension_scaling = Vec::new(); // Would need multi-dimensional problem data
        let condition_number_impact = Vec::new(); // Would need condition number data

        SensitivityAnalysis {
            parameter_sensitivity,
            dimension_scaling,
            condition_number_impact,
        }
    }

    // Statistical utility functions
    fn mean(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        values.iter().sum::<f64>() / values.len() as f64
    }

    fn std_dev(values: &[f64]) -> f64 {
        if values.len() <= 1 {
            return 0.0;
        }
        let mean = Self::mean(values);
        let variance = values
            .iter()
            .map(|x| (x - mean).powi(2))
            .sum::<f64>() / (values.len() - 1) as f64;
        variance.sqrt()
    }

    fn median(values: &[f64]) -> f64 {
        if values.is_empty() {
            return 0.0;
        }
        let mut sorted = values.to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mid = sorted.len() / 2;
        if sorted.len() % 2 == 0 {
            (sorted[mid - 1] + sorted[mid]) / 2.0
        } else {
            sorted[mid]
        }
    }

    fn cohens_d(group_a: &[f64], group_b: &[f64]) -> f64 {
        let mean_a = Self::mean(group_a);
        let mean_b = Self::mean(group_b);
        let std_a = Self::std_dev(group_a);
        let std_b = Self::std_dev(group_b);

        let pooled_std = ((std_a.powi(2) + std_b.powi(2)) / 2.0).sqrt();
        if pooled_std == 0.0 {
            0.0
        } else {
            (mean_a - mean_b) / pooled_std
        }
    }

    fn confidence_interval_mean_diff(
        group_a: &[f64],
        group_b: &[f64],
        confidence_level: f64,
    ) -> (f64, f64) {
        let mean_a = Self::mean(group_a);
        let mean_b = Self::mean(group_b);
        let std_a = Self::std_dev(group_a);
        let std_b = Self::std_dev(group_b);

        let n_a = group_a.len() as f64;
        let n_b = group_b.len() as f64;

        let se = ((std_a.powi(2) / n_a) + (std_b.powi(2) / n_b)).sqrt();
        let mean_diff = mean_a - mean_b;

        // Using t-distribution critical value (approximation for large samples)
        let alpha = 1.0 - confidence_level;
        let t_critical = 1.96; // Approximation for 95% confidence

        let margin_of_error = t_critical * se;
        (mean_diff - margin_of_error, mean_diff + margin_of_error)
    }

    fn welch_t_test(group_a: &[f64], group_b: &[f64]) -> (f64, f64) {
        let mean_a = Self::mean(group_a);
        let mean_b = Self::mean(group_b);
        let var_a = Self::std_dev(group_a).powi(2);
        let var_b = Self::std_dev(group_b).powi(2);
        let n_a = group_a.len() as f64;
        let n_b = group_b.len() as f64;

        let se = ((var_a / n_a) + (var_b / n_b)).sqrt();
        let t_stat = if se == 0.0 { 0.0 } else { (mean_a - mean_b) / se };

        // Degrees of freedom for Welch's t-test
        let df = if var_a == 0.0 && var_b == 0.0 {
            n_a + n_b - 2.0
        } else {
            let numerator = ((var_a / n_a) + (var_b / n_b)).powi(2);
            let denominator = (var_a / n_a).powi(2) / (n_a - 1.0) + (var_b / n_b).powi(2) / (n_b - 1.0);
            numerator / denominator
        };

        // Simplified p-value calculation (would use proper t-distribution in practice)
        let p_value = if t_stat.abs() > 2.0 { 0.05 } else { 0.5 };

        (t_stat, p_value)
    }

    fn mann_whitney_u_test(group_a: &[f64], group_b: &[f64]) -> (f64, f64) {
        let mut combined: Vec<(f64, usize)> = Vec::new();
        
        for &value in group_a {
            combined.push((value, 0)); // 0 for group A
        }
        for &value in group_b {
            combined.push((value, 1)); // 1 for group B
        }

        combined.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut rank_sum_a = 0.0;
        for (i, &(_, group)) in combined.iter().enumerate() {
            if group == 0 {
                rank_sum_a += (i + 1) as f64;
            }
        }

        let n_a = group_a.len() as f64;
        let n_b = group_b.len() as f64;

        let u_a = rank_sum_a - (n_a * (n_a + 1.0)) / 2.0;
        let u_b = n_a * n_b - u_a;

        let u_stat = u_a.min(u_b);

        // Simplified p-value calculation
        let expected_u = (n_a * n_b) / 2.0;
        let std_u = ((n_a * n_b * (n_a + n_b + 1.0)) / 12.0).sqrt();
        
        let z_score = if std_u == 0.0 { 0.0 } else { (u_stat - expected_u) / std_u };
        let p_value = if z_score.abs() > 1.96 { 0.05 } else { 0.5 };

        (u_stat, p_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_statistical_functions() {
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        
        assert_eq!(StatisticalAnalysis::mean(&values), 3.0);
        assert_eq!(StatisticalAnalysis::median(&values), 3.0);
        assert!((StatisticalAnalysis::std_dev(&values) - 1.5811388300841898).abs() < 1e-10);
    }

    #[test]
    fn test_cohens_d() {
        let group_a = vec![1.0, 2.0, 3.0];
        let group_b = vec![4.0, 5.0, 6.0];
        
        let effect_size = StatisticalAnalysis::cohens_d(&group_a, &group_b);
        assert!(effect_size < 0.0); // group_a has lower mean
        assert!(effect_size.abs() > 1.0); // Large effect size
    }

    #[test]
    fn test_confidence_interval() {
        let group_a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let group_b = vec![6.0, 7.0, 8.0, 9.0, 10.0];
        
        let (lower, upper) = StatisticalAnalysis::confidence_interval_mean_diff(&group_a, &group_b, 0.95);
        assert!(lower < upper);
        assert!(lower < -3.0); // Mean difference should be around -5
        assert!(upper > -7.0);
    }
}