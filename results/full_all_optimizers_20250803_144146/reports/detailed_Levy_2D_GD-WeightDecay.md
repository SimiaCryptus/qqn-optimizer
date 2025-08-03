# Detailed Analysis: GD-WeightDecay on Levy_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_2D
**Optimizer:** GD-WeightDecay
**Problem Family:** Levy
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.925582e-7
* **Worst Final Value:** 9.999434e-7
* **Mean Final Value:** 9.964724e-7
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.957e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.083e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.996e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.093e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1423</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.967e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.086e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.984e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.090e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.926e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.076e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.958e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.083e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.976e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.088e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.966e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.086e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.978e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.088e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2899</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.944e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.080e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1427</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.970e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.087e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1469</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1472</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2941</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.934e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.078e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.999e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.094e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1424</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1427</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.961e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.084e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.955e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.083e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.960e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.084e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.968e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.086e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.941e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.079e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.961e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.084e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1475</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.994e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.092e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2903</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 1450.5
* **Average Function Evaluations:** 1453.5
* **Average Time to Convergence:** 0.046s
* **Fastest Convergence:** 1424 iterations (0.045s)
* **Slowest Convergence:** 1480 iterations (0.048s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 9.925582e-7
**Final Gradient Norm:** 5.075536e-4
**Iterations:** 1473
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.137e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.901e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.905e0, 2.137e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.137e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.901e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.896e0, 2.136e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.119e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.897e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.879e0, 2.134e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.087e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.888e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.856e0, 2.132e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.043e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.873e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.828e0, 2.128e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1469</td><td style="border: 1px solid #ddd; padding: 4px;">1.023e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.150e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1470</td><td style="border: 1px solid #ddd; padding: 4px;">1.015e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.132e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1471</td><td style="border: 1px solid #ddd; padding: 4px;">1.008e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.113e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1472</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-6</td><td style="border: 1px solid #ddd; padding: 4px;">5.094e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1473</td><td style="border: 1px solid #ddd; padding: 4px;">9.926e-7</td><td style="border: 1px solid #ddd; padding: 4px;">5.076e-4</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.004e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1453.5
- **Average Gradient Evaluations per Run:** 2904.1
- **Average Iterations per Run:** 1450.5
- **Average Time per Run:** 0.046s
- **Function Evaluations per Second:** 31570.5
- **Iterations per Second:** 31505.3
### Resource Utilization
- **Total Function Evaluations:** 29071
- **Total Gradient Evaluations:** 58082
- **Total Computation Time:** 0.9s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Levy_2D_results.csv)
* [Convergence Plot](../plots/Levy_2D.png)
* [Log Scale Convergence Plot](../plots/Levy_2D_log.png)


---
*Detailed report for GD-WeightDecay on Levy_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
