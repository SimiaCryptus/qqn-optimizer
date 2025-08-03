# Detailed Analysis: Trust Region-Adaptive on SparseRosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_10D
**Optimizer:** Trust Region-Adaptive
**Problem Family:** SparseRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.021467e1
* **Worst Final Value:** 1.476411e2
* **Mean Final Value:** 2.923942e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.082e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.125e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1706</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.029e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.051e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1754</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.064e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.136e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">655</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1312</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.175e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.630e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.021e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.947e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2720</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1814</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.043e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.977e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.070e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.160e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">745</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2237</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.127e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.386e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2681</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1788</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.057e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.570e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">958</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2876</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.101e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.841e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.093e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.951e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1346</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">898</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.476e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.941e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.092e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.543e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1722</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.140e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.112e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2576</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1718</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.095e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.654e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">814</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.053e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.294e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">762</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.050e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.503e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.262e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.789e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.127e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.003e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.034e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.498e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">702</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 16 runs
- MaxFunctionEvaluations: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 2.021467e1
**Final Gradient Norm:** 5.946877e0
**Iterations:** 906
**Convergence Reason:** GradientTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.459e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.582e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.067e0, 8.832e-1, -1.397e0, 8.570e-1, -1.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.459e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.582e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.913e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.067e0, 8.832e-1, -1.396e0, 8.570e-1, -1.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.457e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.576e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.915e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.067e0, 8.832e-1, -1.396e0, 8.571e-1, -1.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.454e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.570e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.917e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.067e0, 8.832e-1, -1.396e0, 8.572e-1, -1.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.452e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.564e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.919e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.067e0, 8.833e-1, -1.396e0, 8.573e-1, -1.096e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">901</td><td style="border: 1px solid #ddd; padding: 4px;">2.104e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.117e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.072e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.693e-1, 9.264e-1, -1.025e0, 1.007e0, -1.042e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">902</td><td style="border: 1px solid #ddd; padding: 4px;">2.081e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.484e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.175e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.674e-1, 9.269e-1, -1.020e0, 1.009e0, -1.041e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">903</td><td style="border: 1px solid #ddd; padding: 4px;">2.058e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.746e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.104e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.650e-1, 9.275e-1, -1.015e0, 1.011e0, -1.040e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">904</td><td style="border: 1px solid #ddd; padding: 4px;">2.037e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.831e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.365e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.610e-1, 9.281e-1, -1.006e0, 1.014e0, -1.038e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">905</td><td style="border: 1px solid #ddd; padding: 4px;">2.021e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.947e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.610e-1, 9.281e-1, -1.006e0, 1.014e0, -1.038e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2139.5
- **Average Gradient Evaluations per Run:** 1427.0
- **Average Iterations per Run:** 712.3
- **Average Time per Run:** 0.014s
- **Function Evaluations per Second:** 151948.7
- **Iterations per Second:** 50588.0
### Resource Utilization
- **Total Function Evaluations:** 42790
- **Total Gradient Evaluations:** 28540
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.50
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
*Detailed report for Trust Region-Adaptive on SparseRosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
