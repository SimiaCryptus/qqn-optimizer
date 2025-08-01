use crate::benchmarks::evaluation::{ConvergenceReason, SingleResult};

pub(crate) fn generate_failure_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
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
        match &run.convergence_reason {
            ConvergenceReason::TimeLimit => timeout_failures += 1,
            ConvergenceReason::NumericalError => numerical_failures += 1,
            ConvergenceReason::MaxIterations => max_iter_failures += 1,
            ConvergenceReason::MaxFunctionEvaluations => {
                if run.iterations < 10 {
                    early_failures += 1;
                }
            }
            _ => {}
        }
    }
    content.push_str(&format!(
        r#"### Failure Patterns
- **Early Failures (< 10 iterations):** {early_failures}
- **Timeout Failures:** {timeout_failures}
- **Numerical Errors:** {numerical_failures}
- **Maximum Iterations Reached:** {max_iter_failures}
"#
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
                    format!("- Error: {error}")
                } else {
                    String::new()
                }
            ));
        }
    }
    Ok(content)
}
