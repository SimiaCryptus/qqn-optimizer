# Detailed Analysis: L-BFGS-Conservative on IllConditionedRosenbrock_2D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_2D_alpha100
**Optimizer:** L-BFGS-Conservative
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 16 (80.0%)

### Quick Statistics
* **Best Final Value:** 2.050670e-9
* **Worst Final Value:** 4.764278e-3
* **Mean Final Value:** 2.694050e-4
* **Success Rate:** 80.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.724e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.354e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">270</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2841</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1090</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.051e-9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.639e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">125</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">887</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">506</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.285e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.990e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2851</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.947e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.567e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">111</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">876</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.526e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.829e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">90</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">801</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">375</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.829e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.561e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">645</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.022</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.271e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.462e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1622</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">779</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.812e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.237e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">657</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.023</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.021e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.828e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">110</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">718</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.895e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.491e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">109</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">687</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">445</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.399e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.865e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.067</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.452e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.527e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">205</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2057</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">829</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.226e-8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.323e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1610</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">718</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.182e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.062e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">200</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1816</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">805</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.259e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.135e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">278</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2899</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.040</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.484e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.940e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">113</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">800</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.234e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.406e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">803</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">484</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.651e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.144e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1736</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">796</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Yes</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.036e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.992e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">282</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.764e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.532e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3853</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (16 out of 20)

* **Average Iterations to Convergence:** 163.6
* **Average Function Evaluations:** 1411.6
* **Average Time to Convergence:** 0.023s
* **Fastest Convergence:** 109 iterations (0.014s)
* **Slowest Convergence:** 278 iterations (0.040s)

### Failed Runs (4 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 4 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** 2.050670e-9
**Final Gradient Norm:** 1.638516e-3
**Iterations:** 125
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.416e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.631e2</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.204e0, 1.145e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.416e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.631e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.078e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.104e0, 1.185e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">4.542e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.030e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.563e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.074e0, 1.196e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">4.478e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.619e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.075e0, 1.195e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">4.468e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.549e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.075e0, 1.195e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">121</td><td style="border: 1px solid #ddd; padding: 4px;">1.449e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.434e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[9.979e-1, 9.955e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">122</td><td style="border: 1px solid #ddd; padding: 4px;">1.446e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.383e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.813e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[9.980e-1, 9.956e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">123</td><td style="border: 1px solid #ddd; padding: 4px;">1.445e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.392e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.813e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[9.982e-1, 9.961e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">124</td><td style="border: 1px solid #ddd; padding: 4px;">1.444e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.467e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">125</td><td style="border: 1px solid #ddd; padding: 4px;">2.051e-9</td><td style="border: 1px solid #ddd; padding: 4px;">1.639e-3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.000e0, 1.000e0]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1800.6
- **Average Gradient Evaluations per Run:** 861.2
- **Average Iterations per Run:** 213.1
- **Average Time per Run:** 0.030s
- **Function Evaluations per Second:** 61014.8
- **Iterations per Second:** 7219.4
### Resource Utilization
- **Total Function Evaluations:** 36012
- **Total Gradient Evaluations:** 17224
- **Total Computation Time:** 0.6s
- **Function/Gradient Ratio:** 2.09
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0
### Failed Run Details

**Run 1 (ID: 3)**
- Final Value: 4.285031e-6
- Final Gradient Norm: 1.989947e-2
- Iterations: 537
- Function Evaluations: 2851
- Reason: MaxFunctionEvaluations

**Run 2 (ID: 11)**
- Final Value: 9.399468e-6
- Final Gradient Norm: 6.864957e-3
- Iterations: 535
- Function Evaluations: 2859
- Reason: MaxFunctionEvaluations

**Run 3 (ID: 19)**
- Final Value: 6.035541e-4
- Final Gradient Norm: 9.991893e-1
- Iterations: 282
- Function Evaluations: 3863
- Reason: MaxFunctionEvaluations

**Run 4 (ID: 20)**
- Final Value: 4.764278e-3
- Final Gradient Norm: 6.531842e-2
- Iterations: 290
- Function Evaluations: 3853
- Reason: MaxFunctionEvaluations



## Data Files
* [Raw CSV Data](../data/problems/IllConditionedRosenbrock_2D_alpha100_results.csv)
* [Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100.png)
* [Log Scale Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100_log.png)


---
*Detailed report for L-BFGS-Conservative on IllConditionedRosenbrock_2D_alpha100*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
