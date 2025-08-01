use crate::benchmarks::evaluation::SingleResult;

pub(crate) fn generate_parameter_evolution_analysis(
    runs: &[&SingleResult],
) -> anyhow::Result<String> {
    let mut content = String::from("## Parameter Evolution Analysis\n\n");
    // Find the run with the best final value for detailed analysis
    let best_run = runs
        .iter()
        .filter(|r| r.final_value.is_finite())
        .min_by(|a, b| {
            a.final_value
                .partial_cmp(&b.final_value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
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
            content.push_str(r#"<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
"#);
            content.push_str(
                r#"<tr style="background-color: #f2f2f2;">
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
"#,
            );
            content.push_str(
                r#"<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
"#,
            );
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
                    .map(|p| format!("{p:.3e}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                let params_display = if iter_data.parameters.len() > 5 {
                    format!("{params_str}, ...")
                } else {
                    params_str
                };
                content.push_str(&format!(
                    r#"<tr><td style="border: 1px solid #ddd; padding: 4px;">{}</td><td style="border: 1px solid #ddd; padding: 4px;">{:.3e}</td><td style="border: 1px solid #ddd; padding: 4px;">{:.3e}</td><td style="border: 1px solid #ddd; padding: 4px;">{:.3e}</td><td style="border: 1px solid #ddd; padding: 4px;">[{}]</td></tr>
"#,
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
