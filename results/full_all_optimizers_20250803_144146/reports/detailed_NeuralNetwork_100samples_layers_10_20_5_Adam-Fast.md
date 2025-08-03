# Detailed Analysis: Adam-Fast on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** Adam-Fast
**Problem Family:** Neural Networks
**Dimension:** 325
**Success Threshold:** 3.820e-2
**Total Runs:** 20
**Successful Runs:** 8 (40.0%)

### Quick Statistics
* **Best Final Value:** 3.703019e-2
* **Worst Final Value:** 6.580659e-2
* **Mean Final Value:** 4.464093e-2
* **Success Rate:** 40.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.812e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.602e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.287e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.970e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.760e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.537e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">223</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.349</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.703e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.281e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.807e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.178e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.409e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.349e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">91</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.986e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.952e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">329</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.529</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.775e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.292e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.243</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.940e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.415e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">87</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.019e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.964e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.383e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.186e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">283</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.757e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.289e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.787e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.778e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.304e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.834e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">108</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">219</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">218</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.354</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.816e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.516e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.581e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.689e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.320e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.641e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">64</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.217</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.516e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.663e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.763e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.676e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.265</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.558e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.255e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.255</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (8 out of 20)

* **Average Iterations to Convergence:** 70.5
* **Average Function Evaluations:** 144.0
* **Average Time to Convergence:** 0.229s
* **Fastest Convergence:** 54 iterations (0.179s)
* **Slowest Convergence:** 86 iterations (0.275s)

### Failed Runs (12 out of 20)

**Failure Reasons:**
- FunctionTolerance: 12 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 4)
**Final Value:** 3.703019e-2
**Final Gradient Norm:** 2.281404e-1
**Iterations:** 54
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.051e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.071e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.988e-1, 4.546e-2, -1.409e-1, 7.582e-4, -1.214e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.051e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.071e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.988e-1, 1.455e-1, -4.088e-2, 1.006e-1, -2.214e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.811e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.073e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.018e-1, 1.648e-1, -5.828e-2, 2.007e-1, -2.053e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.551e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.277e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.658e-1, 1.800e-1, -8.277e-2, 2.980e-1, -1.833e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.501e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.825e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.452e-1, 2.127e-1, -1.027e-1, 3.909e-1, -1.466e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">50</td><td style="border: 1px solid #ddd; padding: 4px;">4.059e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.317e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.290e-1, 5.098e-1, -5.359e-1, -4.761e-2, -4.094e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">51</td><td style="border: 1px solid #ddd; padding: 4px;">4.062e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.378e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.490e-1, 5.086e-1, -5.255e-1, -3.351e-2, -4.010e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">52</td><td style="border: 1px solid #ddd; padding: 4px;">3.954e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.110e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.583e-1, 5.243e-1, -5.200e-1, -2.323e-2, -3.980e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">53</td><td style="border: 1px solid #ddd; padding: 4px;">3.843e-2</td><td style="border: 1px solid #ddd; padding: 4px;">3.141e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.701e-1, 5.396e-1, -5.239e-1, -9.058e-3, -3.856e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">54</td><td style="border: 1px solid #ddd; padding: 4px;">3.703e-2</td><td style="border: 1px solid #ddd; padding: 4px;">2.281e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.701e-1, 5.396e-1, -5.239e-1, -9.058e-3, -3.856e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 184.6
- **Average Gradient Evaluations per Run:** 184.0
- **Average Iterations per Run:** 90.8
- **Average Time per Run:** 0.295s
- **Function Evaluations per Second:** 625.6
- **Iterations per Second:** 307.7
### Resource Utilization
- **Total Function Evaluations:** 3692
- **Total Gradient Evaluations:** 3680
- **Total Computation Time:** 5.9s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for Adam-Fast on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
