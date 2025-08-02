# Detailed Analysis: Adam-WeightDecay on StyblinskiTang_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** StyblinskiTang_10D
**Optimizer:** Adam-WeightDecay
**Problem Family:** StyblinskiTang
**Dimension:** 10
**Success Threshold:** -3.780e2
**Total Runs:** 20
**Successful Runs:** 7 (35.0%)

### Quick Statistics
* **Best Final Value:** -3.782304e2
* **Worst Final Value:** -3.633793e2
* **Mean Final Value:** -3.734992e2
* **Success Rate:** 35.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.593e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">958</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1919</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.970e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">957</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1916</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.717e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">950</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1903</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1902</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.859e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">941</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.780e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.749e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1721</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.129e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">938</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1879</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.773e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">938</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1879</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.043</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.622e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1926</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.679e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1887</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.176e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">962</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1927</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1926</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.634e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.876e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">952</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1907</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1906</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.735e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.741e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.734e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1705</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.544e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1941</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1940</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.781e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.749e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">834</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1671</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1671</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.484e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1908</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.782e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.732e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">864</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1731</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.775e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.137e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">953</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1909</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1908</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.781e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.723e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1715</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (7 out of 20)

* **Average Iterations to Convergence:** 853.1
* **Average Function Evaluations:** 1709.3
* **Average Time to Convergence:** 0.040s
* **Fastest Convergence:** 856 iterations (0.039s)
* **Slowest Convergence:** 859 iterations (0.041s)

### Failed Runs (13 out of 20)

**Failure Reasons:**
- FunctionTolerance: 13 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** -3.782304e2
**Final Gradient Norm:** 2.734347e1
**Iterations:** 851
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.306e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.020e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.936e-2, -1.154e-1, 7.187e-2, 4.788e-2, 1.550e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.306e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.020e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.236e-2, -1.184e-1, 6.887e-2, 4.488e-2, 1.520e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-1.391e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.034e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.536e-2, -1.214e-1, 6.587e-2, 4.188e-2, 1.492e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-1.477e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.836e-2, -1.244e-1, 6.287e-2, 3.888e-2, 1.464e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-1.565e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.060e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.014e-1, -1.274e-1, 5.986e-2, 3.587e-2, 1.436e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">847</td><td style="border: 1px solid #ddd; padding: 4px;">-3.772e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.826e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.566e0, -2.551e0, -2.659e0, -2.646e0, -2.693e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">848</td><td style="border: 1px solid #ddd; padding: 4px;">-3.775e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.803e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.569e0, -2.555e0, -2.662e0, -2.649e0, -2.695e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">849</td><td style="border: 1px solid #ddd; padding: 4px;">-3.777e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.780e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.572e0, -2.558e0, -2.664e0, -2.652e0, -2.697e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">850</td><td style="border: 1px solid #ddd; padding: 4px;">-3.780e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.757e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.575e0, -2.561e0, -2.667e0, -2.655e0, -2.700e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">851</td><td style="border: 1px solid #ddd; padding: 4px;">-3.782e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.734e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.575e0, -2.561e0, -2.667e0, -2.655e0, -2.700e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1837.7
- **Average Gradient Evaluations per Run:** 1837.0
- **Average Iterations per Run:** 917.4
- **Average Time per Run:** 0.043s
- **Function Evaluations per Second:** 42873.5
- **Iterations per Second:** 21401.8
### Resource Utilization
- **Total Function Evaluations:** 36754
- **Total Gradient Evaluations:** 36741
- **Total Computation Time:** 0.9s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/StyblinskiTang_10D_results.csv)
* [Convergence Plot](../plots/StyblinskiTang_10D.png)
* [Log Scale Convergence Plot](../plots/StyblinskiTang_10D_log.png)


---
*Detailed report for Adam-WeightDecay on StyblinskiTang_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
