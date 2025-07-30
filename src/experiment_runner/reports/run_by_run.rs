use crate::benchmarks::evaluation::SingleResult;

pub(crate) fn generate_run_by_run_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let mut content = String::from(
        r#"## Run-by-Run Analysis
<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 12px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Run</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Final Value</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Iterations</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Evals</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Evals</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Time (s)</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Converged</th>
<th style="border: 1px solid #ddd; padding: 6px; text-align: center;">Reason</th>
</tr>
"#,
    );
    for (i, run) in runs.iter().enumerate() {
        let style = if run.convergence_achieved {
            "background-color: #d4edda;"
        } else {
            "background-color: #f8d7da;"
        };
        let convergence_reason = format!("{:?}", run.convergence_reason)
            .replace("GradientTolerance", "Grad Tol")
            .replace("FunctionTolerance", "Func Tol")
            .replace("MaxIterations", "Max Iter")
            .replace("MaxFunctionEvaluations", "Max Func")
            .replace("TimeLimit", "Time")
            .replace("NumericalError", "Num Err")
            .replace("\"", "");
        content.push_str(&format!(
            r#"<tr style="{}">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{:.3e}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{:.3e}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{:.3}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">{}</td>
</tr>
"#,
            style,
            i + 1,
            run.final_value,
            run.final_gradient_norm,
            run.iterations,
            run.function_evaluations,
            run.gradient_evaluations,
            run.execution_time.as_secs_f64(),
            if run.convergence_achieved {
                "Yes"
            } else {
                "No"
            },
            convergence_reason
        ));
    }
    content.push_str("</table>\n\n");
    Ok(content)
}
