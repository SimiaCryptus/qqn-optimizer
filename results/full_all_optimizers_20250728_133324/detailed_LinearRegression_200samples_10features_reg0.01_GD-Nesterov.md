# Detailed Analysis: GD-Nesterov on LinearRegression_200samples_10features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LinearRegression_200samples_10features_reg0.01
**Optimizer:** GD-Nesterov
**Problem Family:** ML Regression
**Dimension:** 10
**Success Threshold:** 4.820e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.778880e0
* **Worst Final Value:** 1.820965e0
* **Mean Final Value:** 1.802852e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.779e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.592e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.816e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.644e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.788e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.608e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.791e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.623e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.807e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.650e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.796e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.622e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.812e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.626e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.818e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.652e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.794e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.624e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.804e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.632e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.819e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.654e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.786e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.610e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.816e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.647e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.794e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.623e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.810e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.641e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.814e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.643e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.784e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.600e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.821e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.682e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.035</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.814e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.661e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.036</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.794e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.654e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 1.778880e0
**Final Gradient Norm:** 2.591926e0
**Iterations:** 35
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.112e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.382e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.205e-2, 1.650e-1, 1.534e-1, -1.947e-1, -3.367e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.112e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.382e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[7.660e-2, 1.683e-1, 1.523e-1, -1.859e-1, -2.540e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.100e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.368e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.890e-2, 1.774e-1, 1.496e-1, -1.621e-1, -2.960e-3, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.068e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.332e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.044e-1, 1.890e-1, 1.463e-1, -1.319e-1, 2.562e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.028e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.285e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.229e-1, 2.029e-1, 1.427e-1, -9.584e-2, 5.982e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">30</td><td style="border: 1px solid #ddd; padding: 4px;">1.398e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.318e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[6.211e-1, 1.059e0, 1.282e0, 1.957e0, 2.385e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">31</td><td style="border: 1px solid #ddd; padding: 4px;">1.522e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.469e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[6.050e-1, 1.079e0, 1.378e0, 2.015e0, 2.470e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">32</td><td style="border: 1px solid #ddd; padding: 4px;">1.640e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.579e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.875e-1, 1.095e0, 1.468e0, 2.066e0, 2.547e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">33</td><td style="border: 1px solid #ddd; padding: 4px;">1.730e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.635e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.690e-1, 1.106e0, 1.551e0, 2.109e0, 2.615e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">34</td><td style="border: 1px solid #ddd; padding: 4px;">1.777e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.638e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.501e-1, 1.113e0, 1.625e0, 2.146e0, 2.674e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 38.0
- **Average Gradient Evaluations per Run:** 71.9
- **Average Iterations per Run:** 35.0
- **Average Time per Run:** 0.035s
- **Function Evaluations per Second:** 1086.5
- **Iterations per Second:** 1000.6
### Resource Utilization
- **Total Function Evaluations:** 759
- **Total Gradient Evaluations:** 1438
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 0.53
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/LinearRegression_200samples_10features_reg0.01_results.csv)
* [Convergence Plot](convergence_LinearRegression_200samples_10features_reg0.01.png)
* [Log Scale Convergence Plot](convergence_LinearRegression_200samples_10features_reg0.01_log.png)


---
*Detailed report for GD-Nesterov on LinearRegression_200samples_10features_reg0.01*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
