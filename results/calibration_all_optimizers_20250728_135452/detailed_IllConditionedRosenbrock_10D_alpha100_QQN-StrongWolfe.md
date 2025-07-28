# Detailed Analysis: QQN-StrongWolfe on IllConditionedRosenbrock_10D_alpha100
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** IllConditionedRosenbrock_10D_alpha100
**Optimizer:** QQN-StrongWolfe
**Problem Family:** IllConditionedRosenbrock
**Dimension:** 10
**Success Threshold:** 1.000e-6
**Total Runs:** 20
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 8.590210e-1
* **Worst Final Value:** 9.520888e0
* **Mean Final Value:** 4.709021e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.590e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.187e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">548</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">464</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.150e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.521e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">553</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.088e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.658e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">43</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">561</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.881e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.785e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">588</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.548e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.074e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">571</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.198e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.811e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">564</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">450</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.652e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.438e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">474</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.456e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.676e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">44</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">528</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.291e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.592e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">584</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">430</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.853e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.322e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">592</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">448</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.226e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.664e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">565</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">442</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.079e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.907e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">589</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">436</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.264e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.816e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">569</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">466</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.324e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.113e1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">590</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">426</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.350e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.376e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">574</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">462</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.009e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.929e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">587</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">456</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.521e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.468e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">563</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">452</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.154e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.600e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">576</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">440</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.657e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.691e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">558</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">478</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.130e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.114e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">540</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">472</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (20 out of 20)

**Failure Reasons:**
- MaxFunctionEvaluations: 20 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 1)
**Final Value:** 8.590210e-1
**Final Gradient Norm:** 1.187359e1
**Iterations:** 44
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.089e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.169e3</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.177e0, 1.061e0, -1.378e0, 9.795e-1, -1.054e0, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.089e3</td><td style="border: 1px solid #ddd; padding: 4px;">2.169e3</td><td style="border: 1px solid #ddd; padding: 4px;">1.074e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.008e0, -1.015e-2, -2.913e-1, 3.294e-1, -5.319e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.020e2</td><td style="border: 1px solid #ddd; padding: 4px;">5.667e2</td><td style="border: 1px solid #ddd; padding: 4px;">3.906e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[6.242e-1, 8.042e-1, -1.649e-1, -1.861e-1, 1.416e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.708e2</td><td style="border: 1px solid #ddd; padding: 4px;">4.593e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[8.275e-1, 1.344e-1, 1.835e-1, -1.138e-1, 1.240e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">6.893e1</td><td style="border: 1px solid #ddd; padding: 4px;">2.356e2</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[4.324e-1, 1.566e-1, 1.255e-1, -2.073e-2, 2.290e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">40</td><td style="border: 1px solid #ddd; padding: 4px;">1.518e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.578e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.896e-1, 9.852e-1, 9.732e-1, 9.455e-1, 8.948e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">41</td><td style="border: 1px solid #ddd; padding: 4px;">1.330e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.306e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.933e-1, 9.853e-1, 9.745e-1, 9.570e-1, 9.150e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">42</td><td style="border: 1px solid #ddd; padding: 4px;">1.175e0</td><td style="border: 1px solid #ddd; padding: 4px;">7.777e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[9.976e-1, 9.849e-1, 9.752e-1, 9.622e-1, 9.172e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">43</td><td style="border: 1px solid #ddd; padding: 4px;">1.045e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.133e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.003e0, 9.941e-1, 9.856e-1, 9.774e-1, 9.530e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">44</td><td style="border: 1px solid #ddd; padding: 4px;">8.590e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.187e1</td><td style="border: 1px solid #ddd; padding: 4px;">1.953e-3</td><td style="border: 1px solid #ddd; padding: 4px;">[1.003e0, 9.941e-1, 9.856e-1, 9.774e-1, 9.530e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 568.1
- **Average Gradient Evaluations per Run:** 454.1
- **Average Iterations per Run:** 34.6
- **Average Time per Run:** 0.016s
- **Function Evaluations per Second:** 36249.2
- **Iterations per Second:** 2210.7
### Resource Utilization
- **Total Function Evaluations:** 11363
- **Total Gradient Evaluations:** 9082
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 1.25
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/IllConditionedRosenbrock_10D_alpha100_results.csv)
* [Convergence Plot](convergence_IllConditionedRosenbrock_10D_alpha100.png)
* [Log Scale Convergence Plot](convergence_IllConditionedRosenbrock_10D_alpha100_log.png)


---
*Detailed report for QQN-StrongWolfe on IllConditionedRosenbrock_10D_alpha100*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
