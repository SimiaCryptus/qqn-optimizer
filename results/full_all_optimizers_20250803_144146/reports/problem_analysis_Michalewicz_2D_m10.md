# Comprehensive Analysis: Michalewicz_2D_m10

[← Back to Main Report](../benchmark_report.md)

## Problem Overview

**Name:** Michalewicz_2D_m10
**Family:** Michalewicz
**Dimension:** 2
**Optimal Value:** -9.960e-1
**Total Runs:** 500

## Performance Rankings

<table style="border-collapse: collapse; width: 100%; margin: 20px 0;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 8px;">Rank</th>
<th style="border: 1px solid #ddd; padding: 8px;">Optimizer</th>
<th style="border: 1px solid #ddd; padding: 8px;">Mean Final Value</th>
<th style="border: 1px solid #ddd; padding: 8px;">Success Rate</th>
<th style="border: 1px solid #ddd; padding: 8px;">Mean Function Evals</th>
<th style="border: 1px solid #ddd; padding: 8px;">Mean Time (s)</th>
<th style="border: 1px solid #ddd; padding: 8px;">Detailed Report</th>
</tr>
<tr style="background-color: #d4edda; font-weight: bold;">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-4.98e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">50.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1642.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.032</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam.md">View Details</a></td>
</tr>
<tr style="background-color: #fff3cd;">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Fast</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-4.71e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">40.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1058.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Fast.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-AMSGrad</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-2.99e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">30.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">192.4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-AMSGrad.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Limited</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-2.50e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">466.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Limited.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-MoreThuente</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-2.50e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">752.4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-MoreThuente.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-9.99e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">10.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">802.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Nesterov</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-9.98e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">10.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">272.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Nesterov.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-AdaptiveMomentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-5.00e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">29.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-AdaptiveMomentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-8.97e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1145.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-4.01e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1074.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-2.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-StrongWolfe</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-4.01e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1192.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-StrongWolfe.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Momentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-3.25e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">42.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Momentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-1.99e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1278.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.034</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-1.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-3.24e-6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">423.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-1.16e-6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">252.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-8.81e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Conservative.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Precise</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-7.80e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Precise.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-GoldenSection</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-7.52e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1573.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.026</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-GoldenSection.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-7.20e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">838.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.014</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Conservative.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Standard</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-6.09e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Standard.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-5.33e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-CubicQuadraticInterpolation</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-5.13e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">19.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-CubicQuadraticInterpolation.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Adaptive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-4.40e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Adaptive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Robust</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-3.65e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">13.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Robust.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">-3.49e-7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">13.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-WeightDecay.md">View Details</a></td>
</tr>
</table>

## Convergence Analysis

![Convergence Plot](../plots/Michalewicz_2D_m10.png)
![Log Convergence Plot](../plots/Michalewicz_2D_m10_log.png)

## Raw Data

* [Problem-specific CSV data](../data/problems/Michalewicz_2D_m10_results.csv)
* [Convergence data (linear)](../data/convergence/Michalewicz_2D_m10_data.csv)
* [Convergence data (log)](../data/convergence/Michalewicz_2D_m10_log_data.csv)

---
*Generated on: 2025-08-03 15:33:47 UTC*
