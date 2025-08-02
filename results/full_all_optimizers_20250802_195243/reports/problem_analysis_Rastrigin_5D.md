# Comprehensive Analysis: Rastrigin_5D

[← Back to Main Report](../benchmark_report.md)

## Problem Overview

**Name:** Rastrigin_5D
**Family:** Rastrigin
**Dimension:** 5
**Optimal Value:** 2.040e1
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
<td style="border: 1px solid #ddd; padding: 8px;">GD-AdaptiveMomentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.14e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">65.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">52.4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-AdaptiveMomentum.md">View Details</a></td>
</tr>
<tr style="background-color: #fff3cd;">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Adaptive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.34e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">60.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">646.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Adaptive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Robust</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.24e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">55.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">103.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Robust.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.27e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">55.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">246.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.25e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">50.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">37.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-CubicQuadraticInterpolation</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.30e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">50.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">106.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-CubicQuadraticInterpolation.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.33e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">50.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">45.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-MoreThuente</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.77e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">50.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">165.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-MoreThuente.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Standard</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.31e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">45.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">168.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Standard.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Nesterov</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.37e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">45.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">41.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Nesterov.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Fast</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.33e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">40.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">48.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Fast.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.34e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">40.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">881.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Conservative.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-StrongWolfe</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.36e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">40.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">100.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-StrongWolfe.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-AMSGrad</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.38e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">40.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">719.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-AMSGrad.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-GoldenSection</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.31e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">35.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">266.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-GoldenSection.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Precise</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.39e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">35.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2558.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Precise.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Limited</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.84e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">35.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">115.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Limited.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.42e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">30.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">745.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.016</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.44e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">30.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.48e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">186.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-1.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.08e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">36.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-2.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Momentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.03e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">28.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Momentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6.91e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3002.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.020</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Conservative.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7.97e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3852.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.02e2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">98.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS.md">View Details</a></td>
</tr>
</table>

## Convergence Analysis

![Convergence Plot](../plots/Rastrigin_5D.png)
![Log Convergence Plot](../plots/Rastrigin_5D_log.png)

## Raw Data

* [Problem-specific CSV data](../data/problems/Rastrigin_5D_results.csv)
* [Convergence data (linear)](../data/convergence/Rastrigin_5D_data.csv)
* [Convergence data (log)](../data/convergence/Rastrigin_5D_log_data.csv)

---
*Generated on: 2025-08-02 20:44:04 UTC*
