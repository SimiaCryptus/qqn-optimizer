# Detailed Analysis: Adam-WeightDecay on Rastrigin_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rastrigin_2D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Highly Multimodal
**Dimension:** 2
**Success Threshold:** 7.960e0
**Total Runs:** 20
**Successful Runs:** 13 (65.0%)

### Quick Statistics
* **Best Final Value:** 7.975450e0
* **Worst Final Value:** 1.295407e1
* **Mean Final Value:** 8.971732e0
* **Success Rate:** 65.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.977e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.721e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.977e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.680e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">190</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.604e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">267</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">266</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.791e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">232</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.853e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">309</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">308</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.578e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">239</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.795e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.540e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.573e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">325</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">324</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.977e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.646e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">228</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.975e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.530e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">278</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.630e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.945e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">239</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">238</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.977e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.681e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">99</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">201</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">200</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.597e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">274</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.569e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">311</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">310</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.589e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">126</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">254</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.976e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.604e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">190</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.776e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">105</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">213</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">212</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.802e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">85</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (13 out of 20)
- **Average Iterations to Convergence:** 118.6
- **Average Function Evaluations:** 240.2
- **Average Time to Convergence:** 0.005s
- **Fastest Convergence:** 85 iterations (0.004s)
- **Slowest Convergence:** 161 iterations (0.007s)
### Failed Runs (7 out of 20)

**Failure Reasons:**
- FunctionTolerance: 7 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 11)
**Final Value:** 7.975450e0
**Final Gradient Norm:** 3.530471e0
**Iterations:** 138
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.720e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.614e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 2.386e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.720e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.614e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.993e0, 2.383e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.706e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.703e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.991e0, 2.380e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.692e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.788e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.989e0, 2.377e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.677e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.873e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.988e0, 2.374e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">133</td><td style="border: 1px solid #ddd; padding: 4px;">7.962e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.340e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.989e0, 1.985e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">134</td><td style="border: 1px solid #ddd; padding: 4px;">7.965e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.982e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.991e0, 1.983e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">135</td><td style="border: 1px solid #ddd; padding: 4px;">7.968e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.624e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.992e0, 1.982e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">136</td><td style="border: 1px solid #ddd; padding: 4px;">7.972e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.153e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.991e0, 1.981e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">137</td><td style="border: 1px solid #ddd; padding: 4px;">7.974e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.421e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.981e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 252.3
- **Average Gradient Evaluations per Run:** 251.3
- **Average Iterations per Run:** 124.7
- **Average Time per Run:** 0.005s
- **Function Evaluations per Second:** 48343.6
- **Iterations per Second:** 23884.4
### Resource Utilization
- **Total Function Evaluations:** 5046
- **Total Gradient Evaluations:** 5026
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Rastrigin_2D_results.csv)
* [Convergence Plot](convergence_Rastrigin_2D.png)
* [Log Scale Convergence Plot](convergence_Rastrigin_2D_log.png)


---
*Detailed report for Adam-WeightDecay on Rastrigin_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
