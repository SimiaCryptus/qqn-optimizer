# Detailed Analysis: QQN-StrongWolfe on PenaltyI_5D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_5D_alpha1e6
**Optimizer:** QQN-StrongWolfe
**Problem Family:** PenaltyI
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.864307e0
* **Worst Final Value:** 7.736873e0
* **Mean Final Value:** 4.211602e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.864e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.385e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3098</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1914</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.288e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.141e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.932e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.510e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1832</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.796e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.897e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1830</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.705e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.632e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1920</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.621e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.806e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1860</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.566e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.777e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1882</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.941e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.830e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1916</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.734e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.044e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1908</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.570e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.127e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1946</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.833e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.916e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3094</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1932</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.780e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.855e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1902</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.667e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.437e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1898</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.915e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.736e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1854</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.305e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.636e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1836</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.809e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.386e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3093</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1938</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.756e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.404e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1858</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.042e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.488e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.737e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.914e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.369e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.671e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
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
**Final Value:** 2.864307e0
**Final Gradient Norm:** 3.384853e0
**Iterations:** 125
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.739e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.223e6</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.865e-1, 5.388e-1, 5.336e-1, 6.125e-1, 4.951e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.739e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.223e6</td><td style="border: 1px solid #ddd; padding: 4px;">2.441e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.629e1, -1.405e2, -1.379e2, -1.764e2, -1.192e2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.975e4</td><td style="border: 1px solid #ddd; padding: 4px;">5.992e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.750e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.582e1, -3.437e1, -3.373e1, -4.334e1, -2.905e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.609e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.498e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.750e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.571e0, -1.281e1, -1.257e1, -1.632e1, -1.074e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.559e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.851e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.750e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.567e0, -4.396e0, -4.300e0, -5.766e0, -3.585e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">121</td><td style="border: 1px solid #ddd; padding: 4px;">2.865e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.385e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.290e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.481e-1, 2.486e-1, 2.390e-1, 2.299e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">122</td><td style="border: 1px solid #ddd; padding: 4px;">2.865e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.135e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.113e-6</td><td style="border: 1px solid #ddd; padding: 4px;">[2.499e-1, 2.481e-1, 2.486e-1, 2.390e-1, 2.299e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">123</td><td style="border: 1px solid #ddd; padding: 4px;">2.865e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.385e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.580e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.482e-1, 2.487e-1, 2.391e-1, 2.300e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">124</td><td style="border: 1px solid #ddd; padding: 4px;">2.864e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.483e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.225e-6</td><td style="border: 1px solid #ddd; padding: 4px;">[2.497e-1, 2.482e-1, 2.488e-1, 2.391e-1, 2.301e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">125</td><td style="border: 1px solid #ddd; padding: 4px;">2.864e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.385e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.632e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[2.497e-1, 2.482e-1, 2.488e-1, 2.391e-1, 2.301e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3121.1
- **Average Gradient Evaluations per Run:** 1901.5
- **Average Iterations per Run:** 127.2
- **Average Time per Run:** 0.086s
- **Function Evaluations per Second:** 36368.5
- **Iterations per Second:** 1482.2
### Resource Utilization
- **Total Function Evaluations:** 62422
- **Total Gradient Evaluations:** 38030
- **Total Computation Time:** 1.7s
- **Function/Gradient Ratio:** 1.64
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/PenaltyI_5D_alpha1e6_results.csv)
* [Convergence Plot](../plots/PenaltyI_5D_alpha1e6.png)
* [Log Scale Convergence Plot](../plots/PenaltyI_5D_alpha1e6_log.png)


---
*Detailed report for QQN-StrongWolfe on PenaltyI_5D_alpha1e6*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
