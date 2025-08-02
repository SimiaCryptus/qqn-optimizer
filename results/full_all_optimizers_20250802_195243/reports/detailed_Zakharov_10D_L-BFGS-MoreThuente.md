# Detailed Analysis: L-BFGS-MoreThuente on Zakharov_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_10D
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** Zakharov
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 4.865183e-7
* **Worst Final Value:** 4.266494e-6
* **Mean Final Value:** 1.670953e-6
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.865e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.689e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2894</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.266e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.261e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.368e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.279e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.299e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.932e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.342e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.428e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.035e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.581e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">379</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2881</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.609e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.829e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.366e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.152e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.589e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.867e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2880</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.125e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.273e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">390</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.889e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.171e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.155e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.379e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.579e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.968e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2882</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.161e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.551e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.557e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.210e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.937e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.441e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.061</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.647e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.976e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.367e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.478e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">383</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.256e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.624e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">382</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.272e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.800e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2887</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 4.865183e-7
**Final Gradient Norm:** 1.688851e-3
**Iterations:** 387
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.133e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.598e5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.074e0, 1.120e0, 8.058e-1, 9.338e-1, 8.799e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.133e5</td><td style="border: 1px solid #ddd; padding: 4px;">8.598e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.025e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[9.197e-1, 8.117e-1, 3.433e-1, 3.171e-1, 1.090e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.344e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.274e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.074e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.301e-1, 8.348e-1, 3.798e-1, 3.661e-1, 1.709e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.817e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.690e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.701e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.214e-1, 8.230e-1, 3.668e-1, 3.495e-1, 1.516e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.081e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.905e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.741e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[6.039e-1, 5.427e-1, 2.482e-1, 2.400e-1, 1.140e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">383</td><td style="border: 1px solid #ddd; padding: 4px;">5.241e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.753e-3</td><td style="border: 1px solid #ddd; padding: 4px;">6.770e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.574e-4, 4.089e-4, 1.828e-4, 1.745e-4, 7.668e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">384</td><td style="border: 1px solid #ddd; padding: 4px;">5.144e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.713e-3</td><td style="border: 1px solid #ddd; padding: 4px;">4.401e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.526e-4, 4.040e-4, 1.796e-4, 1.709e-4, 7.346e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">385</td><td style="border: 1px solid #ddd; padding: 4px;">5.049e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.721e-3</td><td style="border: 1px solid #ddd; padding: 4px;">6.770e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.490e-4, 4.013e-4, 1.795e-4, 1.713e-4, 7.527e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">386</td><td style="border: 1px solid #ddd; padding: 4px;">4.956e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.681e-3</td><td style="border: 1px solid #ddd; padding: 4px;">4.401e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.443e-4, 3.966e-4, 1.763e-4, 1.677e-4, 7.210e-5, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">387</td><td style="border: 1px solid #ddd; padding: 4px;">4.865e-7</td><td style="border: 1px solid #ddd; padding: 4px;">1.689e-3</td><td style="border: 1px solid #ddd; padding: 4px;">6.771e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.443e-4, 3.966e-4, 1.763e-4, 1.677e-4, 7.210e-5, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2886.3
- **Average Gradient Evaluations per Run:** 2120.1
- **Average Iterations per Run:** 382.1
- **Average Time per Run:** 0.059s
- **Function Evaluations per Second:** 49186.0
- **Iterations per Second:** 6512.2
### Resource Utilization
- **Total Function Evaluations:** 57727
- **Total Gradient Evaluations:** 42401
- **Total Computation Time:** 1.2s
- **Function/Gradient Ratio:** 1.36
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Zakharov_10D_results.csv)
* [Convergence Plot](../plots/Zakharov_10D.png)
* [Log Scale Convergence Plot](../plots/Zakharov_10D_log.png)


---
*Detailed report for L-BFGS-MoreThuente on Zakharov_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
