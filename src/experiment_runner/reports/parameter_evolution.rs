use crate::benchmarks::evaluation::SingleResult;
/// Configuration for parameter evolution display
struct EvolutionConfig {
    /// Maximum parameters to show in the table
    max_params_display: usize,
    /// Number of iterations to show from start and end
    iterations_per_side: usize,
    /// Maximum total iterations to show without truncation
    max_iterations_full: usize,
}
impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            max_params_display: 5,
            iterations_per_side: 5,
            max_iterations_full: 10,
        }
    }
}
/// Finds the run with the best (minimum) final value
fn find_best_run<'a>(runs: &'a [&'a SingleResult]) -> Option<&'a SingleResult> {
    runs.iter()
        .filter(|r| r.final_value.is_finite())
        .min_by(|a, b| a.final_value.partial_cmp(&b.final_value).unwrap_or(std::cmp::Ordering::Equal))
}


pub(crate) fn generate_parameter_evolution_analysis(
    runs: &[&SingleResult],
) -> anyhow::Result<String> {
    let mut content = String::from("## Parameter Evolution Analysis\n\n");
    // Find the run with the best final value for detailed analysis
    if runs.is_empty() {
        content.push_str("*No runs available for analysis.*\n");
        return Ok(content);
    }
    
    let config = EvolutionConfig::default();
    let best_run = find_best_run(runs);
    
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
            content.push_str("\n#### Parameter Evolution (Selected Iterations)\n\n");
            content.push_str(&generate_table_header());
            let iterations_to_show = select_iterations_to_display(
                &best_run.trace.iterations,
                &config
            );
            
            for iter_data in iterations_to_show {
                content.push_str(&generate_table_row(iter_data, config.max_params_display));
            }
            
            content.push_str("</table>\n\n");
        }
    } else {
        content.push_str("*No valid runs with finite values found.*\n");
    }
    
    Ok(content)
}
/// Generates the HTML table header
fn generate_table_header() -> String {
    const TABLE_STYLE: &str = "border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;";
    const HEADER_STYLE: &str = "background-color: #f2f2f2;";
    const CELL_STYLE: &str = "border: 1px solid #ddd; padding: 4px;";
    format!(
        r#"<table style="{TABLE_STYLE}">
<tr style="{HEADER_STYLE}">
<th style="{CELL_STYLE}">Iteration</th>
<th style="{CELL_STYLE}">Function Value</th>
<th style="{CELL_STYLE}">Gradient Norm</th>
<th style="{CELL_STYLE}">Step Size</th>
<th style="{CELL_STYLE}">Parameters (first {})</th>
</tr>
"#,
        EvolutionConfig::default().max_params_display
    )
}
/// Generates a single table row for an iteration
fn generate_table_row(iter_data: &crate::benchmarks::evaluation::IterationData, max_params: usize) -> String {
    const CELL_STYLE: &str = "border: 1px solid #ddd; padding: 4px;";
    let params_display = format_parameters(&iter_data.parameters, max_params);
    format!(
        r#"<tr>
<td style="{CELL_STYLE}">{}</td>
<td style="{CELL_STYLE}">{:.3e}</td>
<td style="{CELL_STYLE}">{:.3e}</td>
<td style="{CELL_STYLE}">{:.3e}</td>
<td style="{CELL_STYLE}">[{}]</td>
</tr>
"#,
        iter_data.iteration,
        iter_data.function_value,
        iter_data.gradient_norm,
        iter_data.step_size,
        params_display
    )
}
/// Formats parameters for display, truncating if necessary
fn format_parameters(parameters: &[f64], max_display: usize) -> String {
    let params_str = parameters
        .iter()
        .take(max_display)
        .map(|p| format!("{p:.3e}"))
        .collect::<Vec<_>>()
        .join(", ");
    if parameters.len() > max_display {
        format!("{params_str}, ...")
    } else {
        params_str
    }
}
/// Selects which iterations to display based on the configuration
fn select_iterations_to_display<'a>(
    iterations: &'a [crate::benchmarks::evaluation::IterationData],
    config: &EvolutionConfig,
) -> Vec<&'a crate::benchmarks::evaluation::IterationData> {
    if iterations.len() <= config.max_iterations_full {
        iterations.iter().collect()
    } else {
        let mut selected = Vec::new();
        // First N iterations
        let first_count = config.iterations_per_side.min(iterations.len());
        for i in 0..first_count {
            selected.push(&iterations[i]);
        }
        // Last N iterations (avoiding overlap)
        let last_start = iterations.len().saturating_sub(config.iterations_per_side);
        let last_start = last_start.max(first_count);
        for i in last_start..iterations.len() {
            selected.push(&iterations[i]);
        }
        selected
    }
}