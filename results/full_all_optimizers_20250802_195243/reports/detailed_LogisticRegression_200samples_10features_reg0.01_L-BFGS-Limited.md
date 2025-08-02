# Detailed Analysis: L-BFGS-Limited on LogisticRegression_200samples_10features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LogisticRegression_200samples_10features_reg0.01
**Optimizer:** L-BFGS-Limited
**Problem Family:** Regression
**Dimension:** 10
**Success Threshold:** 3.230e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.232540e-1
* **Worst Final Value:** 3.236464e-1
* **Mean Final Value:** 3.232877e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.236e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.242e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">513</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3458</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1544</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.586</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.767e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.614</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.020e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.618</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.842e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.621</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.727e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.625</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.501e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.620</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.615</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.224e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.625</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.036e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.261e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.620</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.859e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.630</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.149e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.352e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.627</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.371e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.296e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.639</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.517e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.628</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.451e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.626</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.944e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.091e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">624</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.924e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">623</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.616</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 3.232540e-1
**Final Gradient Norm:** 2.727207e-4
**Iterations:** 624
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.937e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.211e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.087e-1, 2.943e-2, 8.734e-2, -1.061e-1, -3.886e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.937e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.211e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.088e-1, 4.039e-2, 9.226e-2, -9.141e-2, -1.552e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.698e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.115e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.093e-1, 5.083e-2, 9.726e-2, -7.678e-2, 6.793e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.479e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.025e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.850e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.730e-1, 1.286e-1, 1.933e-1, 1.591e-1, 1.884e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.994e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.337e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.469e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[2.932e-1, 2.032e-1, 3.372e-1, 4.749e-1, 3.773e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">620</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.757e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.477e-1, 5.050e-1, 6.535e-1, 7.860e-1, 9.028e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">621</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.750e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.477e-1, 5.050e-1, 6.535e-1, 7.859e-1, 9.028e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">622</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.742e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.477e-1, 5.050e-1, 6.535e-1, 7.859e-1, 9.029e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">623</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.735e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.477e-1, 5.050e-1, 6.535e-1, 7.859e-1, 9.029e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">624</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.727e-4</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.477e-1, 5.050e-1, 6.535e-1, 7.859e-1, 9.029e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3146.0
- **Average Gradient Evaluations per Run:** 1859.6
- **Average Iterations per Run:** 618.2
- **Average Time per Run:** 1.621s
- **Function Evaluations per Second:** 1940.3
- **Iterations per Second:** 381.3
### Resource Utilization
- **Total Function Evaluations:** 62920
- **Total Gradient Evaluations:** 37192
- **Total Computation Time:** 32.4s
- **Function/Gradient Ratio:** 1.69
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/LogisticRegression_200samples_10features_reg0.01_results.csv)
* [Convergence Plot](../plots/LogisticRegression_200samples_10features_reg0.01.png)
* [Log Scale Convergence Plot](../plots/LogisticRegression_200samples_10features_reg0.01_log.png)


---
*Detailed report for L-BFGS-Limited on LogisticRegression_200samples_10features_reg0.01*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
