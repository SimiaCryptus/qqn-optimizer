# Detailed Analysis: Adam-AMSGrad on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** Adam-AMSGrad
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.349633e-2
* **Worst Final Value:** 1.951702e0
* **Mean Final Value:** 6.015407e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.778e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.361e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.502e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.724e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.681e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.846e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.394e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.049e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.083e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.653e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.198e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.440e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.430e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.046e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.876e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.351e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.179e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.301e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.350e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.049e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.445e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.256e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.017e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.686e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.220e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.593e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.399e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.452e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.278e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.357e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.105e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.825e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.402e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.935e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.153e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.679e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.064e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.051e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.952e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.435e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 2.349633e-2
**Final Gradient Norm:** 2.049029e-1
**Iterations:** 249
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.472e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.183e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.739e-2, 1.323e-2, 3.064e-2, 2.555e-1, 3.209e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.472e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.183e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.839e-2, 1.423e-2, 3.164e-2, 2.545e-1, 3.109e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.411e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.175e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.939e-2, 1.523e-2, 3.264e-2, 2.535e-1, 3.009e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.351e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.167e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.039e-2, 1.623e-2, 3.364e-2, 2.525e-1, 2.910e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.291e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.159e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.139e-2, 1.723e-2, 3.463e-2, 2.515e-1, 2.811e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">245</td><td style="border: 1px solid #ddd; padding: 4px;">2.436e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.017e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.942e-1, 1.047e-2, 6.500e-3, 5.333e-3, 3.824e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">246</td><td style="border: 1px solid #ddd; padding: 4px;">2.415e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.025e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.930e-1, 1.033e-2, 6.421e-3, 5.310e-3, 3.777e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">247</td><td style="border: 1px solid #ddd; padding: 4px;">2.393e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.034e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.919e-1, 1.021e-2, 6.342e-3, 5.287e-3, 3.730e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">248</td><td style="border: 1px solid #ddd; padding: 4px;">2.371e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.042e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.907e-1, 1.008e-2, 6.263e-3, 5.262e-3, 3.684e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">249</td><td style="border: 1px solid #ddd; padding: 4px;">2.350e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.049e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.907e-1, 1.008e-2, 6.263e-3, 5.262e-3, 3.684e-3, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 502.0
- **Average Gradient Evaluations per Run:** 502.0
- **Average Iterations per Run:** 249.0
- **Average Time per Run:** 0.015s
- **Function Evaluations per Second:** 33946.7
- **Iterations per Second:** 16838.1
### Resource Utilization
- **Total Function Evaluations:** 10040
- **Total Gradient Evaluations:** 10040
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Trigonometric_10D_results.csv)
* [Convergence Plot](convergence_Trigonometric_10D.png)
* [Log Scale Convergence Plot](convergence_Trigonometric_10D_log.png)


---
*Detailed report for Adam-AMSGrad on Trigonometric_10D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
