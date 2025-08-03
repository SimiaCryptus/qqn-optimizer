# Detailed Analysis: Adam on StyblinskiTang_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** StyblinskiTang_5D
**Optimizer:** Adam
**Problem Family:** StyblinskiTang
**Dimension:** 5
**Success Threshold:** -1.950e2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -8.298896e1
* **Worst Final Value:** -6.635081e1
* **Mean Final Value:** -7.384963e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.723e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.230e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.060e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.278e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.966e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.115e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.299e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.306e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.635e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.941e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.111e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.012e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.123e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.141e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.853e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.969e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.532e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.069e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.986e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.269e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-8.033e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.274e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.999e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.990e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.077e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.004e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.171e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.147e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.177e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.021e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.770e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.240e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.516e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.201e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.062</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.033e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.998e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.254e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.030e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-7.384e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.179e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.060</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** -8.298896e1
**Final Gradient Norm:** 4.305530e1
**Iterations:** 1249
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.849e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.499e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.745e-1, -1.339e-1, -1.455e-1, 8.760e-2, -9.680e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.849e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.499e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.755e-1, -1.349e-1, -1.465e-1, 8.660e-2, -9.780e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-1.869e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.532e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.765e-1, -1.359e-1, -1.475e-1, 8.560e-2, -9.880e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-1.889e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.565e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.775e-1, -1.369e-1, -1.485e-1, 8.460e-2, -9.980e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-1.909e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.599e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.785e-1, -1.379e-1, -1.495e-1, 8.360e-2, -1.008e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1245</td><td style="border: 1px solid #ddd; padding: 4px;">-8.260e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.301e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.360e0, -1.353e0, -1.355e0, -1.395e0, -1.350e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1246</td><td style="border: 1px solid #ddd; padding: 4px;">-8.270e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.302e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.361e0, -1.354e0, -1.356e0, -1.396e0, -1.351e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1247</td><td style="border: 1px solid #ddd; padding: 4px;">-8.280e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.303e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.362e0, -1.355e0, -1.357e0, -1.397e0, -1.352e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1248</td><td style="border: 1px solid #ddd; padding: 4px;">-8.289e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.304e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.363e0, -1.356e0, -1.358e0, -1.398e0, -1.353e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1249</td><td style="border: 1px solid #ddd; padding: 4px;">-8.299e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.306e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.363e0, -1.356e0, -1.358e0, -1.398e0, -1.353e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2502.0
- **Average Gradient Evaluations per Run:** 2502.0
- **Average Iterations per Run:** 1249.0
- **Average Time per Run:** 0.055s
- **Function Evaluations per Second:** 45469.4
- **Iterations per Second:** 22698.4
### Resource Utilization
- **Total Function Evaluations:** 50040
- **Total Gradient Evaluations:** 50040
- **Total Computation Time:** 1.1s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/StyblinskiTang_5D_results.csv)
* [Convergence Plot](../plots/StyblinskiTang_5D.png)
* [Log Scale Convergence Plot](../plots/StyblinskiTang_5D_log.png)


---
*Detailed report for Adam on StyblinskiTang_5D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
