# Comprehensive Analysis: IllConditionedRosenbrock_2D_alpha100

[← Back to Main Report](../benchmark_report.md)

## Problem Overview

**Name:** IllConditionedRosenbrock_2D_alpha100
**Family:** IllConditionedRosenbrock
**Dimension:** 2
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
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.69e-4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">80.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1800.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.030</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Conservative.md">View Details</a></td>
</tr>
<tr style="background-color: #fff3cd;">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-MoreThuente</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.52e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">65.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1487.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-MoreThuente.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-CubicQuadraticInterpolation</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.48e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">35.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1722.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.070</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-CubicQuadraticInterpolation.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.88e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">479.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.012</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-2.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-StrongWolfe</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.46e-2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">20.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2343.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.072</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-StrongWolfe.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-Bisection-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.65e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2369.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-Bisection-1.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 8px;">QQN-GoldenSection</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.25e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4459.4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.081</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_QQN-GoldenSection.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-AdaptiveMomentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">8.18e-1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">49.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-AdaptiveMomentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.22e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2502.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.23e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">854.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.021</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Nesterov</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.49e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">46.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Nesterov.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Fast</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.13e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">313.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Fast.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.65e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">58.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-AMSGrad</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.83e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">678.1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.015</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-AMSGrad.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Limited</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.93e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2251.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.025</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Limited.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-Robust</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.04e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">419.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-Robust.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 8px;">Adam-WeightDecay</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.11e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">231.9</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Adam-WeightDecay.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Adaptive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.12e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">494.4</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.003</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Adaptive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Standard</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.18e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">89.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Standard.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">4.66e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">27.6</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.000</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 8px;">GD-Momentum</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">5.61e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">23.8</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_GD-Momentum.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Precise</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">7.26e0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">946.2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Precise.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 8px;">Trust Region-Conservative</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2.84e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">2770.7</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.017</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_Trust_Region-Conservative.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS-Aggressive</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3.18e1</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">3852.0</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.027</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS-Aggressive.md">View Details</a></td>
</tr>
<tr style="">
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 8px;">L-BFGS</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">1.36e2</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.0%</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">121.5</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;">0.002</td>
<td style="border: 1px solid #ddd; padding: 8px; text-align: center;"><a href="detailed_problem_name_L-BFGS.md">View Details</a></td>
</tr>
</table>

## Convergence Analysis

![Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100.png)
![Log Convergence Plot](../plots/IllConditionedRosenbrock_2D_alpha100_log.png)

## Raw Data

* [Problem-specific CSV data](../data/problems/IllConditionedRosenbrock_2D_alpha100_results.csv)
* [Convergence data (linear)](../data/convergence/IllConditionedRosenbrock_2D_alpha100_data.csv)
* [Convergence data (log)](../data/convergence/IllConditionedRosenbrock_2D_alpha100_log_data.csv)

---
*Generated on: 2025-08-02 20:44:04 UTC*
