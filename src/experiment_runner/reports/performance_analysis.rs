use crate::benchmarks::evaluation::SingleResult;

pub fn generate_performance_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let mut content = String::from("## Performance Analysis\n\n");
    let total_func_evals: usize = runs.iter().map(|r| r.function_evaluations).sum();
    let total_grad_evals: usize = runs.iter().map(|r| r.gradient_evaluations).sum();
    let total_time: f64 = runs.iter().map(|r| r.execution_time.as_secs_f64()).sum();
    let total_iterations: usize = runs.iter().map(|r| r.iterations).sum();
    let avg_func_evals = total_func_evals as f64 / runs.len() as f64;
    let avg_grad_evals = total_grad_evals as f64 / runs.len() as f64;
    let avg_time = total_time / runs.len() as f64;
    let avg_iterations = total_iterations as f64 / runs.len() as f64;
    content.push_str(&format!(
        r#"### Computational Efficiency
- **Average Function Evaluations per Run:** {:.1}
- **Average Gradient Evaluations per Run:** {:.1}
- **Average Iterations per Run:** {:.1}
- **Average Time per Run:** {:.3}s
- **Function Evaluations per Second:** {:.1}
- **Iterations per Second:** {:.1}
### Resource Utilization
- **Total Function Evaluations:** {}
- **Total Gradient Evaluations:** {}
- **Total Computation Time:** {:.1}s
- **Function/Gradient Ratio:** {:.2}
"#,
        avg_func_evals,
        avg_grad_evals,
        avg_iterations,
        avg_time,
        if avg_time > 0.0 {
            avg_func_evals / avg_time
        } else {
            0.0
        },
        if avg_time > 0.0 {
            avg_iterations / avg_time
        } else {
            0.0
        },
        total_func_evals,
        total_grad_evals,
        total_time,
        if total_grad_evals > 0 {
            total_func_evals as f64 / total_grad_evals as f64
        } else {
            0.0
        }
    ));
    Ok(content)
}