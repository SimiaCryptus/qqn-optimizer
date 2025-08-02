# Detailed Analysis: GD-WeightDecay on IllConditionedRosenbrock_10D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_10D_alpha100
**Optimizer:** GD-WeightDecay
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.701533e-1
* **Worst Final Value:** 5.245601e0
* **Mean Final Value:** 1.339844e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.209e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.260e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.197e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.622e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.448e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.851e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.246e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.303e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">164</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.986e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.982e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1276</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.271e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.285e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1631</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1634</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3264</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.274e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.702e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.445e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.388e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.497e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.702e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.445e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.702e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.445e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.702e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.445e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.058</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.273e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.972e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.977e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1293</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.211e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.260e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.840e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.974e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.989e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.983e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.044</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.140e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.104e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">312</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.702e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.445e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.240e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.272e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1665</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3334</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 11 runs
- FunctionTolerance: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 12)
**Final Value:** 1.701533e-1
**Final Gradient Norm:** 2.444994e1
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.616e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.712e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.262e0, 9.398e-1, -1.032e0, 9.456e-1, -1.220e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.616e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.712e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.252e0, 9.225e-1, -1.020e0, 9.229e-1, -1.199e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.532e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.636e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.234e0, 8.916e-1, -9.964e-1, 8.824e-1, -1.160e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.391e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.505e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.210e0, 8.499e-1, -9.643e-1, 8.280e-1, -1.107e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.218e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.342e3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.179e0, 7.997e-1, -9.245e-1, 7.631e-1, -1.044e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1661</td><td style="border: 1px solid #ddd; padding: 4px;">1.702e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.445e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.985e-1, 1.003e0, 9.949e-1, 1.005e0, 9.928e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1662</td><td style="border: 1px solid #ddd; padding: 4px;">1.695e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.440e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 9.966e-1, 1.004e0, 9.936e-1, 1.005e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1663</td><td style="border: 1px solid #ddd; padding: 4px;">1.702e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.445e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.985e-1, 1.003e0, 9.949e-1, 1.005e0, 9.928e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1664</td><td style="border: 1px solid #ddd; padding: 4px;">1.695e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.440e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 9.966e-1, 1.004e0, 9.936e-1, 1.005e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1665</td><td style="border: 1px solid #ddd; padding: 4px;">1.702e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.445e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.001e0, 9.966e-1, 1.004e0, 9.936e-1, 1.005e0, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1217.3
- **Average Gradient Evaluations per Run:** 2431.8
- **Average Iterations per Run:** 1214.3
- **Average Time per Run:** 0.042s
- **Function Evaluations per Second:** 29291.2
- **Iterations per Second:** 29219.0
### Resource Utilization
- **Total Function Evaluations:** 24347
- **Total Gradient Evaluations:** 48636
- **Total Computation Time:** 0.8s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/IllConditionedRosenbrock_10D_alpha100_results.csv)
* [Convergence Plot](../plots/IllConditionedRosenbrock_10D_alpha100.png)
* [Log Scale Convergence Plot](../plots/IllConditionedRosenbrock_10D_alpha100_log.png)


---
*Detailed report for GD-WeightDecay on IllConditionedRosenbrock_10D_alpha100*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
