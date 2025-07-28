# Detailed Analysis: L-BFGS-Aggressive on Ackley_5D_a20_b0.2_c6.28e0
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Ackley_5D_a20_b0.2_c6.28e0
**Optimizer:** L-BFGS-Aggressive
**Problem Family:** Highly Multimodal
**Dimension:** 5
**Success Threshold:** 3.570e0
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.091579e0
* **Worst Final Value:** 4.060007e0
* **Mean Final Value:** 3.398576e0
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.832e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.946e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.794e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.371e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">777</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.981e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.440e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.867e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.969e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.699e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.030e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.058e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.998e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.742e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.378e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.855e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.006e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.780e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.739e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.757e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.509e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.790e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.890e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.826e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.948e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.750e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.699e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.798e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.923e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.671e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.750e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.725e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.213e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.744e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.636e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.712e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.397e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.722e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.804e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.011</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.764e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.870e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">778</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.783e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.821e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.825e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.063e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.800e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.737e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.799e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.969e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.444e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.783e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.024e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.005e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.918e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">769</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.092e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.648e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.867e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.279e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.725e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.290e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.532e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.820e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.009</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.060e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4.078e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">772</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.005</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.817e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.756e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.827e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.021e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.714e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.186e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.861e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.015e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.051e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.190e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">775</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.008</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.745e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.570e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">771</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.004</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.738e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.355e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">770</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">233</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.006</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.453e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.992e0</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">774</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">236</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.007</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- MaxFunctionEvaluations: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 8)
**Final Value:** 1.091579e0
**Final Gradient Norm:** 5.647589e0
**Iterations:** 77
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.763e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.133e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.173e0, 1.187e0, 9.223e-1, 1.090e0, 1.131e0]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">4.763e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.133e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.364e-1, -2.695e-1, 1.220e0, 1.295e-1, -8.091e-2]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">3.476e0</td><td style="border: 1px solid #ddd; padding: 4px;">4.715e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.777e-1, 2.532e-1, 4.060e-1, -2.322e-1, 1.595e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.870e0</td><td style="border: 1px solid #ddd; padding: 4px;">3.922e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.500e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.987e-1, -2.109e-1, -3.306e-2, 2.158e-1, -1.949e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.922e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.407e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.250e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.316e-1, 1.310e-1, 3.345e-2, -1.303e-1, 1.316e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">73</td><td style="border: 1px solid #ddd; padding: 4px;">1.092e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.316e-1, 1.310e-1, 3.345e-2, -1.303e-1, 1.316e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">74</td><td style="border: 1px solid #ddd; padding: 4px;">1.092e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.316e-1, 1.310e-1, 3.345e-2, -1.303e-1, 1.316e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">75</td><td style="border: 1px solid #ddd; padding: 4px;">1.092e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.316e-1, 1.310e-1, 3.345e-2, -1.303e-1, 1.316e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">76</td><td style="border: 1px solid #ddd; padding: 4px;">1.092e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.316e-1, 1.310e-1, 3.345e-2, -1.303e-1, 1.316e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">77</td><td style="border: 1px solid #ddd; padding: 4px;">1.092e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.648e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-8</td><td style="border: 1px solid #ddd; padding: 4px;">[1.316e-1, 1.310e-1, 3.345e-2, -1.303e-1, 1.316e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 771.5
- **Average Gradient Evaluations per Run:** 233.7
- **Average Iterations per Run:** 76.2
- **Average Time per Run:** 0.007s
- **Function Evaluations per Second:** 117251.6
- **Iterations per Second:** 11583.8
### Resource Utilization
- **Total Function Evaluations:** 30862
- **Total Gradient Evaluations:** 9347
- **Total Computation Time:** 0.3s
- **Function/Gradient Ratio:** 3.30
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Ackley_5D_a20_b0.2_c6.28e0_results.csv)
* [Convergence Plot](convergence_Ackley_5D_a20_b0.2_c6.28e0.png)
* [Log Scale Convergence Plot](convergence_Ackley_5D_a20_b0.2_c6.28e0_log.png)


---
*Detailed report for L-BFGS-Aggressive on Ackley_5D_a20_b0.2_c6.28e0*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
