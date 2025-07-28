# Detailed Analysis: Adam on Matyas_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Matyas_2D
**Optimizer:** Adam
**Problem Family:** Convex Unimodal
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.867869e-2
* **Worst Final Value:** 3.881198e-2
* **Mean Final Value:** 2.768732e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.827e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.441e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.954e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.451e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.199e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.334e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.656e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.241e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.438e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.542e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.370e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.802e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.881e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.656e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.822e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.323e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.332e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.188e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.903e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.914e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.364e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.130e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.294e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.722e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.376e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.466e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.457e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.437e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.762e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.163e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.357e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.079e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.691e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.419e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.851e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.034e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.972e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.988e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.868e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.082e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 1.867869e-2
**Final Gradient Norm:** 4.082030e-2
**Iterations:** 249
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.362e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.425e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.036e-1, 9.265e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.362e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.425e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.026e-1, 9.255e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.355e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.419e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.016e-1, 9.245e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.348e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.414e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.006e-1, 9.235e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.340e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.409e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.996e-1, 9.225e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">245</td><td style="border: 1px solid #ddd; padding: 4px;">1.887e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.103e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.748e-1, 6.938e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">246</td><td style="border: 1px solid #ddd; padding: 4px;">1.882e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.097e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.740e-1, 6.929e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">247</td><td style="border: 1px solid #ddd; padding: 4px;">1.877e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.092e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.731e-1, 6.921e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">248</td><td style="border: 1px solid #ddd; padding: 4px;">1.873e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.087e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.722e-1, 6.912e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">249</td><td style="border: 1px solid #ddd; padding: 4px;">1.868e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.082e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.722e-1, 6.912e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 502.0
- **Average Gradient Evaluations per Run:** 502.0
- **Average Iterations per Run:** 249.0
- **Average Time per Run:** 0.009s
- **Function Evaluations per Second:** 53599.6
- **Iterations per Second:** 26586.3
### Resource Utilization
- **Total Function Evaluations:** 10040
- **Total Gradient Evaluations:** 10040
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Matyas_2D_results.csv)
* [Convergence Plot](convergence_Matyas_2D.png)
* [Log Scale Convergence Plot](convergence_Matyas_2D_log.png)


---
*Detailed report for Adam on Matyas_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
