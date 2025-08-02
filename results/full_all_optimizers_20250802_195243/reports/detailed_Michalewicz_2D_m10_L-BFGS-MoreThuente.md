# Detailed Analysis: L-BFGS-MoreThuente on Michalewicz_2D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_2D_m10
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** Michalewicz
**Dimension:** 2
**Success Threshold:** -9.960e-1
**Total Runs:** 20
**Successful Runs:** 5 (25.0%)

### Quick Statistics
* **Best Final Value:** -9.999868e-1
* **Worst Final Value:** -4.085898e-13
* **Mean Final Value:** -2.495220e-1
* **Success Rate:** 25.0%


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
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-9.974e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.466e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1649</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-9.999e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.283e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.511e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.626e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.000e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.625e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">971</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.734e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.193e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.690e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.698e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.086e-13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.701e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-9.964e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.614e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">368</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2583</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1847</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.051e-12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.235e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.963e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.120e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.623e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.019e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.545e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.724e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.089e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.152e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-9.967e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.222e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.305e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.143e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.173e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.679e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.876e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.630e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.028e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.263e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">416</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2921</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.084e-12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e-10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.674e-11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.529e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (5 out of 20)

* **Average Iterations to Convergence:** 177.0
* **Average Function Evaluations:** 1245.0
* **Average Time to Convergence:** 0.021s
* **Fastest Convergence:** 18 iterations (0.002s)
* **Slowest Convergence:** 368 iterations (0.040s)

### Failed Runs (15 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 3 runs
- GradientTolerance: 12 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** -9.999868e-1
**Final Gradient Norm:** 4.624730e-2
**Iterations:** 193
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-9.443e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.817e-5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 9.125e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-9.443e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.817e-5</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 9.127e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-9.516e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.845e-5</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 9.129e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-9.590e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.874e-5</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 9.131e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-9.666e-7</td><td style="border: 1px solid #ddd; padding: 4px;">3.904e-5</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 9.133e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">189</td><td style="border: 1px solid #ddd; padding: 4px;">-3.392e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.037e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 1.088e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">190</td><td style="border: 1px solid #ddd; padding: 4px;">-4.538e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.363e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 1.456e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">191</td><td style="border: 1px solid #ddd; padding: 4px;">-6.088e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.128e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.090e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 1.560e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">192</td><td style="border: 1px solid #ddd; padding: 4px;">-9.952e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.685e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.198e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 1.570e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">193</td><td style="border: 1px solid #ddd; padding: 4px;">-1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.625e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.464e-1, 1.570e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 752.4
- **Average Gradient Evaluations per Run:** 538.8
- **Average Iterations per Run:** 107.2
- **Average Time per Run:** 0.010s
- **Function Evaluations per Second:** 71901.3
- **Iterations per Second:** 10249.1
### Resource Utilization
- **Total Function Evaluations:** 15048
- **Total Gradient Evaluations:** 10776
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.40
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Michalewicz_2D_m10_results.csv)
* [Convergence Plot](../plots/Michalewicz_2D_m10.png)
* [Log Scale Convergence Plot](../plots/Michalewicz_2D_m10_log.png)


---
*Detailed report for L-BFGS-MoreThuente on Michalewicz_2D_m10*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
