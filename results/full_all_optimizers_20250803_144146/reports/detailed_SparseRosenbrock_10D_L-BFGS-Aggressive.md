# Detailed Analysis: L-BFGS-Aggressive on SparseRosenbrock_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SparseRosenbrock_10D
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** SparseRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 8.086768e1
* **Worst Final Value:** 4.011532e2
* **Mean Final Value:** 1.807551e2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.326e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.069e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.469e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.017e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.756e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.826e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.137e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.932e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.589e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.415e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.759e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.959e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.914e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.473e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.609e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.221e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.200e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.051e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.516e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.536e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.298e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.066e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.243e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.698e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.163e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.304e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.288e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.591e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.773e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.731e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.012e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.131e3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.744e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.767e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.892e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.318e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.404e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.459e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.087e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.119e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3852</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
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
**Final Value:** 8.086768e1
**Final Gradient Norm:** 4.119473e2
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.185e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.157e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.163e0, 9.157e-1, -1.317e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">8.185e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.157e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.163e0, 9.157e-1, -1.317e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.185e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.157e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.163e0, 9.157e-1, -1.317e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">8.184e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.157e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.163e0, 9.157e-1, -1.317e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">8.184e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.157e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.163e0, 9.157e-1, -1.317e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">380</td><td style="border: 1px solid #ddd; padding: 4px;">8.088e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.161e0, 9.162e-1, -1.315e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">381</td><td style="border: 1px solid #ddd; padding: 4px;">8.088e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.161e0, 9.162e-1, -1.315e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">382</td><td style="border: 1px solid #ddd; padding: 4px;">8.087e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.161e0, 9.162e-1, -1.315e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">383</td><td style="border: 1px solid #ddd; padding: 4px;">8.087e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.161e0, 9.162e-1, -1.315e0, 1.184e0, -1.060e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">384</td><td style="border: 1px solid #ddd; padding: 4px;">8.087e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.119e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.161e0, 9.162e-1, -1.315e0, 1.184e0, -1.060e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 3852.0
- **Average Gradient Evaluations per Run:** 1157.0
- **Average Iterations per Run:** 384.0
- **Average Time per Run:** 0.028s
- **Function Evaluations per Second:** 137597.0
- **Iterations per Second:** 13716.8
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
* [Raw CSV Data](../data/problems/SparseRosenbrock_10D_results.csv)
* [Convergence Plot](../plots/SparseRosenbrock_10D.png)
* [Log Scale Convergence Plot](../plots/SparseRosenbrock_10D_log.png)


---
*Detailed report for L-BFGS-Aggressive on SparseRosenbrock_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
