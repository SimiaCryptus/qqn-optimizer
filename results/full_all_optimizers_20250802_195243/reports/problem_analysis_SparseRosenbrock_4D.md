# Comprehensive Analysis: SparseRosenbrock_4D

[← Back to Main Report](../benchmark_report.md)

## Problem Overview

**Name:** SparseRosenbrock_4D
**Family:** SparseRosenbrock
**Dimension:** 4
**Optimal Value:** 1.000e-6
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
<td style="border: 1px solid #ddd; padding: 8px;">QQN-CubicQuadraticInterpolation</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.85e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">45.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1743.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.073</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-CubicQuadraticInterpolation.md">View Details</a></td>
</tr>
<tr style="background-color: #fff3cd;">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-StrongWolfe</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.39e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2429.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-StrongWolfe.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-MoreThuente</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.75e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">20.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2601.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-MoreThuente.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.56e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2290.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.057</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-1.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-GoldenSection</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.33e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4306.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.085</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-GoldenSection.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.42e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">193.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-2.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-AdaptiveMomentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.31e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">56.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-AdaptiveMomentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.88e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">510.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.013</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Fast</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.92e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">243.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Fast.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.06e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2502.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Limited</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.33e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3916.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.042</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Limited.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.04e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">105.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Nesterov</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.38e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">61.4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Nesterov.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-AMSGrad</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.39e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2394.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-AMSGrad.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6.07e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">843.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.018</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Robust</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7.53e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">672.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Robust.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Standard</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8.25e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">168.3</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Standard.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Adaptive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8.31e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">954.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Adaptive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8.72e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">57.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Momentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.10e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">21.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Momentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Precise</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.16e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1739.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Precise.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.72e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2998.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.019</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Conservative.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6.34e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3852.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.029</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">9.62e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">143.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.27e2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3413.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.039</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Conservative.md">View Details</a></td>
</tr>
</table>

## Convergence Analysis

![Convergence Plot](../plots/SparseRosenbrock_4D.png)
![Log Convergence Plot](../plots/SparseRosenbrock_4D_log.png)

## Raw Data

* [Problem-specific CSV data](../data/problems/SparseRosenbrock_4D_results.csv)
* [Convergence data (linear)](../data/convergence/SparseRosenbrock_4D_data.csv)
* [Convergence data (log)](../data/convergence/SparseRosenbrock_4D_log_data.csv)

---
*Generated on: 2025-08-02 20:44:04 UTC*
