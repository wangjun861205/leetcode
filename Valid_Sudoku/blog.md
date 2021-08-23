Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:

1. Each row must contain the digits 1-9 without repetition.
2. Each column must contain the digits 1-9 without repetition.
3. Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.

Note:

A Sudoku board (partially filled) could be valid but is not necessarily solvable.
Only the filled cells need to be validated according to the mentioned rules.

Example 1:

![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)

Input: board =  
[["5","3",".",".","7",".",".",".","."]   
,["6",".",".","1","9","5",".",".","."]   
,[".","9","8",".",".",".",".","6","."]   
,["8",".",".",".","6",".",".",".","3"]   
,["4",".",".","8",".","3",".",".","1"]   
,["7",".",".",".","2",".",".",".","6"]   
,[".","6",".",".",".",".","2","8","."]   
,[".",".",".","4","1","9",".",".","5"]   
,[".",".",".",".","8",".",".","7","9"]]

Output: true
Example 2:

Input: board =  
[["8","3",".",".","7",".",".",".","."]  
,["6",".",".","1","9","5",".",".","."]  
,[".","9","8",".",".",".",".","6","."]  
,["8",".",".",".","6",".",".",".","3"]  
,["4",".",".","8",".","3",".",".","1"]  
,["7",".",".",".","2",".",".",".","6"]  
,[".","6",".",".",".",".","2","8","."]  
,[".",".",".","4","1","9",".",".","5"]  
,[".",".",".",".","8",".",".","7","9"]]

Output: false  
Explanation:

Same as Example 1, except with the 5 in the top left corner being modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.

Constraints:

- board.length == 9
- board[i].length == 9
- board[i][j] is a digit or '.'.

---

这题没啥可讲的，就按题目里说的那三条来检查就可以

---

代码实现(Rust):

```rust
use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                match board[i][j] {
                    '0' => return false,
                    '.' => continue,
                    _ => {
                        if set.contains(&board[i][j]) {
                            return false;
                        }
                        set.insert(board[i][j]);
                    }
                }
            }
        }
        for i in 0..9 {
            let mut set = HashSet::new();
            for j in 0..9 {
                match board[j][i] {
                    '0' => return false,
                    '.' => continue,
                    _ => {
                        if set.contains(&board[j][i]) {
                            return false;
                        }
                        set.insert(board[j][i]);
                    }
                }
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                let mut set = HashSet::new();
                for k in 0..3 {
                    for l in 0..3 {
                        match board[i * 3 + k][j * 3 + l] {
                            '.' => continue,
                            _ => {
                                if set.contains(&board[i * 3 + k][j * 3 + l]) {
                                    return false;
                                }
                                set.insert(board[i * 3 + k][j * 3 + l]);
                            }
                        }
                    }
                }
            }
        }
        true
    }
}
```
