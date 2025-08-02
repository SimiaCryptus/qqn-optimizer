# Detailed Analysis: Adam-WeightDecay on Booth_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Booth_2D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Booth
**Dimension:** 2
**Success Threshold:** 1.200e-1
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 1.188247e-1
* **Worst Final Value:** 1.199293e-1
* **Mean Final Value:** 1.193404e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.194e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.132e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">980</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.189e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.041e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.190e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.098e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">898</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1799</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1799</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.194e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.102e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1901</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.193e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.065e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1821</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.196e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.135e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">975</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.196e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.085e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.190e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.052e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">959</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.193e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.114e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.192e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.046e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">897</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1797</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1797</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.199e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.073e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1789</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.188e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.079e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">931</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1865</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1865</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.199e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.115e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">923</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.197e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.092e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">982</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.189e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.093e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.199e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.093e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.199e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.064e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.188e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.067e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1817</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.189e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.072e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">994</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.193e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.092e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">902</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1807</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 941.6
* **Average Function Evaluations:** 1886.3
* **Average Time to Convergence:** 0.039s
* **Fastest Convergence:** 902 iterations (0.036s)
* **Slowest Convergence:** 1000 iterations (0.042s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 1.188247e-1
**Final Gradient Norm:** 7.066629e-1
**Iterations:** 907
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.844e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.904e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.215e-2, 1.205e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.844e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.904e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.515e-2, 1.235e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.823e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.897e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[3.815e-2, 1.265e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.802e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.889e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.115e-2, 1.295e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.781e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.881e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.415e-2, 1.325e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">903</td><td style="border: 1px solid #ddd; padding: 4px;">1.237e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.210e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.240e0, 2.747e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">904</td><td style="border: 1px solid #ddd; padding: 4px;">1.225e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.174e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.239e0, 2.748e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">905</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.138e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.238e0, 2.749e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">906</td><td style="border: 1px solid #ddd; padding: 4px;">1.200e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.102e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.237e0, 2.751e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">907</td><td style="border: 1px solid #ddd; padding: 4px;">1.188e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.067e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.237e0, 2.751e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1886.3
- **Average Gradient Evaluations per Run:** 1886.3
- **Average Iterations per Run:** 941.6
- **Average Time per Run:** 0.039s
- **Function Evaluations per Second:** 48476.3
- **Iterations per Second:** 24199.6
### Resource Utilization
- **Total Function Evaluations:** 37726
- **Total Gradient Evaluations:** 37726
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Booth_2D_results.csv)
* [Convergence Plot](../plots/Booth_2D.png)
* [Log Scale Convergence Plot](../plots/Booth_2D_log.png)


---
*Detailed report for Adam-WeightDecay on Booth_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
