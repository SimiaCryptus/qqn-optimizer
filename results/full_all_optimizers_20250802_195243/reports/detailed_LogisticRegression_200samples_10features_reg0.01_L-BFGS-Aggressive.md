# Detailed Analysis: L-BFGS-Aggressive on LogisticRegression_200samples_10features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LogisticRegression_200samples_10features_reg0.01
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Regression
**Dimension:** 10
**Success Threshold:** 3.230e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.233035e-1
* **Worst Final Value:** 3.257765e-1
* **Mean Final Value:** 3.245637e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.243e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.426e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.233e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.333e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.250e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.270e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.088e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1433</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.817e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.533</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.256e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.675e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">443</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3675</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.234e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.741e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">511</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.171e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.166e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.224e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1391</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.570</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.840e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.258e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.767e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.255e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.685e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.247e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.234e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.114e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.538</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.234e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.547e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.253e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.450e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.256e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.673e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.552</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.084e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.246e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.156e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3845</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 2 runs
- MaxFunctionEvaluations: 18 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 3.233035e-1
**Final Gradient Norm:** 2.332773e-3
**Iterations:** 27
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.086e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.267e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.248e-2, -8.775e-2, 1.266e-1, -1.199e-1, 8.403e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.086e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.267e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.300e-3, -3.111e-2, 1.436e-1, -6.052e-2, 1.527e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.147e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.880e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.737e-2, 1.614e-2, 1.617e-1, -1.969e-3, 2.101e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.498e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.578e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.790e-1, 4.328e-1, 5.306e-1, 9.874e-1, 7.250e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.297e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.794e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.740e-1, 5.584e-1, 6.325e-1, 5.733e-1, 9.763e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">22</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.333e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 5.081e-1, 6.482e-1, 8.074e-1, 9.145e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">23</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.333e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 5.081e-1, 6.482e-1, 8.074e-1, 9.145e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">24</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.333e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 5.081e-1, 6.482e-1, 8.074e-1, 9.145e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">25</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.333e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 5.081e-1, 6.482e-1, 8.074e-1, 9.145e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">26</td><td style="border: 1px solid #ddd; padding: 4px;">3.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.333e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.779e-1, 5.081e-1, 6.482e-1, 8.074e-1, 9.145e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3432.2
- **Average Gradient Evaluations per Run:** 1106.6
- **Average Iterations per Run:** 367.3
- **Average Time per Run:** 1.407s
- **Function Evaluations per Second:** 2439.5
- **Iterations per Second:** 261.1
### Resource Utilization
- **Total Function Evaluations:** 68645
- **Total Gradient Evaluations:** 22132
- **Total Computation Time:** 28.1s
- **Function/Gradient Ratio:** 3.10
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
*Detailed report for L-BFGS-Aggressive on LogisticRegression_200samples_10features_reg0.01*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
