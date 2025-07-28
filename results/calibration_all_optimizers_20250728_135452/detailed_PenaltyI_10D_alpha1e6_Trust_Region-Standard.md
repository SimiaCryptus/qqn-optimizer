# Detailed Analysis: Trust Region-Standard on PenaltyI_10D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_10D_alpha1e6
**Optimizer:** Trust Region-Standard
**Problem Family:** PenaltyI
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 4.222119e5
* **Worst Final Value:** 1.117290e6
* **Mean Final Value:** 7.169874e5
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.117e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.114e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.321e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.459e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.471e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.729e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.774e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.763e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.867e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.774e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.665e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.505e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.674e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.863e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.222e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.300e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.800e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.649e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.752e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.517e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.783e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.978e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.299e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.822e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.172e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.438e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.542e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.618e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.080e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.560e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.327e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.932e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.613e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.626e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.732e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.641e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.877e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.884e6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.253e5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.450e6</td>
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

### Best Run Analysis (Run 8)
**Final Value:** 4.222119e5
**Final Gradient Norm:** 1.299549e6
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.224e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.450e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.224e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.693e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.450e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.224e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.693e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.450e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.224e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.693e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.450e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.224e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.693e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.450e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">195</td><td style="border: 1px solid #ddd; padding: 4px;">4.222e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.695e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.449e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">196</td><td style="border: 1px solid #ddd; padding: 4px;">4.222e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.695e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.449e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">197</td><td style="border: 1px solid #ddd; padding: 4px;">4.222e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.695e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.449e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">198</td><td style="border: 1px solid #ddd; padding: 4px;">4.222e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.695e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.449e-1, 4.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">199</td><td style="border: 1px solid #ddd; padding: 4px;">4.222e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e6</td><td style="border: 1px solid #ddd; padding: 4px;">7.695e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[3.176e-1, 3.707e-1, 4.577e-1, 5.449e-1, 4.342e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 602.0
- **Average Gradient Evaluations per Run:** 402.0
- **Average Iterations per Run:** 199.0
- **Average Time per Run:** 0.004s
- **Function Evaluations per Second:** 149983.7
- **Iterations per Second:** 49579.3
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
* [Raw CSV Data](problems/PenaltyI_10D_alpha1e6_results.csv)
* [Convergence Plot](convergence_PenaltyI_10D_alpha1e6.png)
* [Log Scale Convergence Plot](convergence_PenaltyI_10D_alpha1e6_log.png)


---
*Detailed report for Trust Region-Standard on PenaltyI_10D_alpha1e6*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
