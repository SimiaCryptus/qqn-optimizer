# Detailed Analysis: GD-WeightDecay on StyblinskiTang_5D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** StyblinskiTang_5D
**Optimizer:** GD-WeightDecay
**Problem Family:** Highly Multimodal
**Dimension:** 5
**Success Threshold:** -1.960e2
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -1.958308e2
* **Worst Final Value:** -1.559795e2
* **Mean Final Value:** -1.853686e2
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.956e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.140e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">46</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">88</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.953e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.134e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">52</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.813e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.291e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.958e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.538e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">116</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">234</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.817e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.719e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">93</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.560e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.675e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.813e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.261e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">50</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">53</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.816e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.239e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">55</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">58</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.956e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.797e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">48</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">51</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">98</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.817e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.213e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">89</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">92</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.958e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.381e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">112</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">226</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.561e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.121e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.958e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.717e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">115</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">118</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">232</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.816e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.401e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">60</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">63</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.958e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.357e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">222</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.675e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.391e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">54</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">57</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.817e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.055e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">104</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.958e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.563e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">106</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">214</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.958e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.283e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">107</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">216</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.955e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.178e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** -1.958308e2
**Final Gradient Norm:** 1.716701e-3
**Iterations:** 115
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.457e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.253e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-5.055e-3, 3.581e-2, -4.097e-2, -1.025e-1, 7.628e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-2.457e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.253e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.796e-2, 2.617e-2, -5.674e-2, -1.232e-1, 6.988e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-4.490e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.752e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.222e-2, 8.056e-3, -8.640e-2, -1.621e-1, 5.785e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-8.733e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.688e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-7.750e-2, -1.829e-2, -1.295e-1, -2.187e-1, 4.034e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-1.588e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.046e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.244e-1, -5.333e-2, -1.869e-1, -2.938e-1, 1.707e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">110</td><td style="border: 1px solid #ddd; padding: 4px;">-1.958e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.808e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.904e0, -2.904e0, -2.904e0, -2.904e0, -2.904e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">111</td><td style="border: 1px solid #ddd; padding: 4px;">-1.958e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.398e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.904e0, -2.904e0, -2.904e0, -2.904e0, -2.904e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">112</td><td style="border: 1px solid #ddd; padding: 4px;">-1.958e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.676e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.904e0, -2.904e0, -2.904e0, -2.904e0, -2.904e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">113</td><td style="border: 1px solid #ddd; padding: 4px;">-1.958e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.558e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.904e0, -2.904e0, -2.904e0, -2.904e0, -2.904e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">114</td><td style="border: 1px solid #ddd; padding: 4px;">-1.958e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.165e-3</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.904e0, -2.904e0, -2.904e0, -2.904e0, -2.904e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 76.7
- **Average Gradient Evaluations per Run:** 149.3
- **Average Iterations per Run:** 73.7
- **Average Time per Run:** 0.002s
- **Function Evaluations per Second:** 31988.6
- **Iterations per Second:** 30736.6
### Resource Utilization
- **Total Function Evaluations:** 1533
- **Total Gradient Evaluations:** 2986
- **Total Computation Time:** 0.0s
- **Function/Gradient Ratio:** 0.51
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/StyblinskiTang_5D_results.csv)
* [Convergence Plot](convergence_StyblinskiTang_5D.png)
* [Log Scale Convergence Plot](convergence_StyblinskiTang_5D_log.png)


---
*Detailed report for GD-WeightDecay on StyblinskiTang_5D*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
