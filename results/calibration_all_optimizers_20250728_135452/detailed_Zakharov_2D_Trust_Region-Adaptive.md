# Detailed Analysis: Trust Region-Adaptive on Zakharov_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Zakharov_2D
**Optimizer:** Trust Region-Adaptive
**Problem Family:** Zakharov
**Dimension:** 2
**Success Threshold:** 1.000e-8
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.585470e-2
* **Worst Final Value:** 2.116199e2
* **Mean Final Value:** 1.127554e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.337e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.350e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.724e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.093e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">66</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.031e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.136e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.409e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.758e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.331e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.167e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.932e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.629e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">84</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.214e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.442e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.585e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.684e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">83</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">56</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.398e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.379e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.601e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.346e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.728e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.095e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">59</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.227e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.304e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">49</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">149</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">100</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.161e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.807e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">62</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.421e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.188e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.610e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.075e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">86</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.116e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.294e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">114</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.707e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.049e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">101</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.638e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.052e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.443e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.057e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">68</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.153e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.141e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">128</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- GradientTolerance: 18 runs
- FunctionTolerance: 2 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 8)
**Final Value:** 3.585470e-2
**Final Gradient Norm:** 5.683862e-1
**Iterations:** 27
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.010e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.504e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.419e-1, 8.777e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">6.010e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.504e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.662e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.338e-1, 8.632e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">5.764e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.455e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.718e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.253e-1, 8.482e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.518e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.406e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.778e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.166e-1, 8.327e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.273e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.356e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.843e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.075e-1, 8.167e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">22</td><td style="border: 1px solid #ddd; padding: 4px;">7.056e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.965e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.430e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[4.283e-1, 2.371e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">23</td><td style="border: 1px solid #ddd; padding: 4px;">4.848e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.295e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.089e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.575e-1, 1.543e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">24</td><td style="border: 1px solid #ddd; padding: 4px;">2.749e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.587e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.575e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.462e-1, 4.292e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">25</td><td style="border: 1px solid #ddd; padding: 4px;">9.075e-2</td><td style="border: 1px solid #ddd; padding: 4px;">7.973e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.136e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.633e-2, -1.286e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">26</td><td style="border: 1px solid #ddd; padding: 4px;">3.585e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.684e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.633e-2, -1.286e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 124.3
- **Average Gradient Evaluations per Run:** 83.5
- **Average Iterations per Run:** 40.8
- **Average Time per Run:** 0.001s
- **Function Evaluations per Second:** 157848.6
- **Iterations per Second:** 51727.6
### Resource Utilization
- **Total Function Evaluations:** 2487
- **Total Gradient Evaluations:** 1670
- **Total Computation Time:** 0.0s
- **Function/Gradient Ratio:** 1.49
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Zakharov_2D_results.csv)
* [Convergence Plot](convergence_Zakharov_2D.png)
* [Log Scale Convergence Plot](convergence_Zakharov_2D_log.png)


---
*Detailed report for Trust Region-Adaptive on Zakharov_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
