# Detailed Analysis: GD-Nesterov on LogisticRegression_200samples_10features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LogisticRegression_200samples_10features_reg0.01
**Optimizer:** GD-Nesterov
**Problem Family:** ML Regression
**Dimension:** 10
**Success Threshold:** 3.230e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.449438e-1
* **Worst Final Value:** 3.488797e-1
* **Mean Final Value:** 3.472611e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.478e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.827e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.455e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.626e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.471e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.767e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.474e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.802e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.465e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.714e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.476e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.824e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.462e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.687e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.476e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.462e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.689e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.449e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.573e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.482e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.869e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.484e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.885e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.477e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.820e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.489e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.926e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.465e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.721e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.475e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.486e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.900e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.466e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.725e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.331</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.477e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.831e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.482e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.879e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.330</td>
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
**Final Value:** 3.449438e-1
**Final Gradient Norm:** 3.573098e-2
**Iterations:** 332
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.332e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.251e-1, 1.588e-1, -2.291e-3, 1.956e-1, -6.636e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.332e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.996e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.251e-1, 1.589e-1, -2.130e-3, 1.957e-1, -6.584e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.328e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.994e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.251e-1, 1.593e-1, -1.691e-3, 1.957e-1, -6.442e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.317e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.989e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.252e-1, 1.598e-1, -1.136e-3, 1.958e-1, -6.262e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.304e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.983e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.252e-1, 1.603e-1, -4.738e-4, 1.959e-1, -6.048e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">328</td><td style="border: 1px solid #ddd; padding: 4px;">3.455e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.623e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.484e-1, 3.630e-1, 3.763e-1, 5.160e-1, 6.095e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">329</td><td style="border: 1px solid #ddd; padding: 4px;">3.453e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.610e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.487e-1, 3.633e-1, 3.770e-1, 5.167e-1, 6.103e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">330</td><td style="border: 1px solid #ddd; padding: 4px;">3.452e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.598e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.490e-1, 3.637e-1, 3.778e-1, 5.175e-1, 6.112e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">331</td><td style="border: 1px solid #ddd; padding: 4px;">3.451e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.585e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.492e-1, 3.641e-1, 3.785e-1, 5.182e-1, 6.120e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">332</td><td style="border: 1px solid #ddd; padding: 4px;">3.449e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.573e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.492e-1, 3.641e-1, 3.785e-1, 5.182e-1, 6.120e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 335.0
- **Average Gradient Evaluations per Run:** 668.0
- **Average Iterations per Run:** 332.0
- **Average Time per Run:** 0.330s
- **Function Evaluations per Second:** 1014.1
- **Iterations per Second:** 1005.0
### Resource Utilization
- **Total Function Evaluations:** 6700
- **Total Gradient Evaluations:** 13360
- **Total Computation Time:** 6.6s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/LogisticRegression_200samples_10features_reg0.01_results.csv)
* [Convergence Plot](convergence_LogisticRegression_200samples_10features_reg0.01.png)
* [Log Scale Convergence Plot](convergence_LogisticRegression_200samples_10features_reg0.01_log.png)


---
*Detailed report for GD-Nesterov on LogisticRegression_200samples_10features_reg0.01*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
