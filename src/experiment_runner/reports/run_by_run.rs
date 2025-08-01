use crate::benchmarks::evaluation::SingleResult;
/// CSS styles for the run-by-run analysis table
const TABLE_STYLE: &str = "border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 12px;";
const HEADER_STYLE: &str = "background-color: #f2f2f2;";
const CELL_STYLE: &str = "border: 1px solid #ddd; padding: 6px; text-align: center;";
const SUCCESS_ROW_STYLE: &str = "background-color: #d4edda;";
const FAILURE_ROW_STYLE: &str = "background-color: #f8d7da;";
/// Generates a detailed run-by-run analysis table in HTML format
///
/// # Arguments
/// * `runs` - Slice of references to SingleResult instances
///
/// # Returns
/// * `Result<String>` - HTML string containing the formatted table

pub(crate) fn generate_run_by_run_analysis(runs: &[&SingleResult]) -> anyhow::Result<String> {
    let mut content = String::from(
        format!(
            r#"## Run-by-Run Analysis
<table style="{}">
<thead>
<tr style="{}">
<th style="{}">Run</th>
<th style="{}">Final Value</th>
<th style="{}">Gradient Norm</th>
<th style="{}">Iterations</th>
<th style="{}">Func Evals</th>
<th style="{}">Grad Evals</th>
<th style="{}">Time (s)</th>
<th style="{}">Converged</th>
<th style="{}">Reason</th>
</tr>
</thead>
<tbody>
"#,
            TABLE_STYLE,
            HEADER_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE,
            CELL_STYLE
        ),
    );

    for (i, run) in runs.iter().enumerate() {
        let row_style = if run.convergence_achieved {
            SUCCESS_ROW_STYLE
        } else {
            FAILURE_ROW_STYLE
        };
        
        let convergence_reason = format_convergence_reason(&run.convergence_reason);
        
        content.push_str(&format!(
            r#"<tr style="{}">
<td style="{}">{}</td>
<td style="{}">{:.3e}</td>
<td style="{}">{:.3e}</td>
<td style="{}">{}</td>
<td style="{}">{}</td>
<td style="{}">{}</td>
<td style="{}">{:.3}</td>
<td style="{}">{}</td>
<td style="{}">{}</td>
</tr>
"#,
            row_style,
            CELL_STYLE,
            i + 1,
            CELL_STYLE,
            run.final_value,
            CELL_STYLE,
            run.final_gradient_norm,
            CELL_STYLE,
            run.iterations,
            CELL_STYLE,
            run.function_evaluations,
            CELL_STYLE,
            run.gradient_evaluations,
            CELL_STYLE,
            run.execution_time.as_secs_f64(),
            CELL_STYLE,
            if run.convergence_achieved {
                "Yes"
            } else {
                "No"
            },
            CELL_STYLE,
            convergence_reason
        ));
    }
    content.push_str("</tbody>\n</table>\n\n");
    
    Ok(content)
}
/// Formats the convergence reason into a human-readable abbreviated form
fn format_convergence_reason(reason: &impl std::fmt::Debug) -> String {
    format!("{:?}", reason)
        .replace("GradientTolerance", "Grad Tol")
        .replace("FunctionTolerance", "Func Tol")
        .replace("MaxIterations", "Max Iter")
        .replace("MaxFunctionEvaluations", "Max Func")
        .replace("TimeLimit", "Time")
        .replace("NumericalError", "Num Err")
        .replace('"', "")
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    #[test]
    fn test_format_convergence_reason() {
        #[derive(Debug)]
        enum TestReason {
            GradientTolerance,
            MaxIterations,
        }
        assert_eq!(
            format_convergence_reason(&TestReason::GradientTolerance),
            "Grad Tol"
        );
        assert_eq!(
            format_convergence_reason(&TestReason::MaxIterations),
            "Max Iter"
        );
    }
    #[test]
    fn test_generate_run_by_run_analysis_empty() {
        let runs: Vec<&SingleResult> = vec![];
        let result = generate_run_by_run_analysis(&runs).unwrap();
        assert!(result.contains("## Run-by-Run Analysis"));
        assert!(result.contains("<table"));
        assert!(result.contains("</table>"));
    }
}