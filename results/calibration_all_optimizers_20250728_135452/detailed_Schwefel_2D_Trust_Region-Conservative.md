# Detailed Analysis: Trust Region-Conservative on Schwefel_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Schwefel_2D
**Optimizer:** Trust Region-Conservative
**Problem Family:** Schwefel
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 9.431421e2
* **Worst Final Value:** 9.459229e2
* **Mean Final Value:** 9.445516e2
* **Success Rate:** 0.0%


## Run-by-Run Analysis
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
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.446e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.726e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.443e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.729e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.431e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.741e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.445e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.726e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.452e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.719e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.441e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.450e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.721e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.438e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.734e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.443e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.729e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.445e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.726e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.441e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.438e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.734e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.452e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.720e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.446e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.726e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.441e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.459e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.712e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.453e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.719e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.452e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.719e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.441e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.454e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.717e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 9.431421e2
**Final Gradient Norm:** 6.740606e0
**Iterations:** 199
**Convergence Reason:** MaxFunctionEvaluations

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.451e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.720e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.981e1, 9.984e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.451e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.720e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.981e1, 9.984e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.451e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.720e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.981e1, 9.984e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.451e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.720e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.981e1, 9.984e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.451e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.720e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.488e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.981e1, 9.984e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">195</td><td style="border: 1px solid #ddd; padding: 4px;">9.432e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.740e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.484e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.961e1, 9.964e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">196</td><td style="border: 1px solid #ddd; padding: 4px;">9.432e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.740e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.484e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.961e1, 9.963e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">197</td><td style="border: 1px solid #ddd; padding: 4px;">9.432e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.740e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.484e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.961e1, 9.963e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">198</td><td style="border: 1px solid #ddd; padding: 4px;">9.432e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.741e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.484e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.960e1, 9.963e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">199</td><td style="border: 1px solid #ddd; padding: 4px;">9.431e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.741e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.484e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.960e1, 9.963e1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 602.0
- **Average Gradient Evaluations per Run:** 402.0
- **Average Iterations per Run:** 199.0
- **Average Time per Run:** 0.004s
- **Function Evaluations per Second:** 170313.3
- **Iterations per Second:** 56299.6
### Resource Utilization
- **Total Function Evaluations:** 12040
- **Total Gradient Evaluations:** 8040
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Schwefel_2D_results.csv)
* [Convergence Plot](convergence_Schwefel_2D.png)
* [Log Scale Convergence Plot](convergence_Schwefel_2D_log.png)


---
*Detailed report for Trust Region-Conservative on Schwefel_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
