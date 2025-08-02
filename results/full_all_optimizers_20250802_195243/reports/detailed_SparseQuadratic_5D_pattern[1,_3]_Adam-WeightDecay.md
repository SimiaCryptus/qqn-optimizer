# Detailed Analysis: Adam-WeightDecay on SparseQuadratic_5D_pattern[1, 3]
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseQuadratic_5D_pattern[1, 3]
**Optimizer:** Adam-WeightDecay
**Problem Family:** SparseQuadratic
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 9.729033e-7
* **Worst Final Value:** 9.987285e-7
* **Mean Final Value:** 9.851603e-7
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.783e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.012e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">880</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1763</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1763</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.882e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.010e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.729e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.009e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1753</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1753</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.821e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.964e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.805e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.936e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">848</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.984e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.036e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1793</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.946e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.965e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.936e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.967e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.758e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.952e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">848</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1699</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.772e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.999e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">872</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1747</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1747</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.910e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.940e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.936e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.035e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1629</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.987e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.038e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">825</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1653</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1653</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.864e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.968e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">870</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1743</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.768e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.005e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">843</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.902e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.019e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">843</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1689</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.895e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.005e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1737</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1737</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.784e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.025e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">833</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1669</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.799e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.938e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">846</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1695</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1695</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.772e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.994e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 862.4
* **Average Function Evaluations:** 1727.8
* **Average Time to Convergence:** 0.038s
* **Fastest Convergence:** 807 iterations (0.035s)
* **Slowest Convergence:** 945 iterations (0.041s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 9.729033e-7
**Final Gradient Norm:** 2.008648e-3
**Iterations:** 875
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.157e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.251e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.176e0, 1.145e0, 8.955e-1, 9.591e-1, 1.043e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.157e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.251e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.173e0, 1.142e0, 8.925e-1, 9.561e-1, 1.040e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.122e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.236e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.170e0, 1.139e0, 8.895e-1, 9.531e-1, 1.037e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.087e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.221e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.167e0, 1.136e0, 8.865e-1, 9.501e-1, 1.034e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.052e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.206e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.164e0, 1.133e0, 8.835e-1, 9.471e-1, 1.031e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">871</td><td style="border: 1px solid #ddd; padding: 4px;">1.089e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.125e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.232e-4, 5.714e-4, -6.892e-5, -1.219e-4, 1.564e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">872</td><td style="border: 1px solid #ddd; padding: 4px;">1.059e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.095e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.119e-4, 5.631e-4, -6.792e-5, -1.204e-4, 1.491e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">873</td><td style="border: 1px solid #ddd; padding: 4px;">1.029e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.066e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.008e-4, 5.550e-4, -6.694e-5, -1.190e-4, 1.420e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">874</td><td style="border: 1px solid #ddd; padding: 4px;">1.001e-6</td><td style="border: 1px solid #ddd; padding: 4px;">2.037e-3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.898e-4, 5.469e-4, -6.596e-5, -1.175e-4, 1.351e-5]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">875</td><td style="border: 1px solid #ddd; padding: 4px;">9.729e-7</td><td style="border: 1px solid #ddd; padding: 4px;">2.009e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.898e-4, 5.469e-4, -6.596e-5, -1.175e-4, 1.351e-5]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1727.8
- **Average Gradient Evaluations per Run:** 1727.8
- **Average Iterations per Run:** 862.4
- **Average Time per Run:** 0.038s
- **Function Evaluations per Second:** 45574.7
- **Iterations per Second:** 22747.8
### Resource Utilization
- **Total Function Evaluations:** 34556
- **Total Gradient Evaluations:** 34556
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/SparseQuadratic_5D_pattern[1,_3]_results.csv)
* [Convergence Plot](../plots/SparseQuadratic_5D_pattern[1,_3].png)
* [Log Scale Convergence Plot](../plots/SparseQuadratic_5D_pattern[1,_3]_log.png)


---
*Detailed report for Adam-WeightDecay on SparseQuadratic_5D_pattern[1, 3]*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
