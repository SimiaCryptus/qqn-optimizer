# Detailed Analysis: L-BFGS-MoreThuente on PenaltyI_5D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_5D_alpha1e6
**Optimizer:** L-BFGS-MoreThuente
**Problem Family:** PenaltyI
**Dimension:** 5
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.572585e0
* **Worst Final Value:** 1.923780e1
* **Mean Final Value:** 1.289552e1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.550e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.874e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.632e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.081e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.033</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.573e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.362e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">361</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2143</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.790e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.600e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.456e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.633e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.837e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.573e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.244e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.053e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.772e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.369e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.924e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.772e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.467e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.661e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.143e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.760e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.564e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.908e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.646e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.113e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.622e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.056e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.031</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.193e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.909e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.731e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.320e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.688e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.172e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">359</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2867</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.558e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.851e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2144</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.752e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.372e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.292e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.188e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2859</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2145</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 3.572585e0
**Final Gradient Norm:** 6.362065e2
**Iterations:** 361
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.099e5</td><td style="border: 1px solid #ddd; padding: 4px;">9.163e5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.295e-1, 4.849e-1, 4.452e-1, 3.758e-1, 5.119e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.099e5</td><td style="border: 1px solid #ddd; padding: 4px;">9.163e5</td><td style="border: 1px solid #ddd; padding: 4px;">3.499e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.416e-1, -1.309e0, -1.045e0, -5.846e-1, -1.488e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.199e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.378e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.151e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.416e-1, -1.309e0, -1.045e0, -5.846e-1, -1.488e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.199e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.378e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.150e-5</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.415e-1, -1.309e0, -1.045e0, -5.845e-1, -1.488e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.198e1</td><td style="border: 1px solid #ddd; padding: 4px;">9.377e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.390e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.777e-1, -7.573e-1, -5.566e-1, -2.060e-1, -8.936e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">357</td><td style="border: 1px solid #ddd; padding: 4px;">3.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.362e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.242e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[2.496e-1, 1.076e-1, 2.095e-1, 2.503e-1, 3.841e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">358</td><td style="border: 1px solid #ddd; padding: 4px;">3.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.362e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.242e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[2.496e-1, 1.076e-1, 2.095e-1, 2.503e-1, 3.841e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">359</td><td style="border: 1px solid #ddd; padding: 4px;">3.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.362e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.242e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[2.496e-1, 1.076e-1, 2.095e-1, 2.503e-1, 3.841e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">360</td><td style="border: 1px solid #ddd; padding: 4px;">3.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.362e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.242e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[2.496e-1, 1.076e-1, 2.095e-1, 2.503e-1, 3.841e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">361</td><td style="border: 1px solid #ddd; padding: 4px;">3.573e0</td><td style="border: 1px solid #ddd; padding: 4px;">6.362e2</td><td style="border: 1px solid #ddd; padding: 4px;">2.242e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[2.496e-1, 1.076e-1, 2.095e-1, 2.503e-1, 3.841e-2]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 2860.6
- **Average Gradient Evaluations per Run:** 2144.8
- **Average Iterations per Run:** 356.9
- **Average Time per Run:** 0.036s
- **Function Evaluations per Second:** 79779.6
- **Iterations per Second:** 9953.8
### Resource Utilization
- **Total Function Evaluations:** 57211
- **Total Gradient Evaluations:** 42895
- **Total Computation Time:** 0.7s
- **Function/Gradient Ratio:** 1.33
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/PenaltyI_5D_alpha1e6_results.csv)
* [Convergence Plot](../plots/PenaltyI_5D_alpha1e6.png)
* [Log Scale Convergence Plot](../plots/PenaltyI_5D_alpha1e6_log.png)


---
*Detailed report for L-BFGS-MoreThuente on PenaltyI_5D_alpha1e6*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
