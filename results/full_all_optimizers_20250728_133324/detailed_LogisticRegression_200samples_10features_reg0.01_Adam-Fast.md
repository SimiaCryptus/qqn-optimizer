# Detailed Analysis: Adam-Fast on LogisticRegression_200samples_10features_reg0.01
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** LogisticRegression_200samples_10features_reg0.01
**Optimizer:** Adam-Fast
**Problem Family:** ML Regression
**Dimension:** 10
**Success Threshold:** 3.230e-1
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 3.240842e-1
* **Worst Final Value:** 3.251354e-1
* **Mean Final Value:** 3.244954e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.249e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.965e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.321e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.246e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.432e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.471e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.246e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.079e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.556e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.022e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.388e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.733e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.558e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.053</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.579e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.247e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.163e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.570e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.046</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.954e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.550e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.251e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.008e-2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.936e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.246e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.930e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.243e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.079e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.798e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.630e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.047</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.251e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.735e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.244e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.010e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.375e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.693e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.784e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.000e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.686e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.843e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.157e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">73</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">72</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.048</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.247e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.138e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.506e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.241e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.567e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">79</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">78</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.052</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.574e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.051</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.246e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.214e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">81</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">80</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.054</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.242e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.074e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">77</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">76</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.247e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.838e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.049</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.248e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.689e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">71</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">70</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.045</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9.417e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3.245e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8.949e-3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">75</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">74</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.050</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- FunctionTolerance: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 13)
**Final Value:** 3.240842e-1
**Final Gradient Norm:** 7.567418e-3
**Iterations:** 38
**Convergence Reason:** FunctionTolerance

#### Parameter Evolution (Selected Iterations)

<table style="border-collapse: collapse; width: 100%; margin: 20px 0; font-size: 11px;">
<tr style="background-color: #f2f2f2;">
<th style="border: 1px solid #ddd; padding: 4px;">Iteration</th>
<th style="border: 1px solid #ddd; padding: 4px;">Function Value</th>
<th style="border: 1px solid #ddd; padding: 4px;">Gradient Norm</th>
<th style="border: 1px solid #ddd; padding: 4px;">Step Size</th>
<th style="border: 1px solid #ddd; padding: 4px;">Parameters (first 5)</th>
</tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.351e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.359e-1</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.102e-1, 1.965e-2, -4.502e-2, -8.228e-2, 1.018e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">7.351e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.359e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.050e-2, 1.196e-1, 5.498e-2, 1.772e-2, 2.018e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">6.792e-1</td><td style="border: 1px solid #ddd; padding: 4px;">2.152e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.598e-2, 2.150e-1, 1.505e-1, 1.164e-1, 3.002e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">6.297e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.962e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.513e-1, 2.987e-1, 2.349e-1, 2.126e-1, 3.954e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">5.875e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.790e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[2.154e-1, 3.619e-1, 2.997e-1, 3.044e-1, 4.851e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">33</td><td style="border: 1px solid #ddd; padding: 4px;">3.236e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.153e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.412e-1, 4.816e-1, 6.320e-1, 8.309e-1, 9.557e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">34</td><td style="border: 1px solid #ddd; padding: 4px;">3.237e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.309e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.387e-1, 4.847e-1, 6.411e-1, 8.300e-1, 9.589e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">35</td><td style="border: 1px solid #ddd; padding: 4px;">3.238e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.560e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.388e-1, 4.902e-1, 6.526e-1, 8.269e-1, 9.593e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">36</td><td style="border: 1px solid #ddd; padding: 4px;">3.239e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.857e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.416e-1, 4.973e-1, 6.655e-1, 8.221e-1, 9.572e-1, ...]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">37</td><td style="border: 1px solid #ddd; padding: 4px;">3.240e-1</td><td style="border: 1px solid #ddd; padding: 4px;">7.191e-3</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[3.469e-1, 5.052e-1, 6.788e-1, 8.164e-1, 9.527e-1, ...]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 76.5
- **Average Gradient Evaluations per Run:** 75.5
- **Average Iterations per Run:** 36.7
- **Average Time per Run:** 0.050s
- **Function Evaluations per Second:** 1531.9
- **Iterations per Second:** 735.9
### Resource Utilization
- **Total Function Evaluations:** 3058
- **Total Gradient Evaluations:** 3018
- **Total Computation Time:** 2.0s
- **Function/Gradient Ratio:** 1.01
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/LogisticRegression_200samples_10features_reg0.01_results.csv)
* [Convergence Plot](convergence_LogisticRegression_200samples_10features_reg0.01.png)
* [Log Scale Convergence Plot](convergence_LogisticRegression_200samples_10features_reg0.01_log.png)


---
*Detailed report for Adam-Fast on LogisticRegression_200samples_10features_reg0.01*
*Generated on: 2025-07-28 13:54:24 UTC*
*[← Back to Main Report](benchmark_report.md)*
