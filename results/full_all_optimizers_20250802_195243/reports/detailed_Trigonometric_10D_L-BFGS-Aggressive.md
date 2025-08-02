# Detailed Analysis: L-BFGS-Aggressive on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.423422e0
* **Worst Final Value:** 1.745578e1
* **Mean Final Value:** 1.000600e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.333e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.427e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.364e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.295e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.552e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.342e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.477e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.354e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.055e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.298e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.746e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.285e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.021e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.101e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.067e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.701e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.099e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.365e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.879e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.420e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.664e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.612e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.438e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.292e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.573e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.464e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.168e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.226e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.125e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.887e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.423e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.248e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.665e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.504e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.754e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.996e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.463e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.257e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.186e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.484e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 16)
**Final Value:** 2.423422e0
**Final Gradient Norm:** 2.248467e1
**Iterations:** 384
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.426e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.153e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.426e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.153e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.426e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.153e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.426e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.153e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.426e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.250e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.153e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">380</td><td style="border: 1px solid #ddd; padding: 4px;">2.423e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.248e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.152e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">381</td><td style="border: 1px solid #ddd; padding: 4px;">2.423e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.248e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.152e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">382</td><td style="border: 1px solid #ddd; padding: 4px;">2.423e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.248e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.152e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">383</td><td style="border: 1px solid #ddd; padding: 4px;">2.423e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.248e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.152e-2, 4.119e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">384</td><td style="border: 1px solid #ddd; padding: 4px;">2.423e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.248e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[3.309e-1, 7.533e-2, 6.687e-2, 8.152e-2, 4.119e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3852.0
- **Average Gradient Evaluations per Run:** 1157.0
- **Average Iterations per Run:** 384.0
- **Average Time per Run:** 0.032s
- **Function Evaluations per Second:** 119652.1
- **Iterations per Second:** 11927.9
### Resource Utilization
- **Total Function Evaluations:** 77040
- **Total Gradient Evaluations:** 23140
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 3.33
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Trigonometric_10D_results.csv)
* [Convergence Plot](../plots/Trigonometric_10D.png)
* [Log Scale Convergence Plot](../plots/Trigonometric_10D_log.png)


---
*Detailed report for L-BFGS-Aggressive on Trigonometric_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
