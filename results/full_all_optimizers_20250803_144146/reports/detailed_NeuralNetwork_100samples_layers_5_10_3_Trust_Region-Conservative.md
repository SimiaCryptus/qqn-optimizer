# Detailed Analysis: Trust Region-Conservative on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** Trust Region-Conservative
**Problem Family:** Neural Networks
**Dimension:** 93
**Success Threshold:** 1.400e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 2.144461e-1
* **Worst Final Value:** 2.242858e-1
* **Mean Final Value:** 2.193887e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.200e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.230e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">203</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.197e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.496e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">61</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">124</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.117</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.160e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.855e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">94</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.086</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.213e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.362e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.041</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.215e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.485e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.170e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.605e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">224</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">150</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.220e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.874e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.205e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.897e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.243e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.653e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.162e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.259e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">67</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">203</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">136</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.180e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.251e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">230</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.196e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.419e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">82</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">248</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.198e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.685e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.180e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.844e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.219e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.990e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.163e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.642e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.230e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.932e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.183e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.089e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.056</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.200e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.214e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">91</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">275</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.172</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.144e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.477e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.079</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 2.144461e-1
**Final Gradient Norm:** 9.476578e-2
**Iterations:** 42
**Convergence Reason:** GradientTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.461e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.353e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.654e-2, -8.743e-2, 1.625e-1, 4.265e-2, 1.652e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.461e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.353e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.393e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.655e-2, -8.743e-2, 1.624e-1, 4.252e-2, 1.653e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.428e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.335e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.490e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.656e-2, -8.742e-2, 1.623e-1, 4.238e-2, 1.654e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">3.395e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.317e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.592e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.657e-2, -8.742e-2, 1.622e-1, 4.224e-2, 1.655e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">3.362e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.300e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.695e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.658e-2, -8.741e-2, 1.621e-1, 4.209e-2, 1.657e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">2.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.489e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.867e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.555e-2, -8.795e-2, 1.602e-1, 2.092e-2, 1.690e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">38</td><td style="border: 1px solid #ddd; padding: 4px;">2.220e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.848e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.511e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.535e-2, -8.830e-2, 1.612e-1, 1.624e-2, 1.687e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">39</td><td style="border: 1px solid #ddd; padding: 4px;">2.191e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.109e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.741e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.515e-2, -8.904e-2, 1.634e-1, 7.650e-3, 1.677e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">40</td><td style="border: 1px solid #ddd; padding: 4px;">2.165e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.273e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.858e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.447e-2, -9.153e-2, 1.713e-1, -1.438e-2, 1.641e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">41</td><td style="border: 1px solid #ddd; padding: 4px;">2.144e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.477e-2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.447e-2, -9.153e-2, 1.713e-1, -1.438e-2, 1.641e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 157.6
- **Average Gradient Evaluations per Run:** 105.7
- **Average Iterations per Run:** 51.9
- **Average Time per Run:** 0.098s
- **Function Evaluations per Second:** 1612.6
- **Iterations per Second:** 530.7
### Resource Utilization
- **Total Function Evaluations:** 3151
- **Total Gradient Evaluations:** 2114
- **Total Computation Time:** 2.0s
- **Function/Gradient Ratio:** 1.49
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/NeuralNetwork_100samples_layers_5_10_3_results.csv)
* [Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3.png)
* [Log Scale Convergence Plot](../plots/NeuralNetwork_100samples_layers_5_10_3_log.png)


---
*Detailed report for Trust Region-Conservative on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
