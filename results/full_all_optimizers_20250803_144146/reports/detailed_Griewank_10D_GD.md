# Detailed Analysis: GD on Griewank_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_10D
**Optimizer:** GD
**Problem Family:** Griewank
**Dimension:** 10
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.554372e1
* **Worst Final Value:** 2.565663e1
* **Mean Final Value:** 2.561050e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.559e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.584e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.562e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.593e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.562e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.587e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.564e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.574e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.565e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.558e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.560e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.567e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.560e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.563e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.562e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.617e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.561e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.613e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.561e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.607e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.557e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.576e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.561e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.600e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.563e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.607e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.562e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.597e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.566e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.416e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.558e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.580e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.554e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.553e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.561e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.590e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.564e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.597e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.558e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.572e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 17)
**Final Value:** 2.554372e1
**Final Gradient Norm:** 1.552695e-1
**Iterations:** 1665
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.596e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.511e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.989e1, 9.982e1, 1.000e2, 9.990e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.596e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.511e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.989e1, 9.982e1, 1.000e2, 9.990e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.596e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.511e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.989e1, 9.982e1, 1.000e2, 9.990e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.596e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.511e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.989e1, 9.982e1, 1.000e2, 9.990e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.596e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.511e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.989e1, 9.982e1, 1.000e2, 9.990e1, 1.001e2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1661</td><td style="border: 1px solid #ddd; padding: 4px;">2.554e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.554e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.896e1, 9.922e1, 9.925e1, 9.906e1, 9.929e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1662</td><td style="border: 1px solid #ddd; padding: 4px;">2.554e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.553e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.896e1, 9.922e1, 9.925e1, 9.906e1, 9.929e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1663</td><td style="border: 1px solid #ddd; padding: 4px;">2.554e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.553e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.896e1, 9.922e1, 9.925e1, 9.906e1, 9.929e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1664</td><td style="border: 1px solid #ddd; padding: 4px;">2.554e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.553e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.896e1, 9.922e1, 9.925e1, 9.906e1, 9.929e1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1665</td><td style="border: 1px solid #ddd; padding: 4px;">2.554e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.553e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.896e1, 9.922e1, 9.925e1, 9.906e1, 9.929e1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1668.0
- **Average Gradient Evaluations per Run:** 3334.0
- **Average Iterations per Run:** 1665.0
- **Average Time per Run:** 0.047s
- **Function Evaluations per Second:** 35426.3
- **Iterations per Second:** 35362.6
### Resource Utilization
- **Total Function Evaluations:** 33360
- **Total Gradient Evaluations:** 66680
- **Total Computation Time:** 0.9s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Griewank_10D_results.csv)
* [Convergence Plot](../plots/Griewank_10D.png)
* [Log Scale Convergence Plot](../plots/Griewank_10D_log.png)


---
*Detailed report for GD on Griewank_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
