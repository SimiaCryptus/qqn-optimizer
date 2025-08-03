# Detailed Analysis: Adam-WeightDecay on Barrier_10D_mu0.1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Barrier_10D_mu0.1
**Optimizer:** Adam-WeightDecay
**Problem Family:** Barrier
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.997866e0
* **Worst Final Value:** 1.997866e0
* **Mean Final Value:** 1.997866e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.193e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">578</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.332e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">564</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1130</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.048e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">577</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1156</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">597</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1197</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.223e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">536</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1075</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1074</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.126e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">593</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1189</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.252e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">594</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1190</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.387e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.646e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">604</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1211</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1210</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.946e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">566</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.231e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">570</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1143</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1142</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.714e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.626e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">568</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1139</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1138</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.708e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1146</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.609e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">576</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1155</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.840e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">526</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1055</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.539e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">582</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.189e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">539</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1081</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1080</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.682e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1207</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1206</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.028</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.196e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">537</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1077</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1076</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.024</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 20)
**Final Value:** 1.997866e0
**Final Gradient Norm:** 5.196411e-4
**Iterations:** 537
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.075e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.261e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.102e0, 1.133e0, 8.351e-1, 9.553e-1, 8.044e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.075e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.261e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.099e0, 1.130e0, 8.321e-1, 9.523e-1, 8.014e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.069e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.241e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.096e0, 1.127e0, 8.291e-1, 9.493e-1, 7.984e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.063e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.222e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.093e0, 1.124e0, 8.261e-1, 9.463e-1, 7.954e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.058e1</td><td style="border: 1px solid #ddd; padding: 4px;">6.202e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.090e0, 1.121e0, 8.231e-1, 9.433e-1, 7.924e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">532</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.235e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">533</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.210e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">534</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.194e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">535</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.187e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">536</td><td style="border: 1px solid #ddd; padding: 4px;">1.998e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.188e-4</td><td style="border: 1px solid #ddd; padding: 4px;">3.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, 2.236e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1144.5
- **Average Gradient Evaluations per Run:** 1143.5
- **Average Iterations per Run:** 570.8
- **Average Time per Run:** 0.026s
- **Function Evaluations per Second:** 43578.1
- **Iterations per Second:** 21731.9
### Resource Utilization
- **Total Function Evaluations:** 22890
- **Total Gradient Evaluations:** 22870
- **Total Computation Time:** 0.5s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/Barrier_10D_mu0.1_results.csv)
* [Convergence Plot](../plots/Barrier_10D_mu0.1.png)
* [Log Scale Convergence Plot](../plots/Barrier_10D_mu0.1_log.png)


---
*Detailed report for Adam-WeightDecay on Barrier_10D_mu0.1*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
