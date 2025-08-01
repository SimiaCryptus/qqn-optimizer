use crate::benchmarks::evaluation::{ConvergenceReason, SingleResult};
use std::collections::HashMap;

pub(crate) fn generate_failure_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let failed_runs: Vec<_> = runs.iter().filter(|r| !r.convergence_achieved).collect();
    if failed_runs.is_empty() {
        return Ok("## Failure Analysis\n\n*No failed runs to analyze.*\n\n".to_string());
    }
    let mut content = String::from("## Failure Analysis\n\n");
    content.push_str(&format!("Total failed runs: {} out of {}\n\n", failed_runs.len(), runs.len()));
    
    // Analyze failure patterns
    let mut early_failures = 0;
    let mut timeout_failures = 0;
    let mut numerical_failures = 0;
    let mut max_iter_failures = 0;
    let mut function_eval_failures = 0;
    for run in &failed_runs {
        match &run.convergence_reason {
            ConvergenceReason::TimeLimit => timeout_failures += 1,
            ConvergenceReason::NumericalError => numerical_failures += 1,
            ConvergenceReason::MaxIterations => max_iter_failures += 1,
            ConvergenceReason::MaxFunctionEvaluations => {
                if run.iterations < 10 {
                    early_failures += 1;
                } else {
                    function_eval_failures += 1;
                }
            }
            _ => {}
        }
    }
    content.push_str(&format!(
        r#"### Failure Patterns
- **Function Evaluation Limit:** {function_eval_failures}
"#
    ));
    // Add failure rate by problem if there are multiple problems
    let problems: HashMap<&str, Vec<&&SingleResult>> = failed_runs.iter()
        .fold(HashMap::new(), |mut acc, run| {
            acc.entry(run.problem_name.as_str()).or_insert_with(Vec::new).push(run);
            acc
        });
    if problems.len() > 1 {
        content.push_str("\n### Failure Rate by Problem\n\n");
        content.push_str("| Problem | Failed Runs | Failure Rate |\n");
        content.push_str("|---------|-------------|-------------|\n");
        for (problem_name, problem_failed_runs) in &problems {
            let total_problem_runs = runs.iter().filter(|r| r.problem_name == *problem_name).count();
            let failure_rate = (problem_failed_runs.len() as f64 / total_problem_runs as f64) * 100.0;
            content.push_str(&format!(
                "| {} | {} | {:.1}% |\n",
                problem_name,
                problem_failed_runs.len(),
                failure_rate
            ));
        }
        content.push_str("\n");
    }
    
    // Show details of failed runs
    let max_detailed_runs = 10;
    if failed_runs.len() <= max_detailed_runs {
        content.push_str("### Failed Run Details\n\n");
        for (i, run) in failed_runs.iter().enumerate() {
            content.push_str(&format!(
                r#"**Run {} (ID: {}, Problem: {})**
- Iterations: {}
- Function Evaluations: {}
- Reason: {:?}
{}
"#,
                i + 1,
                run.run_id + 1,
                run.problem_name,
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
    } else {
        content.push_str(&format!(
            "\n### Failed Run Summary\n\nShowing {} most representative failures out of {}:\n\n",
            max_detailed_runs,
            failed_runs.len()
        ));
        // Show a sample of different failure types
        let mut shown = 0;
        for reason in &[
            ConvergenceReason::NumericalError,
            ConvergenceReason::TimeLimit,
            ConvergenceReason::MaxIterations,
            ConvergenceReason::MaxFunctionEvaluations,
        ] {
            if let Some(run) = failed_runs.iter().find(|r| &r.convergence_reason == reason) {
                content.push_str(&format!(
                    "- **{:?}** example: Problem {}, {} iterations, final value: {:.6e}\n",
                    reason,
                    run.problem_name,
                    run.iterations,
                    run.final_value
                ));
                shown += 1;
                if shown >= max_detailed_runs { break; }
            }
        }
    }
    content.push_str("\n");
    Ok(content)
}