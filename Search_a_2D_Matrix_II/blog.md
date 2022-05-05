Write an efficient algorithm that searches for a value target in an m x n integer matrix matrix. This matrix has the following properties:

Integers in each row are sorted in ascending from left to right.
Integers in each column are sorted in ascending from top to bottom.

Example 1:

![](https://assets.leetcode.com/uploads/2020/11/24/searchgrid2.jpg)

> Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 5
> Output: true

Example 2:

![](https://assets.leetcode.com/uploads/2020/11/24/searchgrid.jpg)

> Input: matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]], target = 20
> Output: false

Constraints:

m == matrix.length
n == matrix[i].length
1 <= n, m <= 300
-109 <= matrix[i][j] <= 109
All the integers in each row are sorted in ascending order.
All the integers in each column are sorted in ascending order.
-109 <= target <= 109

---

从最右上角的元素开始，如果目标值小于当前值则可以排除当前列， 因为列是递增的， 如果目标值大于当前值则可以排除当前行， 因为行也是递增的, 这样循环查找， 直到找到目标值或者行和列的 index 超出矩阵范围

---

```rust

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let mut row = 0 as i32;
        let mut col = matrix[0].len() as i32 - 1;
        while row < matrix.len() as i32 && col >= 0 {
            let val = matrix[row as usize][col as usize];
            if val == target {
                return true;
            }
            if val > target {
                col -= 1;
            }
            if val < target {
                row += 1;
            }
        }
        false
    }
}
```
