# Detailed Analysis: Adam-Fast on Sphere_2D
[← Back to Main Report](benchmark_report.md)
## Executive Summary
**Problem:** Sphere_2D
**Optimizer:** Adam-Fast
**Problem Family:** Convex Unimodal
**Dimension:** 2
**Success Threshold:** 1.000e-6
**Total Runs:** 40
**Successful Runs:** 0 (0.0%)

### Quick Statistics
* **Best Final Value:** 1.100157e-1
* **Worst Final Value:** 1.353263e-1
* **Mean Final Value:** 1.255304e-1
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
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.234e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.024e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">2</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.353e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.357e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">3</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.100e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.634e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">4</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.285e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.168e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">5</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.293e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.193e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.277e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.148e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.253e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.079e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">8</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.240e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.042e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">9</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.179e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.868e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">10</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.301e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.213e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">11</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.178e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.865e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">12</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.268e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.123e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">13</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.325e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.281e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">14</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.329e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.290e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.329e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.290e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.344e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.332e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.256e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.088e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.305e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.226e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.211e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.959e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">20</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.183e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.879e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">21</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.213e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.967e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">15</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">22</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.318e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.260e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">23</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.200e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.929e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">24</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.323e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.276e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">25</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.275e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.143e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">26</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.263e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.108e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">27</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.183e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.878e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">28</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.174e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.854e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">29</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.236e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.031e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">30</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.312e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.246e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">31</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.224e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.997e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">32</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.220e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.985e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">33</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.235e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.029e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.158e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">6.805e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.283e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.165e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">17</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">36</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.258e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.093e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">37</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.236e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.031e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.229e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.012e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">16</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">35</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">34</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.340e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.322e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">19</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">41</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
<tr style="background-color: #f8d7da;">
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">40</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">1.287e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">7.174e-1</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">18</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">39</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">38</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">0.001</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">✗</td>
<td style="border: 1px solid #ddd; padding: 6px; text-align: center;">Func Tol</td>
</tr>
</table>

## Convergence Analysis

### Failed Runs (40 out of 40)

**Failure Reasons:**
- FunctionTolerance: 40 runs

## Parameter Evolution Analysis

### Best Run Analysis (Run 3)
**Final Value:** 1.100157e-1
**Final Gradient Norm:** 6.633721e-1
**Iterations:** 18
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
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.096e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.895e0</td><td style="border: 1px solid #ddd; padding: 4px;">0.000e0</td><td style="border: 1px solid #ddd; padding: 4px;">[1.187e0, 8.291e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">0</td><td style="border: 1px solid #ddd; padding: 4px;">2.096e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.895e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[1.087e0, 7.291e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">1</td><td style="border: 1px solid #ddd; padding: 4px;">1.712e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.617e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[9.871e-1, 6.296e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">2</td><td style="border: 1px solid #ddd; padding: 4px;">1.371e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.342e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[8.880e-1, 5.312e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">3</td><td style="border: 1px solid #ddd; padding: 4px;">1.071e0</td><td style="border: 1px solid #ddd; padding: 4px;">2.069e0</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[7.898e-1, 4.343e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">13</td><td style="border: 1px solid #ddd; padding: 4px;">4.275e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.135e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-4.166e-2, -2.332e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">14</td><td style="border: 1px solid #ddd; padding: 4px;">5.610e-2</td><td style="border: 1px solid #ddd; padding: 4px;">4.737e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-9.727e-2, -2.504e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">15</td><td style="border: 1px solid #ddd; padding: 4px;">7.216e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.372e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.457e-1, -2.580e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">16</td><td style="border: 1px solid #ddd; padding: 4px;">8.782e-2</td><td style="border: 1px solid #ddd; padding: 4px;">5.927e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-1.868e-1, -2.569e-1]</td></tr>
<tr><td style="border: 1px solid #ddd; padding: 4px;">17</td><td style="border: 1px solid #ddd; padding: 4px;">1.009e-1</td><td style="border: 1px solid #ddd; padding: 4px;">6.352e-1</td><td style="border: 1px solid #ddd; padding: 4px;">1.000e-1</td><td style="border: 1px solid #ddd; padding: 4px;">[-2.203e-1, -2.479e-1]</td></tr>
</table>

## Performance Analysis

### Computational Efficiency
- **Average Function Evaluations per Run:** 36.9
- **Average Gradient Evaluations per Run:** 35.9
- **Average Iterations per Run:** 16.9
- **Average Time per Run:** 0.001s
- **Function Evaluations per Second:** 53972.1
- **Iterations per Second:** 24792.1
### Resource Utilization
- **Total Function Evaluations:** 1476
- **Total Gradient Evaluations:** 1436
- **Total Computation Time:** 0.0s
- **Function/Gradient Ratio:** 1.03
## Failure Analysis

### Failure Patterns
- **Early Failures (< 10 iterations):** 0
- **Timeout Failures:** 0
- **Numerical Errors:** 0
- **Maximum Iterations Reached:** 0


## Data Files
* [Raw CSV Data](problems/Sphere_2D_results.csv)
* [Convergence Plot](convergence_Sphere_2D.png)
* [Log Scale Convergence Plot](convergence_Sphere_2D_log.png)


---
*Detailed report for Adam-Fast on Sphere_2D*
*Generated on: 2025-07-28 14:15:26 UTC*
*[← Back to Main Report](benchmark_report.md)*
