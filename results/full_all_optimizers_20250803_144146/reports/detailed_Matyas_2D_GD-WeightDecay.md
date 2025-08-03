# Detailed Analysis: GD-WeightDecay on Matyas_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Matyas_2D
**Optimizer:** GD-WeightDecay
**Problem Family:** Matyas
**Dimension:** 2
**Success Threshold:** 2.500e-2
**Total Runs:** 20
**Successful Runs:** 20 (100.0%)

### Quick Statistics
* **Best Final Value:** 2.495067e-2
* **Worst Final Value:** 2.499999e-2
* **Mean Final Value:** 2.497744e-2
* **Success Rate:** 100.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.471e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">278</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">281</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.495e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.470e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">321</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.500e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.478e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.500e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.494e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">239</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.471e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">195</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">387</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">304</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">307</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">611</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.495e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.470e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.471e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">276</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">549</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.536e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">661</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.471e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">353</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">226</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">229</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">455</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.499e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.471e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.498e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.470e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">341</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.496e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.468e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">499</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.470e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">245</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.469e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">267</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.500e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.472e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">323</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">643</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.470e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">661</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (20 out of 20)

* **Average Iterations to Convergence:** 220.7
* **Average Function Evaluations:** 223.7
* **Average Time to Convergence:** 0.007s
* **Fastest Convergence:** 58 iterations (0.002s)
* **Slowest Convergence:** 329 iterations (0.010s)

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 2.495067e-2
**Final Gradient Norm:** 4.469939e-2
**Iterations:** 159
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.302e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.007e0, 8.389e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.120e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.302e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.007e0, 8.391e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.111e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.296e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.006e0, 8.395e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.096e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.286e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.004e0, 8.401e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.076e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.273e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.003e0, 8.408e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">155</td><td style="border: 1px solid #ddd; padding: 4px;">2.515e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.489e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.933e-1, 7.911e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">156</td><td style="border: 1px solid #ddd; padding: 4px;">2.510e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.484e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.924e-1, 7.903e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">157</td><td style="border: 1px solid #ddd; padding: 4px;">2.505e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.479e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.916e-1, 7.895e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">158</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.475e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[7.908e-1, 7.888e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">159</td><td style="border: 1px solid #ddd; padding: 4px;">2.495e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.470e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[7.908e-1, 7.888e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 223.7
- **Average Gradient Evaluations per Run:** 444.4
- **Average Iterations per Run:** 220.7
- **Average Time per Run:** 0.007s
- **Function Evaluations per Second:** 34301.8
- **Iterations per Second:** 33841.8
### Resource Utilization
- **Total Function Evaluations:** 4474
- **Total Gradient Evaluations:** 8888
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

*No failed runs to analyze.*



## Data Files
* [Raw CSV Data](../data/problems/Matyas_2D_results.csv)
* [Convergence Plot](../plots/Matyas_2D.png)
* [Log Scale Convergence Plot](../plots/Matyas_2D_log.png)


---
*Detailed report for GD-WeightDecay on Matyas_2D*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
