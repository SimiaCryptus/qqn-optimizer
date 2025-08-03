# Detailed Analysis: L-BFGS-Aggressive on Levi_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Levi_2D
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Levi
**Dimension:** 2
**Success Threshold:** 2.840e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.002537e0
* **Worst Final Value:** 4.169531e0
* **Mean Final Value:** 2.867935e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.983e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.952e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.161e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.046e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.577e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.569e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.130e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.901e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.901e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.376e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.003e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.859e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3848</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.170e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.550e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.567e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.558e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.046e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.453e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.126e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.358e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3847</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.925e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.036e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.639e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.129e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.774e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.009e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.707e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.575e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.487e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.010e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.584e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.631e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.979e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.094e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.729e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.437e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.791e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.992e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.817e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 6)
**Final Value:** 1.002537e0
**Final Gradient Norm:** 4.859056e0
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.145e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.698e-1, 2.527e-3]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.145e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.862e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.862e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.862e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">380</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">381</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">382</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">383</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">384</td><td style="border: 1px solid #ddd; padding: 4px;">1.003e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.859e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.620e0, 1.185e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3851.3
- **Average Gradient Evaluations per Run:** 1157.0
- **Average Iterations per Run:** 384.0
- **Average Time per Run:** 0.022s
- **Function Evaluations per Second:** 178805.7
- **Iterations per Second:** 17827.9
### Resource Utilization
- **Total Function Evaluations:** 77027
- **Total Gradient Evaluations:** 23140
- **Total Computation Time:** 0.4s
- **Function/Gradient Ratio:** 3.33
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Levi_2D_results.csv)
* [Convergence Plot](../plots/Levi_2D.png)
* [Log Scale Convergence Plot](../plots/Levi_2D_log.png)


---
*Detailed report for L-BFGS-Aggressive on Levi_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
