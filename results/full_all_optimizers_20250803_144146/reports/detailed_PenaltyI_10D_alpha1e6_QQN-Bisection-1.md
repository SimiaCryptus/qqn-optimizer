# Detailed Analysis: QQN-Bisection-1 on PenaltyI_10D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_10D_alpha1e6
**Optimizer:** QQN-Bisection-1
**Problem Family:** PenaltyI
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 5.625835e0
* **Worst Final Value:** 5.634851e0
* **Mean Final Value:** 5.628833e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.631e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.557e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2877</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.626e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.298e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.632e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.505e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.628e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.188e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.629e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.745e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2897</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.630e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.619e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.627e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.099e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">64</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.627e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.267e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2904</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.627e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.049e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">64</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.628e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.786e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2922</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.629e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.853e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.628e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.630e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2841</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.635e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.660e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2915</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.626e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.825e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2193</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.629e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.623e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2930</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.633e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.969e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2902</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.075</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.629e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.413e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2915</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.628e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.042e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">65</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2153</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2926</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.628e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.096e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.626e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.109e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2922</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 5.625835e0
**Final Gradient Norm:** 4.108785e0
**Iterations:** 63
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.599e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.625e6</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.062e-1, 3.211e-1, 5.571e-1, 6.003e-1, 3.188e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.599e5</td><td style="border: 1px solid #ddd; padding: 4px;">1.625e6</td><td style="border: 1px solid #ddd; padding: 4px;">4.999e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.501e-1, 2.501e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.658e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.643e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.526e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[2.490e-1, 2.495e-1, 2.480e-1, 2.477e-1, 2.495e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.753e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.490e-1, 2.496e-1, 2.480e-1, 2.477e-1, 2.496e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.753e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.104e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[2.491e-1, 2.496e-1, 2.481e-1, 2.478e-1, 2.497e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">59</td><td style="border: 1px solid #ddd; padding: 4px;">5.626e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.258e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.499e-1, 2.498e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">60</td><td style="border: 1px solid #ddd; padding: 4px;">5.626e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.943e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.815e-6</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.499e-1, 2.498e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">61</td><td style="border: 1px solid #ddd; padding: 4px;">5.626e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.659e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.499e-1, 2.498e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">62</td><td style="border: 1px solid #ddd; padding: 4px;">5.626e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.504e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.499e-1, 2.498e-1, 2.500e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">63</td><td style="border: 1px solid #ddd; padding: 4px;">5.626e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.109e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.537e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.500e-1, 2.500e-1, 2.499e-1, 2.498e-1, 2.500e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2161.3
- **Average Gradient Evaluations per Run:** 2898.1
- **Average Iterations per Run:** 63.0
- **Average Time per Run:** 0.074s
- **Function Evaluations per Second:** 29301.2
- **Iterations per Second:** 854.8
### Resource Utilization
- **Total Function Evaluations:** 43226
- **Total Gradient Evaluations:** 57961
- **Total Computation Time:** 1.5s
- **Function/Gradient Ratio:** 0.75
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/PenaltyI_10D_alpha1e6_results.csv)
* [Convergence Plot](../plots/PenaltyI_10D_alpha1e6.png)
* [Log Scale Convergence Plot](../plots/PenaltyI_10D_alpha1e6_log.png)


---
*Detailed report for QQN-Bisection-1 on PenaltyI_10D_alpha1e6*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
