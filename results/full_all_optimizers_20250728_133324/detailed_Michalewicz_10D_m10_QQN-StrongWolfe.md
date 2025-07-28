# Detailed Analysis: QQN-StrongWolfe on Michalewicz_10D_m10
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Michalewicz_10D_m10
**Optimizer:** QQN-StrongWolfe
**Problem Family:** Highly Multimodal
**Dimension:** 10
**Success Threshold:** -6.260e0
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** -6.259820e0
* **Worst Final Value:** -1.190895e0
* **Mean Final Value:** -3.921353e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.863e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.382e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">573</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">449</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.791e-7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">291</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">268</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.010</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Grad Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.301e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.395e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">258</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.640e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.973e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">602</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">410</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.595e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.503e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.404e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.954e-6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">444</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.435e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.828e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">573</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-1.191e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.551e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">612</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.136e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.929e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">572</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">447</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.564e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.789e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">618</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">413</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-2.723e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.400e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">635</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">402</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.677e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.638e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">576</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.694e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.317e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">465</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.978e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.666e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">466</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-6.260e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.344e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">550</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">457</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.534e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.569e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">42</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">519</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">484</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.611e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.682e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">551</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">461</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-3.617e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.703e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">617</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-5.293e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.077e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">-4.653e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.020e-4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">227</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">203</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- FunctionTolerance: 3 runs
- MaxFunctionEvaluations: 16 runs
- GradientTolerance: 1 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 2)
**Final Value:** -6.259820e0
**Final Gradient Norm:** 5.790566e-7
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.994e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 9.557e-1, 9.178e-1, 9.568e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">-1.994e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.014e1</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 9.596e-1, 9.665e-1, 1.129e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">-2.468e0</td><td style="border: 1px solid #ddd; padding: 4px;">9.551e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.125e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 9.638e-1, 1.085e0, 1.016e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">-3.030e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.242e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.562e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 9.662e-1, 1.143e0, 9.659e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">-3.368e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.463e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 9.653e-1, 1.072e0, 1.040e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">22</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.024e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">23</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.233e-4</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">24</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.738e-5</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">25</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">8.283e-6</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">26</td><td style="border: 1px solid #ddd; padding: 4px;">-6.260e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.791e-7</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[8.948e-1, 7.398e-1, 1.285e0, 1.114e0, 9.967e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 526.8
- **Average Gradient Evaluations per Run:** 411.2
- **Average Iterations per Run:** 28.3
- **Average Time per Run:** 0.017s
- **Function Evaluations per Second:** 31775.7
- **Iterations per Second:** 1707.0
### Resource Utilization
- **Total Function Evaluations:** 10536
- **Total Gradient Evaluations:** 8225
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.28
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Michalewicz_10D_m10_results.csv)
* [Convergence Plot](convergence_Michalewicz_10D_m10.png)
* [Log Scale Convergence Plot](convergence_Michalewicz_10D_m10_log.png)


---
*Detailed report for QQN-StrongWolfe on Michalewicz_10D_m10*
*Generated on: 2025-07-28 13:54:23 UTC*
*[← Back to Main Report](benchmark_report.md)*
