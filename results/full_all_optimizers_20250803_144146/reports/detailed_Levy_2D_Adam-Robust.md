# Detailed Analysis: Adam-Robust on Levy_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levy_2D
**Optimizer:** Adam-Robust
**Problem Family:** Levy
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.827884e-2
* **Worst Final Value:** 9.343171e-2
* **Mean Final Value:** 4.336151e-2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.280e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.599e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.012e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.801e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.343e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.643e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.391e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.229e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.789e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.656e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.737e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.160e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.951e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.343e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.575e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.953e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.482e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.999e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.898e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.059</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.124e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.689e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.600e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.007e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.828e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.751e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.949e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.320e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.969e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.548e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.859e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.142e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.137e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.610e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.840e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.260e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.294e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.839e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.115e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.758e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** 1.827884e-2
**Final Gradient Norm:** 2.751169e-1
**Iterations:** 1249
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.837e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.875e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.856e0, 1.884e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.837e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.875e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.900e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.846e0, 1.874e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.627e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.867e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.801e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.836e0, 1.865e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.420e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.703e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.827e0, 1.855e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.216e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.850e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.606e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.817e0, 1.845e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1245</td><td style="border: 1px solid #ddd; padding: 4px;">1.828e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.751e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[1.115e0, 1.204e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1246</td><td style="border: 1px solid #ddd; padding: 4px;">1.828e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.751e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[1.115e0, 1.204e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1247</td><td style="border: 1px solid #ddd; padding: 4px;">1.828e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.751e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[1.115e0, 1.204e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1248</td><td style="border: 1px solid #ddd; padding: 4px;">1.828e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.751e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[1.115e0, 1.204e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1249</td><td style="border: 1px solid #ddd; padding: 4px;">1.828e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.751e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[1.115e0, 1.204e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2502.0
- **Average Gradient Evaluations per Run:** 2502.0
- **Average Iterations per Run:** 1249.0
- **Average Time per Run:** 0.058s
- **Function Evaluations per Second:** 43266.7
- **Iterations per Second:** 21598.8
### Resource Utilization
- **Total Function Evaluations:** 50040
- **Total Gradient Evaluations:** 50040
- **Total Computation Time:** 1.2s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Levy_2D_results.csv)
* [Convergence Plot](../plots/Levy_2D.png)
* [Log Scale Convergence Plot](../plots/Levy_2D_log.png)


---
*Detailed report for Adam-Robust on Levy_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
