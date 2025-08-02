# Detailed Analysis: GD-WeightDecay on Levy_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_10D
**Optimizer:** GD-WeightDecay
**Problem Family:** Levy
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.927103e-7
* **Worst Final Value:** 9.998997e-7
* **Mean Final Value:** 9.965763e-7
* **Success Rate:** 100.0%


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
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.942e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.728e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1467</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2931</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.998e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.740e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.992e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.739e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1441</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.941e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.728e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.954e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.731e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.949e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.730e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.971e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.734e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.999e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.740e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.990e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.738e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2915</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.973e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.735e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.975e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.735e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.961e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.732e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1446</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.984e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.737e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.947e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.729e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2861</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.934e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.726e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.927e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.725e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1437</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.994e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.739e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.972e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.735e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.927e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.725e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1477</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2957</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.985e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.737e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1438</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1441</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2879</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 1448.7
* **Average Function Evaluations:** 1451.7
* **Average Time to Convergence:** 0.052s
* **Fastest Convergence:** 1443 iterations (0.050s)
* **Slowest Convergence:** 1459 iterations (0.053s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 19)
**Final Value:** 9.927103e-7
**Final Gradient Norm:** 5.724777e-4
**Iterations:** 1477
**Convergence Reason:** FunctionTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.061e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.617e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.023e0, 2.151e0, 1.993e0, 2.072e0, 2.049e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.061e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.617e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.013e0, 2.146e0, 1.987e0, 2.067e0, 2.044e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.996e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.624e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.996e0, 2.136e0, 1.977e0, 2.057e0, 2.034e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.877e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.636e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.973e0, 2.124e0, 1.963e0, 2.043e0, 2.020e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.716e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.647e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.945e0, 2.108e0, 1.947e0, 2.027e0, 2.004e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1473</td><td style="border: 1px solid #ddd; padding: 4px;">1.022e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.789e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 9.999e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1474</td><td style="border: 1px solid #ddd; padding: 4px;">1.015e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.773e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 9.999e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1475</td><td style="border: 1px solid #ddd; padding: 4px;">1.007e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.757e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 9.999e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1476</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.741e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 9.999e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1477</td><td style="border: 1px solid #ddd; padding: 4px;">9.927e-7</td><td style="border: 1px solid #ddd; padding: 4px;">5.725e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 9.999e-1, 9.999e-1, 9.999e-1, 9.999e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1451.7
- **Average Gradient Evaluations per Run:** 2900.3
- **Average Iterations per Run:** 1448.7
- **Average Time per Run:** 0.052s
- **Function Evaluations per Second:** 28115.3
- **Iterations per Second:** 28057.2
### Resource Utilization
- **Total Function Evaluations:** 29033
- **Total Gradient Evaluations:** 58006
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Levy_10D_results.csv)
* [Convergence Plot](../plots/Levy_10D.png)
* [Log Scale Convergence Plot](../plots/Levy_10D_log.png)


---
*Detailed report for GD-WeightDecay on Levy_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
