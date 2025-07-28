# Detailed Analysis: Adam-WeightDecay on Zakharov_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_10D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Zakharov
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 8.382791e2
* **Worst Final Value:** 5.247266e3
* **Mean Final Value:** 2.819160e3
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.726e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.864e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.786e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.498e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.247e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.411e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.383e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.060e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.374e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.730e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.385e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.106e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.452e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.169e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.247e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.057e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.920e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.936e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.199e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.039e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.638e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.437e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.027e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.061e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.532e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.546e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.570e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.805e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.752e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.056e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.755e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.058e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.125e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.564e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.573e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.411e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.803e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.435e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.753e4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 8.382791e2
**Final Gradient Norm:** 6.060145e3
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.490e5</td><td style="border: 1px solid #ddd; padding: 4px;">6.804e5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.155e0, 9.265e-1, 9.799e-1, 8.388e-1, 1.052e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.490e5</td><td style="border: 1px solid #ddd; padding: 4px;">6.804e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.152e0, 9.235e-1, 9.769e-1, 8.358e-1, 1.049e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.433e5</td><td style="border: 1px solid #ddd; padding: 4px;">6.740e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.149e0, 9.205e-1, 9.739e-1, 8.328e-1, 1.046e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.377e5</td><td style="border: 1px solid #ddd; padding: 4px;">6.675e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.146e0, 9.175e-1, 9.709e-1, 8.298e-1, 1.043e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.321e5</td><td style="border: 1px solid #ddd; padding: 4px;">6.611e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.143e0, 9.145e-1, 9.679e-1, 8.268e-1, 1.040e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">245</td><td style="border: 1px solid #ddd; padding: 4px;">1.062e3</td><td style="border: 1px solid #ddd; padding: 4px;">7.242e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.164e-1, 1.885e-1, 2.419e-1, 1.008e-1, 3.144e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">246</td><td style="border: 1px solid #ddd; padding: 4px;">1.002e3</td><td style="border: 1px solid #ddd; padding: 4px;">6.933e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.134e-1, 1.855e-1, 2.389e-1, 9.784e-2, 3.114e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">247</td><td style="border: 1px solid #ddd; padding: 4px;">9.450e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.633e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.104e-1, 1.825e-1, 2.359e-1, 9.484e-2, 3.084e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">248</td><td style="border: 1px solid #ddd; padding: 4px;">8.904e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.342e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.074e-1, 1.795e-1, 2.329e-1, 9.184e-2, 3.054e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">249</td><td style="border: 1px solid #ddd; padding: 4px;">8.383e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.060e3</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.074e-1, 1.795e-1, 2.329e-1, 9.184e-2, 3.054e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 502.0
- **Average Gradient Evaluations per Run:** 502.0
- **Average Iterations per Run:** 249.0
- **Average Time per Run:** 0.012s
- **Function Evaluations per Second:** 42796.6
- **Iterations per Second:** 21227.8
### Resource Utilization
- **Total Function Evaluations:** 10040
- **Total Gradient Evaluations:** 10040
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Zakharov_10D_results.csv)
* [Convergence Plot](convergence_Zakharov_10D.png)
* [Log Scale Convergence Plot](convergence_Zakharov_10D_log.png)


---
*Detailed report for Adam-WeightDecay on Zakharov_10D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
