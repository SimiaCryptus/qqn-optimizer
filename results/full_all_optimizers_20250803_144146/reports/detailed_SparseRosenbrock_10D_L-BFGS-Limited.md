# Detailed Analysis: L-BFGS-Limited on SparseRosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_10D
**Optimizer:** L-BFGS-Limited
**Problem Family:** SparseRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.168393e-1
* **Worst Final Value:** 1.507348e1
* **Mean Final Value:** 1.275584e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.808e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.887e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">998</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.042e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.361e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3990</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.273e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">336</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3992</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.397e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.579e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3994</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.990e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.196e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.168e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.025e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3996</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.145e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.854e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3986</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.941e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.160e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.507e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.421e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">303</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">914</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.601e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.464e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3996</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.846e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.292e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3990</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.351e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.546e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3988</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.070e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.522e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">998</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.680e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.025e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">338</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3989</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.488e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.002e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.871e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.199e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.374e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.528e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.044e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.341e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">340</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.595e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.745e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">339</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3993</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.067e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.547e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">338</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 6)
**Final Value:** 3.168393e-1
**Final Gradient Norm:** 2.025470e0
**Iterations:** 334
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.207e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.835e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.171e0, 8.809e-1, -1.281e0, 8.520e-1, -1.369e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.207e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.835e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.461e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.182e-1, 9.870e-1, -8.393e-1, 1.023e0, -8.688e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.784e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.461e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.804e-1, 9.504e-1, -9.703e-1, 9.418e-1, -1.068e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.068e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.035e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.699e-1, 9.532e-1, -9.654e-1, 9.417e-1, -1.040e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.030e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.189e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.714e-1, 9.490e-1, -9.651e-1, 9.384e-1, -1.053e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">330</td><td style="border: 1px solid #ddd; padding: 4px;">3.246e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.914e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.478e-1, 5.586e-1, 7.455e-1, 5.558e-1, 7.478e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">331</td><td style="border: 1px solid #ddd; padding: 4px;">3.221e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.603e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.505e-1, 5.601e-1, 7.496e-1, 5.563e-1, 7.501e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">332</td><td style="border: 1px solid #ddd; padding: 4px;">3.210e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.068e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.494e-1, 5.610e-1, 7.472e-1, 5.582e-1, 7.494e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">333</td><td style="border: 1px solid #ddd; padding: 4px;">3.179e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.078e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.521e-1, 5.624e-1, 7.511e-1, 5.589e-1, 7.517e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">334</td><td style="border: 1px solid #ddd; padding: 4px;">3.168e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.025e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.521e-1, 5.624e-1, 7.511e-1, 5.589e-1, 7.517e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3999.6
- **Average Gradient Evaluations per Run:** 1009.0
- **Average Iterations per Run:** 334.6
- **Average Time per Run:** 0.050s
- **Function Evaluations per Second:** 80010.7
- **Iterations per Second:** 6694.6
### Resource Utilization
- **Total Function Evaluations:** 79992
- **Total Gradient Evaluations:** 20179
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 3.96
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/SparseRosenbrock_10D_results.csv)
* [Convergence Plot](../plots/SparseRosenbrock_10D.png)
* [Log Scale Convergence Plot](../plots/SparseRosenbrock_10D_log.png)


---
*Detailed report for L-BFGS-Limited on SparseRosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
