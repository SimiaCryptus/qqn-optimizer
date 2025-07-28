# Detailed Analysis: GD on NeuralNetwork_100samples_layers_10_20_5
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** NeuralNetwork_100samples_layers_10_20_5
**Optimizer:** GD
**Problem Family:** ML Neural Networks
**Dimension:** 325
**Success Threshold:** 3.540e-2
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.489363e-1
* **Worst Final Value:** 1.544586e-1
* **Mean Final Value:** 1.517867e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.520e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.069e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.508e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.046e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.522e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.157e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.502e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.061e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.513e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.084e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.532e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.119e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.545e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.012e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.509e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.106e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.490e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.198e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.503e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.155e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.519e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.085e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.510e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.079e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.535e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.135e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.895</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.544e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.049e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.489e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.218e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.517e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.168e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.884</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.532e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.981e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.521e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.078e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.513e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.191e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.883</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.538e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.053e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.885</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.504e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.259e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.521e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.154e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.512e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.125e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.889</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.518e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.132e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.510e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.114e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.506e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.003e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.888</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.535e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.128e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.506e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.122e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.524e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.055e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.497e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.173e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.515e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.086e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.524e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.160e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.518e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.065e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.892</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.519e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.148e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.900</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.525e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.127e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.530e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.101e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.893</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.526e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.025e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.887</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.519e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.048e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.891</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.527e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.112e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.890</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.517e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.207e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">332</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">335</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">668</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.886</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Max Func</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- MaxFunctionEvaluations: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 15)
**Final Value:** 1.489363e-1
**Final Gradient Norm:** 1.217553e-1
**Iterations:** 332
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.272e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.402e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[5.670e-2, -1.658e-1, 4.182e-2, -1.906e-1, -2.252e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.272e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.402e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.648e-2, -1.655e-1, 4.179e-2, -1.905e-1, -2.250e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">2.233e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.360e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.628e-2, -1.652e-1, 4.177e-2, -1.905e-1, -2.247e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">2.196e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.318e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.609e-2, -1.649e-1, 4.175e-2, -1.905e-1, -2.245e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">2.162e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.278e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[5.589e-2, -1.647e-1, 4.173e-2, -1.904e-1, -2.243e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">328</td><td style="border: 1px solid #ddd; padding: 4px;">1.491e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.670e-2, -1.141e-1, 2.716e-2, -1.873e-1, -2.312e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">329</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.663e-2, -1.140e-1, 2.712e-2, -1.873e-1, -2.312e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">330</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.222e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.655e-2, -1.139e-1, 2.708e-2, -1.873e-1, -2.313e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">331</td><td style="border: 1px solid #ddd; padding: 4px;">1.490e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.221e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.648e-2, -1.138e-1, 2.704e-2, -1.873e-1, -2.313e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">332</td><td style="border: 1px solid #ddd; padding: 4px;">1.489e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.218e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-2</td><td style="border: 1px solid #ddd; padding: 4px;">[1.648e-2, -1.138e-1, 2.704e-2, -1.873e-1, -2.313e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 335.0
- **Average Gradient Evaluations per Run:** 668.0
- **Average Iterations per Run:** 332.0
- **Average Time per Run:** 0.889s
- **Function Evaluations per Second:** 376.9
- **Iterations per Second:** 373.5
### Resource Utilization
- **Total Function Evaluations:** 13400
- **Total Gradient Evaluations:** 26720
- **Total Computation Time:** 35.6s
- **Function/Gradient Ratio:** 0.50
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/NeuralNetwork_100samples_layers_10_20_5_results.csv)
* [Convergence Plot](convergence_NeuralNetwork_100samples_layers_10_20_5.png)
* [Log Scale Convergence Plot](convergence_NeuralNetwork_100samples_layers_10_20_5_log.png)


---
*Detailed report for GD on NeuralNetwork_100samples_layers_10_20_5*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
