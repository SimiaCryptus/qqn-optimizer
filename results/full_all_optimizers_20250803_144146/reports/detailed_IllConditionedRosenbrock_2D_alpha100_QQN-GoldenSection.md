# Detailed Analysis: QQN-GoldenSection on IllConditionedRosenbrock_2D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_2D_alpha100
**Optimizer:** QQN-GoldenSection
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.992054e-5
* **Worst Final Value:** 1.931237e-1
* **Mean Final Value:** 1.254169e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.259e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.085e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4508</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.783e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.193e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4508</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.642e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.894e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4428</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.496e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.387e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4432</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">577</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.728e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.693e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.066e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.499e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.623e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">95</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4427</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">577</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.903e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.200e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.155e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.735e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4460</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.674e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.810e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4508</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.446e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.531e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.487e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.600e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4471</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.783e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.788e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.640e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.379e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.578e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.827e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4463</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.531e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.644e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.931e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.147e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4476</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.961e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4429</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.601e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.795e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4439</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.992e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.122e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 6.992054e-5
**Final Gradient Norm:** 1.121857e-2
**Iterations:** 94
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.362e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.844e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.366e0, 1.042e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.362e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.844e2</td><td style="border: 1px solid #ddd; padding: 4px;">6.540e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.068e0, 1.150e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.285e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.761e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.857e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.015e0, 1.003e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.137e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.622e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.470e-1, 8.638e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.899e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.765e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-7.041e-1, 4.192e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">90</td><td style="border: 1px solid #ddd; padding: 4px;">7.039e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.126e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.841e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.916e-1, 9.833e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">91</td><td style="border: 1px solid #ddd; padding: 4px;">7.027e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.016e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.255e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.916e-1, 9.833e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">92</td><td style="border: 1px solid #ddd; padding: 4px;">7.015e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.124e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.841e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.916e-1, 9.833e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">93</td><td style="border: 1px solid #ddd; padding: 4px;">7.004e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.015e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.255e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.916e-1, 9.833e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">94</td><td style="border: 1px solid #ddd; padding: 4px;">6.992e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.122e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.841e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.916e-1, 9.833e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 4459.4
- **Average Gradient Evaluations per Run:** 570.7
- **Average Iterations per Run:** 94.0
- **Average Time per Run:** 0.083s
- **Function Evaluations per Second:** 53591.1
- **Iterations per Second:** 1129.0
### Resource Utilization
- **Total Function Evaluations:** 89189
- **Total Gradient Evaluations:** 11414
- **Total Computation Time:** 1.7s
- **Function/Gradient Ratio:** 7.81
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/IllConditionedRosenbrock_2D_alpha100_results.csv)
* [Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100.png)
* [Log Scale Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100_log.png)


---
*Detailed report for QQN-GoldenSection on IllConditionedRosenbrock_2D_alpha100*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
