# Detailed Analysis: Trust Region-Adaptive on Schwefel_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Schwefel_2D
**Optimizer:** Trust Region-Adaptive
**Problem Family:** Schwefel
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 7.109538e2
* **Worst Final Value:** 7.111617e2
* **Mean Final Value:** 7.110640e2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.859e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2831</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.770e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2819</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1880</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.024e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.515e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.112e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.978e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.703e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">948</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2846</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1898</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.289e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.765e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.195e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2819</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1880</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.396e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2849</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1900</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.714e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.529e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">939</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2819</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1880</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.880e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2843</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1896</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.112e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.959e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2843</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1896</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.618e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.350e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2831</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.121e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">942</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2828</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.027e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">951</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2855</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.110e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.985e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">947</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2843</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1896</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.111e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.503e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">945</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2837</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 6)
**Final Value:** 7.109538e2
**Final Gradient Norm:** 3.702971e-1
**Iterations:** 948
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.474e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.696e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.998e1, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.474e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.696e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.734e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.996e1, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.471e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.699e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.732e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.993e1, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.469e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.702e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.730e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.991e1, 1.001e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.466e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.704e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.729e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.988e1, 1.000e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">943</td><td style="border: 1px solid #ddd; padding: 4px;">7.119e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.900e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.165e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.742e1, 6.743e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">944</td><td style="border: 1px solid #ddd; padding: 4px;">7.116e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.058e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.542e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.717e1, 6.718e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">945</td><td style="border: 1px solid #ddd; padding: 4px;">7.114e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.115e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.088e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.688e1, 6.689e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">946</td><td style="border: 1px solid #ddd; padding: 4px;">7.112e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.027e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.974e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.653e1, 6.654e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">947</td><td style="border: 1px solid #ddd; padding: 4px;">7.110e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.703e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[6.653e1, 6.654e1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2834.8
- **Average Gradient Evaluations per Run:** 1890.5
- **Average Iterations per Run:** 944.2
- **Average Time per Run:** 0.018s
- **Function Evaluations per Second:** 160727.5
- **Iterations per Second:** 53538.0
### Resource Utilization
- **Total Function Evaluations:** 56695
- **Total Gradient Evaluations:** 37810
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Schwefel_2D_results.csv)
* [Convergence Plot](../plots/Schwefel_2D.png)
* [Log Scale Convergence Plot](../plots/Schwefel_2D_log.png)


---
*Detailed report for Trust Region-Adaptive on Schwefel_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
