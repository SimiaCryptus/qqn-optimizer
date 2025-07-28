# Detailed Analysis: QQN-MoreThuente on SVM_100samples_5features_C1
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** SVM_100samples_5features_C1
**Optimizer:** QQN-MoreThuente
**Problem Family:** ML Classification
**Dimension:** 5
**Success Threshold:** 6.430e-1
**Total Runs:** 20
**Successful Runs:** 10 (50.0%)

### Quick Statistics
* **Best Final Value:** 6.429278e-1
* **Worst Final Value:** 6.429642e-1
* **Mean Final Value:** 6.429350e-1
* **Success Rate:** 50.0%


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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.081e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">556</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.038e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">384</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">493</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.160</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.049e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">375</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">473</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.154</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.062e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">290</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">381</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.122</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.039e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">547</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.177</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.039e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">453</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.060e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">431</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.038e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">451</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">555</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.114e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">418</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">523</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.038e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">461</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">560</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.186</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.126e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">393</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">494</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.162</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.212e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">397</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.132</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.038e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">562</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.184</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.430e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.276e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">357</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.147</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.239e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">417</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.115e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">559</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.183</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.215e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">546</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.182</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.430e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.337e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">435</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.185</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #d4edda;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.038e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">295</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">356</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.120</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✓</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.429e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.042e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">535</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Successful Runs (10 out of 20)
- **Average Iterations to Convergence:** 27.1
- **Average Function Evaluations:** 367.4
- **Average Time to Convergence:** 0.152s
- **Fastest Convergence:** 22 iterations (0.120s)
- **Slowest Convergence:** 32 iterations (0.177s)
### Failed Runs (10 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 10 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** 6.429278e-1
**Final Gradient Norm:** 1.038338e-2
**Iterations:** 28
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.064e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.012e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.851e-1, -1.601e-1, -1.066e-2, -9.456e-2, 1.510e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.064e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.012e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.931e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.789e-2, 7.133e-2, 2.889e-1, 3.029e-1, 3.681e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.620e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.618e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.617e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[5.251e-2, 8.735e-2, 3.025e-1, 3.294e-1, 3.672e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.532e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.709e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.073e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.671e-3, 1.280e-1, 3.110e-1, 3.727e-1, 3.520e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.439e-1</td><td style="border: 1px solid #ddd; padding: 4px;">4.772e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.937e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-8.988e-3, 1.451e-1, 3.125e-1, 3.991e-1, 3.469e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">24</td><td style="border: 1px solid #ddd; padding: 4px;">6.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.038e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.745e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.450e-2, 1.366e-1, 3.199e-1, 4.008e-1, 3.445e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">25</td><td style="border: 1px solid #ddd; padding: 4px;">6.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.038e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.673e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.450e-2, 1.366e-1, 3.199e-1, 4.008e-1, 3.445e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">26</td><td style="border: 1px solid #ddd; padding: 4px;">6.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.038e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.609e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.450e-2, 1.366e-1, 3.199e-1, 4.008e-1, 3.445e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">27</td><td style="border: 1px solid #ddd; padding: 4px;">6.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.038e-2</td><td style="border: 1px solid #ddd; padding: 4px;">9.284e-10</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.450e-2, 1.366e-1, 3.199e-1, 4.008e-1, 3.445e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">28</td><td style="border: 1px solid #ddd; padding: 4px;">6.429e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.038e-2</td><td style="border: 1px solid #ddd; padding: 4px;">8.685e-10</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.450e-2, 1.366e-1, 3.199e-1, 4.008e-1, 3.445e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 409.4
- **Average Gradient Evaluations per Run:** 510.2
- **Average Iterations per Run:** 29.2
- **Average Time per Run:** 0.168s
- **Function Evaluations per Second:** 2442.2
- **Iterations per Second:** 174.2
### Resource Utilization
- **Total Function Evaluations:** 8188
- **Total Gradient Evaluations:** 10205
- **Total Computation Time:** 3.4s
- **Function/Gradient Ratio:** 0.80
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/SVM_100samples_5features_C1_results.csv)
* [Convergence Plot](convergence_SVM_100samples_5features_C1.png)
* [Log Scale Convergence Plot](convergence_SVM_100samples_5features_C1_log.png)


---
*Detailed report for QQN-MoreThuente on SVM_100samples_5features_C1*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
