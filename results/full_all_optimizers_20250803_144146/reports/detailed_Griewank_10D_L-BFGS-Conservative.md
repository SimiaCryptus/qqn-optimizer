# Detailed Analysis: L-BFGS-Conservative on Griewank_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_10D
**Optimizer:** L-BFGS-Conservative
**Problem Family:** Griewank
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.014431e1
* **Worst Final Value:** 2.494112e1
* **Mean Final Value:** 2.330695e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.167e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">227</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">910</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.494e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.807e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">663</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">530</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.199e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.359e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">552</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2790</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.105e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.345e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2788</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.360e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.501e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2787</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.652e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.494e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.655e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">638</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">510</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.716e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.561e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1724</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.494e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.888e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.133e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1368</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.494e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.723e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">498</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.494e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.898e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">608</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.426e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.047e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">199</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">798</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.015e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.884e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2784</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.222e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.243e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">554</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2786</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2221</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.975e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">409</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1638</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.146e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.069e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">459</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2318</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.520e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1650</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1306</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.014e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.318e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1915</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 5 runs
- FunctionTolerance: 15 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 2.014431e1
**Final Gradient Norm:** 1.317555e-4
**Iterations:** 478
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.599e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 9.990e1, 1.001e2, 1.002e2, 9.980e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.599e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 9.990e1, 1.001e2, 1.002e2, 9.980e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.599e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e2, 9.990e1, 1.001e2, 1.002e2, 9.979e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.599e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.379e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[9.990e1, 9.988e1, 1.000e2, 1.001e2, 9.970e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.595e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.355e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[9.981e1, 9.986e1, 9.996e1, 9.998e1, 9.961e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">473</td><td style="border: 1px solid #ddd; padding: 4px;">2.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.106e1, 8.876e1, 9.236e1, 8.778e1, 9.108e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">474</td><td style="border: 1px solid #ddd; padding: 4px;">2.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.456e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.106e1, 8.876e1, 9.236e1, 8.778e1, 9.108e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">475</td><td style="border: 1px solid #ddd; padding: 4px;">2.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.417e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.106e1, 8.876e1, 9.236e1, 8.778e1, 9.108e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">476</td><td style="border: 1px solid #ddd; padding: 4px;">2.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.381e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.106e1, 8.876e1, 9.236e1, 8.778e1, 9.108e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">477</td><td style="border: 1px solid #ddd; padding: 4px;">2.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.348e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.106e1, 8.876e1, 9.236e1, 8.778e1, 9.108e1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1668.3
- **Average Gradient Evaluations per Run:** 1326.4
- **Average Iterations per Run:** 330.8
- **Average Time per Run:** 0.048s
- **Function Evaluations per Second:** 34739.8
- **Iterations per Second:** 6888.4
### Resource Utilization
- **Total Function Evaluations:** 33366
- **Total Gradient Evaluations:** 26528
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.26
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Griewank_10D_results.csv)
* [Convergence Plot](../plots/Griewank_10D.png)
* [Log Scale Convergence Plot](../plots/Griewank_10D_log.png)


---
*Detailed report for L-BFGS-Conservative on Griewank_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
