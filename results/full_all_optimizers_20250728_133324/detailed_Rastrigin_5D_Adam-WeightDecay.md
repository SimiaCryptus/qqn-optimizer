# Detailed Analysis: Adam-WeightDecay on Rastrigin_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Rastrigin_5D
**Optimizer:** Adam-WeightDecay
**Problem Family:** Highly Multimodal
**Dimension:** 5
**Success Threshold:** 2.090e1
**Total Runs:** 20
**Successful Runs:** 11 (55.0%)

### Quick Statistics
* **Best Final Value:** 1.694223e1
* **Worst Final Value:** 2.988225e1
* **Mean Final Value:** 2.192173e1
* **Success Rate:** 55.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.992e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.478e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">235</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">234</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.988e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.157e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">253</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">252</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.491e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.965e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">272</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.994e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.807e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">143</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">289</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">288</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.779e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.956e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">263</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">262</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.994e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.409e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">137</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">277</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">276</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.490e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.421e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">123</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.994e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.406e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.561e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">127</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">257</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">256</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.491e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.556e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">273</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">272</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.490e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.528e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">271</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.690e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.292e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">279</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">278</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.491e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.619e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">251</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">250</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.696e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.890e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">297</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">296</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.609e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">269</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.694e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.702e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">258</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.191e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.493e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">259</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">258</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.862e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">237</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.193e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.676e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">287</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">286</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (11 out of 20)
- **Average Iterations to Convergence:** 132.2
- **Average Function Evaluations:** 267.4
- **Average Time to Convergence:** 0.006s
- **Fastest Convergence:** 116 iterations (0.005s)
- **Slowest Convergence:** 147 iterations (0.008s)
### Failed Runs (9 out of 20)

**Failure Reasons:**
- FunctionTolerance: 9 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 17)
**Final Value:** 1.694223e1
**Final Gradient Norm:** 4.702281e0
**Iterations:** 128
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.282e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.218e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[2.184e0, 2.267e0, 2.289e0, 2.105e0, 1.455e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.282e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.218e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.181e0, 2.264e0, 2.286e0, 2.102e0, 1.452e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">7.205e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.217e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.178e0, 2.261e0, 2.283e0, 2.099e0, 1.449e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">7.128e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.215e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.175e0, 2.258e0, 2.280e0, 2.096e0, 1.446e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">7.050e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.212e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.172e0, 2.255e0, 2.277e0, 2.093e0, 1.443e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">123</td><td style="border: 1px solid #ddd; padding: 4px;">1.692e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.249e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.990e0, 1.989e0, 1.990e0, 9.872e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">124</td><td style="border: 1px solid #ddd; padding: 4px;">1.693e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.081e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.991e0, 1.990e0, 1.990e0, 9.855e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">125</td><td style="border: 1px solid #ddd; padding: 4px;">1.693e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.774e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.990e0, 1.992e0, 1.991e0, 1.990e0, 9.844e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">126</td><td style="border: 1px solid #ddd; padding: 4px;">1.694e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.277e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.989e0, 1.992e0, 1.992e0, 1.990e0, 9.838e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">127</td><td style="border: 1px solid #ddd; padding: 4px;">1.694e1</td><td style="border: 1px solid #ddd; padding: 4px;">4.584e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.500e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.989e0, 1.993e0, 1.993e0, 1.990e0, 9.837e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 266.8
- **Average Gradient Evaluations per Run:** 265.8
- **Average Iterations per Run:** 131.9
- **Average Time per Run:** 0.006s
- **Function Evaluations per Second:** 44439.4
- **Iterations per Second:** 21969.8
### Resource Utilization
- **Total Function Evaluations:** 5336
- **Total Gradient Evaluations:** 5316
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Rastrigin_5D_results.csv)
* [Convergence Plot](convergence_Rastrigin_5D.png)
* [Log Scale Convergence Plot](convergence_Rastrigin_5D_log.png)


---
*Detailed report for Adam-WeightDecay on Rastrigin_5D*
*Generated on: 2025-07-28 13:54:23 UTC*
*[← Back to Main Report](benchmark_report.md)*
