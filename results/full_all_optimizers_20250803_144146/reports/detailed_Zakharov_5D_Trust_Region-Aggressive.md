# Detailed Analysis: Trust Region-Aggressive on Zakharov_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_5D
**Optimizer:** Trust Region-Aggressive
**Problem Family:** Zakharov
**Dimension:** 5
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.016179e-1
* **Worst Final Value:** 4.253601e2
* **Mean Final Value:** 5.654297e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.496e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.262e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.097e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.002e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.767e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.476e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1949</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1300</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.647e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.177e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">671</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.158e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.354e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2687</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1792</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.222e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.358e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.634e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.179e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">742</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2228</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.016e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.621e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">707</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.238e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.087e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2303</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.970e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.782e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">616</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1850</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1234</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.166e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.360e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.043e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.435e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">764</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2294</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.254e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.369e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.905e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.114e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">843</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2531</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1688</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.091e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.699e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">873</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1748</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.056e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.850e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">585</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1757</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.186e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.781e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">783</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2351</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.632e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.314e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.898e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.411e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.404e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.692e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">659</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1979</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 15 runs
- MaxFunctionEvaluations: 5 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 8)
**Final Value:** 6.016179e-1
**Final Gradient Norm:** 2.620695e0
**Iterations:** 707
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.698e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.147e-1, 9.415e-1, 9.120e-1, 8.441e-1, 1.136e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.812e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.698e3</td><td style="border: 1px solid #ddd; padding: 4px;">7.020e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[8.146e-1, 9.413e-1, 9.118e-1, 8.437e-1, 1.135e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.808e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.692e3</td><td style="border: 1px solid #ddd; padding: 4px;">7.028e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[8.145e-1, 9.411e-1, 9.115e-1, 8.433e-1, 1.135e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.804e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.686e3</td><td style="border: 1px solid #ddd; padding: 4px;">7.035e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[8.144e-1, 9.409e-1, 9.112e-1, 8.429e-1, 1.134e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.800e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.680e3</td><td style="border: 1px solid #ddd; padding: 4px;">7.043e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[8.143e-1, 9.407e-1, 9.109e-1, 8.426e-1, 1.134e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">702</td><td style="border: 1px solid #ddd; padding: 4px;">1.204e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.653e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.623e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[6.034e-1, 5.233e-1, 2.878e-1, 1.406e-2, 9.789e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">703</td><td style="border: 1px solid #ddd; padding: 4px;">8.543e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.526e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.129e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.941e-1, 5.060e-1, 2.628e-1, -1.860e-2, 5.692e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">704</td><td style="border: 1px solid #ddd; padding: 4px;">5.254e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.315e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.271e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.793e-1, 4.792e-1, 2.248e-1, -6.768e-2, -4.774e-3]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">705</td><td style="border: 1px solid #ddd; padding: 4px;">2.384e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.075e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.928e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.433e-1, 4.197e-1, 1.448e-1, -1.675e-1, -1.311e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">706</td><td style="border: 1px solid #ddd; padding: 4px;">6.016e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.621e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.433e-1, 4.197e-1, 1.448e-1, -1.675e-1, -1.311e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2415.8
- **Average Gradient Evaluations per Run:** 1611.2
- **Average Iterations per Run:** 804.4
- **Average Time per Run:** 0.017s
- **Function Evaluations per Second:** 141395.8
- **Iterations per Second:** 47078.3
### Resource Utilization
- **Total Function Evaluations:** 48316
- **Total Gradient Evaluations:** 32224
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Zakharov_5D_results.csv)
* [Convergence Plot](../plots/Zakharov_5D.png)
* [Log Scale Convergence Plot](../plots/Zakharov_5D_log.png)


---
*Detailed report for Trust Region-Aggressive on Zakharov_5D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
