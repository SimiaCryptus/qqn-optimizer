# Detailed Analysis: Trust Region-Adaptive on StyblinskiTang_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** StyblinskiTang_10D
**Optimizer:** Trust Region-Adaptive
**Problem Family:** Highly Multimodal
**Dimension:** 10
**Success Threshold:** -3.780e2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -5.283566e1
* **Worst Final Value:** -4.926805e1
* **Mean Final Value:** -5.101248e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.078e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.841e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.134e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.728e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.284e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.919e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.057e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.773e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.227e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.876e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.127e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.786e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.103e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.881e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.057e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.725e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.079e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.813e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.927e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.466e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.996e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.590e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.033e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.621e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.042e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.645e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.031e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.814e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.139e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.850e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.162e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.830e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.997e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.830e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.253e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.977e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.277e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.961e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.023e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.785e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** -5.283566e1
**Final Gradient Norm:** 3.919129e1
**Iterations:** 199
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.942e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.532e-1, 1.949e-1, -1.669e-1, -7.771e-2, 2.908e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.942e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.611e-1, 1.958e-1, -1.751e-1, -8.370e-2, 2.583e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-3.195e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.282e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.951e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.688e-1, 1.968e-1, -1.832e-1, -8.954e-2, 2.265e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-3.448e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.313e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.905e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.763e-1, 1.977e-1, -1.910e-1, -9.524e-2, 1.955e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-3.701e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.343e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.862e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.837e-1, 1.986e-1, -1.987e-1, -1.008e-1, 1.652e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">195</td><td style="border: 1px solid #ddd; padding: 4px;">-5.183e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.889e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.428e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.879e-1, 2.870e-1, -9.301e-1, -6.466e-1, -2.854e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">196</td><td style="border: 1px solid #ddd; padding: 4px;">-5.208e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.897e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.415e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.904e-1, 2.873e-1, -9.327e-1, -6.486e-1, -2.866e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">197</td><td style="border: 1px solid #ddd; padding: 4px;">-5.234e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.904e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.403e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.929e-1, 2.877e-1, -9.353e-1, -6.506e-1, -2.877e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">198</td><td style="border: 1px solid #ddd; padding: 4px;">-5.259e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.912e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.391e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.954e-1, 2.880e-1, -9.379e-1, -6.526e-1, -2.889e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">199</td><td style="border: 1px solid #ddd; padding: 4px;">-5.284e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.919e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.379e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.954e-1, 2.880e-1, -9.379e-1, -6.526e-1, -2.889e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 602.0
- **Average Gradient Evaluations per Run:** 402.0
- **Average Iterations per Run:** 199.0
- **Average Time per Run:** 0.004s
- **Function Evaluations per Second:** 154010.4
- **Iterations per Second:** 50910.4
### Resource Utilization
- **Total Function Evaluations:** 12040
- **Total Gradient Evaluations:** 8040
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/StyblinskiTang_10D_results.csv)
* [Convergence Plot](convergence_StyblinskiTang_10D.png)
* [Log Scale Convergence Plot](convergence_StyblinskiTang_10D_log.png)


---
*Detailed report for Trust Region-Adaptive on StyblinskiTang_10D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
