# Detailed Analysis: Trust Region-Conservative on LinearRegression_100samples_5features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LinearRegression_100samples_5features_reg0.01
**Optimizer:** Trust Region-Conservative
**Problem Family:** Regression
**Dimension:** 5
**Success Threshold:** 7.150e-2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.255314e0
* **Worst Final Value:** 9.769246e0
* **Mean Final Value:** 7.874616e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.725e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.322e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.186e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.499e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.858e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.387e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.710e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.763e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.414e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.048e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.765</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.228e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.120e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.269e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.489e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.417e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.169e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.372e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.618e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.411e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.739e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.768</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.954e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.450e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.255e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.684e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.773</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.769e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.167e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.768</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.841e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.382e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.743e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.073e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.141e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.509e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.214e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.069e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.767</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.554e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.771e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.819e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.392e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.613e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.845e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">999</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.766</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 12)
**Final Value:** 6.255314e0
**Final Gradient Norm:** 5.683942e0
**Iterations:** 999
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.624e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.292e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.927e-2, 1.421e-1, 1.309e-1, 5.761e-2, 9.588e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.624e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.292e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.076e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.906e-2, 1.423e-1, 1.312e-1, 5.822e-2, 9.665e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.623e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.289e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.077e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.886e-2, 1.426e-1, 1.314e-1, 5.884e-2, 9.743e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.622e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.286e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.077e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.865e-2, 1.428e-1, 1.317e-1, 5.945e-2, 9.820e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.621e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.283e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.077e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.845e-2, 1.431e-1, 1.319e-1, 6.007e-2, 9.898e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">995</td><td style="border: 1px solid #ddd; padding: 4px;">6.295e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.703e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.754e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.459e-1, 4.623e-1, 4.986e-1, 8.167e-1, 1.041e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">996</td><td style="border: 1px solid #ddd; padding: 4px;">6.285e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.698e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.755e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.462e-1, 4.627e-1, 4.992e-1, 8.177e-1, 1.042e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">997</td><td style="border: 1px solid #ddd; padding: 4px;">6.275e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.693e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.756e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.465e-1, 4.631e-1, 4.997e-1, 8.187e-1, 1.043e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">998</td><td style="border: 1px solid #ddd; padding: 4px;">6.265e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.689e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.758e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.468e-1, 4.636e-1, 5.003e-1, 8.197e-1, 1.045e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">999</td><td style="border: 1px solid #ddd; padding: 4px;">6.255e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.684e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.759e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.468e-1, 4.636e-1, 5.003e-1, 8.197e-1, 1.045e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3002.0
- **Average Gradient Evaluations per Run:** 2002.0
- **Average Iterations per Run:** 999.0
- **Average Time per Run:** 0.769s
- **Function Evaluations per Second:** 3905.5
- **Iterations per Second:** 1299.7
### Resource Utilization
- **Total Function Evaluations:** 60040
- **Total Gradient Evaluations:** 40040
- **Total Computation Time:** 15.4s
- **Function/Gradient Ratio:** 1.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/LinearRegression_100samples_5features_reg0.01_results.csv)
* [Convergence Plot](../plots/LinearRegression_100samples_5features_reg0.01.png)
* [Log Scale Convergence Plot](../plots/LinearRegression_100samples_5features_reg0.01_log.png)


---
*Detailed report for Trust Region-Conservative on LinearRegression_100samples_5features_reg0.01*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
