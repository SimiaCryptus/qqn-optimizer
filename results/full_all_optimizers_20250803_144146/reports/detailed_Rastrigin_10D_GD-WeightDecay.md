# Detailed Analysis: GD-WeightDecay on Rastrigin_10D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rastrigin_10D
**Optimizer:** GD-WeightDecay
**Problem Family:** Rastrigin
**Dimension:** 10
**Success Threshold:** 4.180e1
**Total Runs:** 20
**Successful Runs:** 9 (45.0%)

### Quick Statistics
* **Best Final Value:** 3.601899e1
* **Worst Final Value:** 4.975279e1
* **Mean Final Value:** 4.391543e1
* **Success Rate:** 45.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.975e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.022e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">200</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.096e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.015e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.132e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.429e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.477e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.526e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.676e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.329e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.478e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.995e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.676e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.981e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.111e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.144e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.101e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.074e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.602e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.117e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.477e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.211e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.676e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.981e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">96</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.477e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.277e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.107e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.404e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.080e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.777e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.676e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.759e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">198</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.676e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.719e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">97</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.180e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.648e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.975e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.977e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.180e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.973e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (9 out of 20)

* **Average Iterations to Convergence:** 21.1
* **Average Function Evaluations:** 24.1
* **Average Time to Convergence:** 0.001s
* **Fastest Convergence:** 13 iterations (0.001s)
* **Slowest Convergence:** 39 iterations (0.001s)

### Failed Runs (11 out of 20)

**Failure Reasons:**
- FunctionTolerance: 11 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 10)
**Final Value:** 3.601899e1
**Final Gradient Norm:** 4.117326e1
**Iterations:** 18
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.619e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.420e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.809e0, 2.280e0, 2.472e0, 2.128e0, 1.487e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.619e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.420e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.828e0, 2.257e0, 2.466e0, 2.110e0, 1.484e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.548e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.862e0, 2.214e0, 2.455e0, 2.081e0, 1.478e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.424e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.358e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.906e0, 2.156e0, 2.438e0, 2.044e0, 1.469e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.269e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.245e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.953e0, 2.086e0, 2.413e0, 2.006e0, 1.456e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">14</td><td style="border: 1px solid #ddd; padding: 4px;">5.844e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.052e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.966e0, 2.038e0, 1.986e0, 2.005e0, 9.009e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">15</td><td style="border: 1px solid #ddd; padding: 4px;">5.520e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.670e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.975e0, 2.028e0, 2.031e0, 1.998e0, 9.449e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">16</td><td style="border: 1px solid #ddd; padding: 4px;">4.939e1</td><td style="border: 1px solid #ddd; padding: 4px;">8.041e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.986e0, 2.011e0, 2.058e0, 1.990e0, 9.921e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">17</td><td style="border: 1px solid #ddd; padding: 4px;">4.191e1</td><td style="border: 1px solid #ddd; padding: 4px;">7.103e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.996e0, 1.991e0, 2.060e0, 1.984e0, 1.031e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">18</td><td style="border: 1px solid #ddd; padding: 4px;">3.602e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.117e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.996e0, 1.991e0, 2.060e0, 1.984e0, 1.031e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 61.3
- **Average Gradient Evaluations per Run:** 119.0
- **Average Iterations per Run:** 58.3
- **Average Time per Run:** 0.002s
- **Function Evaluations per Second:** 29911.0
- **Iterations per Second:** 28447.1
### Resource Utilization
- **Total Function Evaluations:** 1226
- **Total Gradient Evaluations:** 2381
- **Total Computation Time:** 0.0s
- **Function/Gradient Ratio:** 0.51
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Rastrigin_10D_results.csv)
* [Convergence Plot](../plots/Rastrigin_10D.png)
* [Log Scale Convergence Plot](../plots/Rastrigin_10D_log.png)


---
*Detailed report for GD-WeightDecay on Rastrigin_10D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
