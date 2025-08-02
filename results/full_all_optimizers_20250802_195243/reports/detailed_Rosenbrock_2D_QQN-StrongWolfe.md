# Detailed Analysis: QQN-StrongWolfe on Rosenbrock_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rosenbrock_2D
**Optimizer:** QQN-StrongWolfe
**Problem Family:** Rosenbrock
**Dimension:** 2
**Success Threshold:** 8.450e-3
**Total Runs:** 20
**Successful Runs:** 7 (35.0%)

### Quick Statistics
* **Best Final Value:** 4.577618e-3
* **Worst Final Value:** 3.112737e-1
* **Mean Final Value:** 5.665541e-2
* **Success Rate:** 35.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.994e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.749e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">503</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">380</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.215e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.992e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2747</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.578e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.778e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.053e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.220e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.607e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.434e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2217</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.411e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.257e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">814</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">625</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.906e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.973e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2899</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.360e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.062e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2832</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.087e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.693e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2093</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.078e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.543e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2873</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2129</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.124e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.487e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">266</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.601e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.908e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2920</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.084</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.175e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.703e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.063e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.007e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2963</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.113e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.228e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2751</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.419e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.913e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2856</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.894e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.702e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2791</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.082</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.205e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.598e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2872</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.083</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.864e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.730e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.396e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.402e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">632</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">461</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (7 out of 20)

* **Average Iterations to Convergence:** 24.0
* **Average Function Evaluations:** 424.6
* **Average Time to Convergence:** 0.012s
* **Fastest Convergence:** 14 iterations (0.004s)
* **Slowest Convergence:** 34 iterations (0.023s)

### Failed Runs (13 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 13 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 4.577618e-3
**Final Gradient Norm:** 4.778240e-1
**Iterations:** 28
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.863e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.821e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.348e0, 1.161e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.863e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.821e2</td><td style="border: 1px solid #ddd; padding: 4px;">7.812e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.455e0, 2.187e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.955e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.215e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.494e0, 2.174e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.838e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.767e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.476e0, 2.180e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.269e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.628e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.476e0, 2.180e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">24</td><td style="border: 1px solid #ddd; padding: 4px;">2.558e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.005e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.116e0, 1.244e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">25</td><td style="border: 1px solid #ddd; padding: 4px;">1.383e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.054e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.766e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.115e0, 1.245e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">26</td><td style="border: 1px solid #ddd; padding: 4px;">1.337e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.007e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.016e0, 1.021e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">27</td><td style="border: 1px solid #ddd; padding: 4px;">1.032e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.574e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.066e0, 1.139e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">28</td><td style="border: 1px solid #ddd; padding: 4px;">4.578e-3</td><td style="border: 1px solid #ddd; padding: 4px;">4.778e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.066e0, 1.139e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2004.3
- **Average Gradient Evaluations per Run:** 1524.5
- **Average Iterations per Run:** 81.3
- **Average Time per Run:** 0.058s
- **Function Evaluations per Second:** 34520.0
- **Iterations per Second:** 1401.1
### Resource Utilization
- **Total Function Evaluations:** 40087
- **Total Gradient Evaluations:** 30491
- **Total Computation Time:** 1.2s
- **Function/Gradient Ratio:** 1.31
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Rosenbrock_2D_results.csv)
* [Convergence Plot](../plots/Rosenbrock_2D.png)
* [Log Scale Convergence Plot](../plots/Rosenbrock_2D_log.png)


---
*Detailed report for QQN-StrongWolfe on Rosenbrock_2D*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
