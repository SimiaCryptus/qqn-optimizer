# Detailed Analysis: GD-WeightDecay on SVM_200samples_10features_C1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SVM_200samples_10features_C1
**Optimizer:** GD-WeightDecay
**Problem Family:** SVM
**Dimension:** 10
**Success Threshold:** 6.860e-1
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 6.861311e-1
* **Worst Final Value:** 6.861333e-1
* **Mean Final Value:** 6.861324e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.576e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.180</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.256e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.261e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">170</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">342</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.642e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">179</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">360</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.288e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">201</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">204</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.198</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.299e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.295e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">366</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.486e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">165</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">326</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.159</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.292e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">358</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.478e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.169</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.583e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">158</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">318</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.157</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.417e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">344</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.580e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">175</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">178</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">352</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.174</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.073e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">338</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.166</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.415e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">168</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">338</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.167</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.478e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.481e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">176</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">348</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.586e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">188</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">378</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.412e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">298</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.148</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.861e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.403e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">192</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">195</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">No</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 5)
**Final Value:** 6.861311e-1
**Final Gradient Norm:** 1.287600e-2
**Iterations:** 201
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.757e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.587e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.476e-2, 1.375e-1, -1.288e-1, 1.967e-2, -1.133e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">9.757e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.587e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.407e-2, 1.372e-1, -1.278e-1, 1.991e-2, -1.118e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">9.720e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.544e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.282e-2, 1.368e-1, -1.259e-1, 2.034e-2, -1.091e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">9.655e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.467e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.114e-2, 1.363e-1, -1.234e-1, 2.092e-2, -1.056e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">9.567e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.363e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.912e-2, 1.357e-1, -1.204e-1, 2.161e-2, -1.012e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">196</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.289e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.816e-2, 4.439e-2, 3.095e-2, 6.388e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">197</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.288e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.817e-2, 4.441e-2, 3.100e-2, 6.392e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">198</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.575e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.815e-2, 4.439e-2, 3.101e-2, 6.395e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">199</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.634e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.813e-2, 4.437e-2, 3.096e-2, 6.392e-2, 1.181e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">200</td><td style="border: 1px solid #ddd; padding: 4px;">6.861e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.286e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[5.811e-2, 4.437e-2, 3.095e-2, 6.393e-2, 1.182e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 177.0
- **Average Gradient Evaluations per Run:** 350.0
- **Average Iterations per Run:** 174.0
- **Average Time per Run:** 0.172s
- **Function Evaluations per Second:** 1027.3
- **Iterations per Second:** 1009.9
### Resource Utilization
- **Total Function Evaluations:** 3540
- **Total Gradient Evaluations:** 7000
- **Total Computation Time:** 3.4s
- **Function/Gradient Ratio:** 0.51
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
*Detailed report for GD-WeightDecay on SVM_200samples_10features_C1*
*Generated on: 2025-08-03 15:33:46 UTC*
*[← Back to Main Report](../benchmark_report.md)*
