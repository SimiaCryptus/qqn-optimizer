# Detailed Analysis: L-BFGS-MoreThuente on SparseRosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_10D
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** SparseRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.905648e-4
* **Worst Final Value:** 2.323763e1
* **Mean Final Value:** 7.679098e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.756e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.410e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">368</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2875</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.669e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.414e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.194e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.338e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.019e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.473e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2900</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.119e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.882e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2857</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.006e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.901e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2820</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.659e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.641e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2910</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.115e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.931e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">313</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2815</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2187</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.471e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.441e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">414</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.624e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.444e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">405</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2910</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.324e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.065e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">214</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.021e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.162e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">292</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2794</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2208</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.160e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.734e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.546e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.393e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">400</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.346e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.096e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2956</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.817e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.828e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.597e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.714e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">415</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.906e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.373e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">403</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2910</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.842e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.279e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2969</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2037</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.593e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.580e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2785</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 1 runs
- MaxFunctionEvaluations: 19 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 2.905648e-4
**Final Gradient Norm:** 3.372688e-2
**Iterations:** 403
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.111e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.133e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.146e0, 1.054e0, -1.226e0, 1.033e0, -1.309e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.111e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.133e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.165e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.003e0, 1.114e0, -9.522e-1, 1.142e0, -8.869e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.555e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.069e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.534e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.064e0, 1.081e0, -1.084e0, 1.070e0, -1.101e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.625e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e2</td><td style="border: 1px solid #ddd; padding: 4px;">8.783e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.041e0, 1.090e0, -1.040e0, 1.088e0, -1.041e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.117e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.477e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.756e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.030e0, 1.082e0, -1.030e0, 1.080e0, -1.037e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">399</td><td style="border: 1px solid #ddd; padding: 4px;">2.913e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.378e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.020e0, 1.002e0, 1.004e0, 9.917e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">400</td><td style="border: 1px solid #ddd; padding: 4px;">2.911e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.228e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.134e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.020e0, 1.002e0, 1.004e0, 9.917e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">401</td><td style="border: 1px solid #ddd; padding: 4px;">2.909e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.375e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.020e0, 1.002e0, 1.004e0, 9.917e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">402</td><td style="border: 1px solid #ddd; padding: 4px;">2.907e-4</td><td style="border: 1px solid #ddd; padding: 4px;">2.227e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.134e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.019e0, 1.002e0, 1.004e0, 9.917e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">403</td><td style="border: 1px solid #ddd; padding: 4px;">2.906e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.373e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.086e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.010e0, 1.019e0, 1.002e0, 1.004e0, 9.917e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2749.9
- **Average Gradient Evaluations per Run:** 2027.7
- **Average Iterations per Run:** 360.8
- **Average Time per Run:** 0.051s
- **Function Evaluations per Second:** 54171.8
- **Iterations per Second:** 7106.5
### Resource Utilization
- **Total Function Evaluations:** 54999
- **Total Gradient Evaluations:** 40554
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.36
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
*Detailed report for L-BFGS-MoreThuente on SparseRosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
