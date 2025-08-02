# Detailed Analysis: Adam-AMSGrad on SVM_200samples_10features_C1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SVM_200samples_10features_C1
**Optimizer:** Adam-AMSGrad
**Problem Family:** SVM
**Dimension:** 10
**Success Threshold:** 6.860e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.861333e-1
* **Worst Final Value:** 6.861820e-1
* **Mean Final Value:** 6.861474e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.405e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.574e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1249</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2502</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.684e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">967</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1937</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1936</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.214</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.820e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2285</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2284</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.492e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1196</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2395</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.500</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.481e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">874</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1751</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1750</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.099</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.642e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">930</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1862</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.364e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1779</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1778</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.119</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.535e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1066</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2135</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2134</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.337</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.417e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2095</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2094</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.328</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.452e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">863</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1729</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1728</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.097</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.471e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">991</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1985</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1984</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.242</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.669e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1202</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2407</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2406</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.509</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.417e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">871</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1745</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1744</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.096</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.333e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">905</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1813</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1812</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.140</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.368e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1521</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1520</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.955</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.862e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.712e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">878</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1759</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1758</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.102</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.358e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">918</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1839</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1838</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.163</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.626e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">898</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1799</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1798</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.133</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.326e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1038</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2079</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2078</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.305</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 18 runs
- MaxFunctionEvaluations: 2 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 14)
**Final Value:** 6.861333e-1
**Final Gradient Norm:** 1.416536e-2
**Iterations:** 871
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.056e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.458e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.458e-1, -8.929e-2, -1.699e-1, -8.986e-2, -9.729e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.056e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.458e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.448e-1, -8.829e-2, -1.689e-1, -8.886e-2, -9.629e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.053e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.438e-1, -8.729e-2, -1.679e-1, -8.786e-2, -9.529e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.050e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.399e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.428e-1, -8.629e-2, -1.669e-1, -8.686e-2, -9.429e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.047e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.370e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.418e-1, -8.529e-2, -1.659e-1, -8.586e-2, -9.329e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">866</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.299e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.858e-2, 4.450e-2, 3.165e-2, 6.428e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">867</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.584e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.856e-2, 4.451e-2, 3.164e-2, 6.428e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">868</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.584e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.852e-2, 4.451e-2, 3.163e-2, 6.428e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">869</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.581e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.845e-2, 4.449e-2, 3.162e-2, 6.429e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">870</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.415e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.843e-2, 4.448e-2, 3.161e-2, 6.428e-2, 1.181e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 1996.0
- **Average Gradient Evaluations per Run:** 1995.1
- **Average Iterations per Run:** 996.5
- **Average Time per Run:** 1.254s
- **Function Evaluations per Second:** 1591.5
- **Iterations per Second:** 794.5
### Resource Utilization
- **Total Function Evaluations:** 39920
- **Total Gradient Evaluations:** 39902
- **Total Computation Time:** 25.1s
- **Function/Gradient Ratio:** 1.00
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](../data/problems/SVM_200samples_10features_C1_results.csv)
* [Convergence Plot](../plots/SVM_200samples_10features_C1.png)
* [Log Scale Convergence Plot](../plots/SVM_200samples_10features_C1_log.png)


---
*Detailed report for Adam-AMSGrad on SVM_200samples_10features_C1*
*Generated on: 2025-08-02 20:44:03 UTC*
*[← Back to Main Report](../benchmark_report.md)*
