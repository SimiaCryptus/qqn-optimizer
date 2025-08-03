# Algorithm Efficiency Matrix

Mean Function Evaluations for Successful Runs

| Problem Family | Adam | GD | L-BFGS | QQN | Trust Region |
|---|---|---|---|---|---|
| **Ackley** | N/A | N/A | 24 ± 28 | 26 ± 16 | 27 ± 0 |
| **Barrier** | N/A | N/A | N/A | N/A | N/A |
| **Beale** | 1540 ± 160 | 96 ± 83 | 129 ± 63 | 263 ± 187 | 816 ± 712 |
| **Booth** | 1886 ± 74 | 80 ± 16 | 84 ± 72 | 87 ± 46 | 923 ± 76 |
| **GoldsteinPrice** | N/A | N/A | 72 ± 6 | 287 ± 119 | N/A |
| **Griewank** | N/A | N/A | N/A | 395 ± 0 | N/A |
| **Himmelblau** | 1744 ± 36 | 36 ± 8 | 113 ± 101 | 79 ± 42 | 1217 ± 820 |
| **IllConditionedRosenbrock** | N/A | N/A | 1133 ± 784 | 1282 ± 689 | N/A |
| **Levi** | N/A | N/A | 68 ± 62 | 138 ± 217 | N/A |
| **Levy** | 2167 ± 182 | 1334 ± 344 | 274 ± 287 | 147 ± 112 | N/A |
| **Matyas** | 324 ± 339 | 206 ± 252 | 26 ± 10 | 54 ± 45 | N/A |
| **Michalewicz** | 409 ± 333 | 321 ± 408 | 1123 ± 744 | 1237 ± 806 | N/A |
| **Neural Networks** | 953 ± 722 | N/A | 1709 ± 863 | 1139 ± 883 | N/A |
| **NoisySphere** | 10 ± 7 | 8 ± 5 | 78 ± 94 | 62 ± 50 | N/A |
| **PenaltyI** | N/A | N/A | N/A | N/A | N/A |
| **Rastrigin** | 403 ± 297 | 18 ± 7 | 96 ± 153 | 82 ± 53 | 661 ± 750 |
| **Regression** | 213 ± 23 | 230 ± 113 | 192 ± 179 | 91 ± 46 | N/A |
| **Rosenbrock** | 1352 ± 886 | 43 ± 16 | 693 ± 1032 | 552 ± 751 | 1682 ± 233 |
| **SVM** | 1295 ± 708 | 186 ± 90 | 62 ± 40 | 93 ± 60 | N/A |
| **Schwefel** | N/A | N/A | N/A | 62 ± 54 | N/A |
| **SparseQuadratic** | 1630 ± 111 | 229 ± 128 | 107 ± 141 | 66 ± 42 | N/A |
| **SparseRosenbrock** | N/A | N/A | 1544 ± 800 | 1492 ± 544 | N/A |
| **Sphere** | 1448 ± 761 | 62 ± 64 | 49 ± 66 | 19 ± 13 | 549 ± 796 |
| **StyblinskiTang** | 1128 ± 875 | 55 ± 19 | 130 ± 103 | 140 ± 170 | 848 ± 749 |
| **Trigonometric** | 1074 ± 652 | 258 ± 169 | 446 ± 597 | 473 ± 491 | N/A |
| **Zakharov** | 2095 ± 174 | 367 ± 138 | 464 ± 566 | 151 ± 63 | N/A |

**Purpose:** Shows mean function evaluations ± standard deviation for successful runs only across problem families. Lower values indicate higher efficiency.
