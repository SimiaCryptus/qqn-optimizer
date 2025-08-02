# Detailed Analysis: L-BFGS-MoreThuente on Trigonometric_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Trigonometric_10D
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** Trigonometric
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 12 (60.0%)

### Quick Statistics
* **Best Final Value:** 4.776242e-7
* **Worst Final Value:** 1.600158e-1
* **Mean Final Value:** 1.342421e-2
* **Success Rate:** 60.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.251e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.681e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.997e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.892e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">748</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.838e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.146e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.776e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.664e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">64</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">470</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">342</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.121e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.022e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2914</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2088</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.071</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.847e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.432e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">414</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2920</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.071</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.626e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.296e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">411</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2914</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.467e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.276e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">411</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2913</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.070</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.975e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.254e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1943</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1389</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.600e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.506e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2915</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2087</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.996e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.585e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">307</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.245e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.173e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1078</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">780</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.520e-5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.887e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.071</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.951e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.776e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">333</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1677</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.955e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.107e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">687</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.772e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.615e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">220</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1626</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.992e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.257e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">412</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.860e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.234e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.356e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.408e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2917</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2089</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.069</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.379e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.424e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">414</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2916</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.072</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (12 out of 20)

* **Average Iterations to Convergence:** 183.2
* **Average Function Evaluations:** 1305.0
* **Average Time to Convergence:** 0.032s
* **Fastest Convergence:** 64 iterations (0.011s)
* **Slowest Convergence:** 333 iterations (0.057s)

### Failed Runs (8 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 8 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 4.776242e-7
**Final Gradient Norm:** 5.664377e-3
**Iterations:** 64
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.022e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.860e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.610e-1, 3.271e-1, 2.941e-1, 1.454e-1, 3.342e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.022e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.860e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.093e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[4.711e-1, 4.173e-1, 3.528e-1, 1.680e-1, 3.081e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">8.696e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.286e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.267e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.722e-1, 4.158e-1, 3.462e-1, 1.615e-1, 2.833e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.669e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.675e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.784e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.827e-1, 4.228e-1, 3.471e-1, 1.587e-1, 2.675e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.413e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.218e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.193e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[4.933e-1, 4.295e-1, 3.471e-1, 1.552e-1, 2.495e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">60</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e-5</td><td style="border: 1px solid #ddd; padding: 4px;">3.630e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.458e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.679e-1, 2.337e-1, 1.481e-1, 1.083e-1, 8.499e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">61</td><td style="border: 1px solid #ddd; padding: 4px;">9.067e-6</td><td style="border: 1px solid #ddd; padding: 4px;">3.009e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.840e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.679e-1, 2.339e-1, 1.480e-1, 1.084e-1, 8.548e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">62</td><td style="border: 1px solid #ddd; padding: 4px;">1.531e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.541e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.494e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.679e-1, 2.339e-1, 1.479e-1, 1.084e-1, 8.554e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">1.434e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.171e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.680e-1, 2.341e-1, 1.479e-1, 1.084e-1, 8.561e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">64</td><td style="border: 1px solid #ddd; padding: 4px;">4.776e-7</td><td style="border: 1px solid #ddd; padding: 4px;">5.664e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.680e-1, 2.341e-1, 1.479e-1, 1.084e-1, 8.561e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1949.3
- **Average Gradient Evaluations per Run:** 1398.7
- **Average Iterations per Run:** 275.0
- **Average Time per Run:** 0.048s
- **Function Evaluations per Second:** 40984.7
- **Iterations per Second:** 5782.0
### Resource Utilization
- **Total Function Evaluations:** 38986
- **Total Gradient Evaluations:** 27973
- **Total Computation Time:** 1.0s
- **Function/Gradient Ratio:** 1.39
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
*Detailed report for L-BFGS-MoreThuente on Trigonometric_10D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
