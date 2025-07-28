# Detailed Analysis: Adam-WeightDecay on Griewank_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Griewank_2D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Griewank
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 5.438691e0
* **Worst Final Value:** 5.766924e0
* **Mean Final Value:** 5.583562e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.601e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.191e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.613e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.112e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.737e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.234e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.510e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.234e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.552e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.263e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.689e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.888e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.498e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.235e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.467e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.210e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.638e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.818e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.767e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.863e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.498e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.285e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.568e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.070e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.575e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.488e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.240e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.466e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.247e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.614e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.060e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.622e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.930e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.439e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.187e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.563e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.648e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.849e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 18)
**Final Value:** 5.438691e0
**Final Gradient Norm:** 6.186582e-1
**Iterations:** 249
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.934e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.998e1, 9.987e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">5.934e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e1, 9.987e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.932e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.488e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e1, 9.986e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.930e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.476e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.997e1, 9.986e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.928e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.463e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[9.996e1, 9.986e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">245</td><td style="border: 1px solid #ddd; padding: 4px;">5.447e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.207e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e2, 9.914e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">246</td><td style="border: 1px solid #ddd; padding: 4px;">5.445e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.202e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e2, 9.913e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">247</td><td style="border: 1px solid #ddd; padding: 4px;">5.443e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.197e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e2, 9.913e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">248</td><td style="border: 1px solid #ddd; padding: 4px;">5.441e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.192e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e2, 9.913e1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">249</td><td style="border: 1px solid #ddd; padding: 4px;">5.439e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.187e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.002e2, 9.913e1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 502.0
- **Average Gradient Evaluations per Run:** 502.0
- **Average Iterations per Run:** 249.0
- **Average Time per Run:** 0.010s
- **Function Evaluations per Second:** 48503.4
- **Iterations per Second:** 24058.5
### Resource Utilization
- **Total Function Evaluations:** 10040
- **Total Gradient Evaluations:** 10040
- **Total Computation Time:** 0.2s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Griewank_2D_results.csv)
* [Convergence Plot](convergence_Griewank_2D.png)
* [Log Scale Convergence Plot](convergence_Griewank_2D_log.png)


---
*Detailed report for Adam-WeightDecay on Griewank_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
