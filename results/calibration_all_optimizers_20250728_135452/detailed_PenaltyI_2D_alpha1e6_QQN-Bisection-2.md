# Detailed Analysis: QQN-Bisection-2 on PenaltyI_2D_alpha1e6
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** PenaltyI_2D_alpha1e6
**Optimizer:** QQN-Bisection-2
**Problem Family:** PenaltyI
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.275940e0
* **Worst Final Value:** 6.624668e0
* **Mean Final Value:** 2.682532e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.524e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.469e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.764e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.949e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">278</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">181</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.013e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.472e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.930e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.779e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">261</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.822e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.217e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.780e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.335e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">215</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">151</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.559e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.497e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">173</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">131</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.767e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.367e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">152</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">121</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.556e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.494e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">257</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">171</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.879e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.086e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">299</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.886e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.863e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">299</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">191</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.321e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.299e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">320</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">201</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.707e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.336e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.138e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.706e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">385</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">231</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.625e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.094e2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">343</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">211</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.560e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.200e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.276e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.325e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">161</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.929e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.423e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.304e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.674e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">194</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">141</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.311e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.290e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">241</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Num Err</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- NumericalError: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 17)
**Final Value:** 1.275940e0
**Final Gradient Norm:** 2.325388e1
**Iterations:** 12
**Convergence Reason:** NumericalError

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.234e5</td><td style="border: 1px solid #ddd; padding: 4px;">7.025e5</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.149e-1, 4.806e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">1.234e5</td><td style="border: 1px solid #ddd; padding: 4px;">7.025e5</td><td style="border: 1px solid #ddd; padding: 4px;">4.768e-7</td><td style="border: 1px solid #ddd; padding: 4px;">[2.623e-1, 2.607e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.659e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.254e4</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.532e0, -3.912e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">5.474e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.480e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.530e0, -3.911e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.469e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.479e1</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.825e-1, -2.276e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">7</td><td style="border: 1px solid #ddd; padding: 4px;">1.847e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.718e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.285e-2, 1.856e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">1.504e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.453e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.281e-1, 2.258e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">1.360e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.332e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.498e-1, 2.450e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">1.293e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.274e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.551e-1, 2.497e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">11</td><td style="border: 1px solid #ddd; padding: 4px;">1.277e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-4</td><td style="border: 1px solid #ddd; padding: 4px;">[1.554e-1, 2.500e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 267.6
- **Average Gradient Evaluations per Run:** 175.9
- **Average Iterations per Run:** 13.5
- **Average Time per Run:** 0.005s
- **Function Evaluations per Second:** 50645.1
- **Iterations per Second:** 2554.5
### Resource Utilization
- **Total Function Evaluations:** 5353
- **Total Gradient Evaluations:** 3518
- **Total Computation Time:** 0.1s
- **Function/Gradient Ratio:** 1.52
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 20
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/PenaltyI_2D_alpha1e6_results.csv)
* [Convergence Plot](convergence_PenaltyI_2D_alpha1e6.png)
* [Log Scale Convergence Plot](convergence_PenaltyI_2D_alpha1e6_log.png)


---
*Detailed report for QQN-Bisection-2 on PenaltyI_2D_alpha1e6*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
