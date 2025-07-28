# Detailed Analysis: QQN-CubicQuadraticInterpolation on NeuralNetwork_100samples_layers_5_10_3
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_5_10_3
**Optimizer:** QQN-CubicQuadraticInterpolation
**Problem Family:** ML Neural Networks
**Dimension:** 93
**Success Threshold:** 1.360e-1
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.867647e-1
* **Worst Final Value:** 2.056426e-1
* **Mean Final Value:** 1.936043e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.948e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.640e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.872e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.164e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.928e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.031e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.939e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.414e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.904e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.485e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.497</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.947e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.426e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.951e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.484e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.877e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.055e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.952e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.678e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.930e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.452e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.947e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.233e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.489</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.956e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.905e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.483</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.004e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.991e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.480</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.915e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.350e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.965e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.832e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.493</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.914e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.497</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.934e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.272e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.489</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.056e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.520e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">394</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">656</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.487</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.897e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.907e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.490</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.876e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.283e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.937e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.896e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.993e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.747e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.953e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.236e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.932e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.870e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.869e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.062e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.949e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.468e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.906e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.218e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.492</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2.035e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.671e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.868e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.167e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.494</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.955e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.044e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">404</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.500</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.986e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5.652e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.493</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.898e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.827e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.486</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.996e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.670e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.493</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.920e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.072e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.495</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.938e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.233e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.482</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.899e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.110e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.929e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.757e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.907e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.485e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">386</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">648</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.481</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.895e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.812e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.496</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.966e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.467e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">396</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">660</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.491</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- MaxFunctionEvaluations: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 9)
**Final Value:** 1.867647e-1
**Final Gradient Norm:** 1.166988e-1
**Iterations:** 12
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.349e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.235e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.308e-2, -5.051e-2, 1.978e-1, 3.690e-4, 1.045e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">3.349e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.235e0</td><td style="border: 1px solid #ddd; padding: 4px;">5.810e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[4.957e-2, -5.010e-2, 2.113e-1, -3.107e-3, 1.188e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.388e-1</td><td style="border: 1px solid #ddd; padding: 4px;">5.085e-1</td><td style="border: 1px solid #ddd; padding: 4px;">3.408e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.996e-2, -5.152e-2, 2.015e-1, -2.313e-3, 1.121e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.220e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.188e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">[3.181e-2, -5.249e-2, 1.987e-1, -1.428e-3, 1.102e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.202e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.821e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.363e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.506e-3, -5.496e-2, 1.910e-1, 2.816e-3, 1.058e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">8</td><td style="border: 1px solid #ddd; padding: 4px;">2.007e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.364e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.452e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-3.677e-1, -5.627e-2, 6.566e-2, 1.273e-1, 3.530e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">9</td><td style="border: 1px solid #ddd; padding: 4px;">1.980e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.360e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.090e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.072e-1, -5.737e-2, 3.855e-2, 1.588e-1, 2.746e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">10</td><td style="border: 1px solid #ddd; padding: 4px;">1.947e-1</td><td style="border: 1px solid #ddd; padding: 4px;">9.210e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.090e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.667e-1, -5.881e-2, 2.481e-2, 2.058e-1, 1.496e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">11</td><td style="border: 1px solid #ddd; padding: 4px;">1.909e-1</td><td style="border: 1px solid #ddd; padding: 4px;">8.167e-2</td><td style="border: 1px solid #ddd; padding: 4px;">1.090e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.189e-1, -6.218e-2, 4.377e-3, 3.108e-1, -3.559e-2, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">12</td><td style="border: 1px solid #ddd; padding: 4px;">1.868e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.167e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.090e0</td><td style="border: 1px solid #ddd; padding: 4px;">[-6.189e-1, -6.218e-2, 4.377e-3, 3.108e-1, -3.559e-2, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 394.3
- **Average Gradient Evaluations per Run:** 657.8
- **Average Iterations per Run:** 11.8
- **Average Time per Run:** 0.490s
- **Function Evaluations per Second:** 805.0
- **Iterations per Second:** 24.2
### Resource Utilization
- **Total Function Evaluations:** 15772
- **Total Gradient Evaluations:** 26312
- **Total Computation Time:** 19.6s
- **Function/Gradient Ratio:** 0.60
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/NeuralNetwork_100samples_layers_5_10_3_results.csv)
* [Convergence Plot](convergence_NeuralNetwork_100samples_layers_5_10_3.png)
* [Log Scale Convergence Plot](convergence_NeuralNetwork_100samples_layers_5_10_3_log.png)


---
*Detailed report for QQN-CubicQuadraticInterpolation on NeuralNetwork_100samples_layers_5_10_3*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
