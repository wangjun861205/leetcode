You are given an m x n binary matrix matrix.

You can choose any number of columns in the matrix and flip every cell in that column (i.e., Change the value of the cell from 0 to 1 or vice versa).

Return the maximum number of rows that have all values equal after some number of flips.

Example 1:

> Input: matrix = [[0,1],[1,1]]  
> Output: 1

Explanation: After flipping no values, 1 row has all values equal.

Example 2:

> Input: matrix = [[0,1],[1,0]]  
> Output: 2

Explanation: After flipping values in the first column, both rows have equal values.

Example 3:

> Input: matrix = [[0,0,0],[0,0,1],[1,1,0]]  
> Output: 2

Explanation: After flipping values in the first two columns, the last two rows have equal values.

Constraints:

- m == matrix.length
- n == matrix[i].length
- 1 <= m, n <= 300
- matrix[i][j] is either 0 or 1.

---

```
[0, 1, 0]
[1, 0, 1]
[0, 0, 1]
```

翻转第 1 列和第 3 列

```
[1, 1, 1]
[0, 0, 0]
[1, 0, 0]
```

我们发现翻转之后第 1 行和第 3 行是符合要求的全部为 0 或者全部为 1 的行。回头来看， 因为我们可以对任何一列做翻转操作， 所以对于任何**一行**而言， 无论这一行是什么样的， 我们都可以通过翻转来使它符合要求， 而这些翻转又会同时影响其他行的相同的列，所以这个问题就可以转换成，与任意一行每一列都相同或者每一列都相反的行的最大数量。

---

```rust
use std::collections::HashMap;

impl Solution {
    pub fn max_equal_rows_after_flips(matrix: Vec<Vec<i32>>) -> i32 {
        let counts = matrix.into_iter().fold(HashMap::new(), |mut m, l| {
            let (normal, counter) =
                l.into_iter()
                    .fold((String::new(), String::new()), |(mut s1, mut s2), v| {
                        if v == 0 {
                            s1.push('0');
                            s2.push('1');
                        } else {
                            s1.push('1');
                            s2.push('0');
                        }
                        (s1, s2)
                    });
            *m.entry(normal).or_insert(0) += 1;
            *m.entry(counter).or_insert(0) += 1;
            m
        });
        *counts.values().into_iter().max().unwrap()
    }
}
```
