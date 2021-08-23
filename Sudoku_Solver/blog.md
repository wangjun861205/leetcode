Write a program to solve a Sudoku puzzle by filling the empty cells.

A sudoku solution must satisfy all of the following rules:

Each of the digits 1-9 must occur exactly once in each row.
Each of the digits 1-9 must occur exactly once in each column.
Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
The '.' character indicates empty cells.

Example 1:

![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)

Input:  
board = [  
 ["5","3",".",".","7",".",".",".","."],  
 ["6",".",".","1","9","5",".",".","."],  
 [".","9","8",".",".",".",".","6","."],  
 ["8",".",".",".","6",".",".",".","3"],  
 ["4",".",".","8",".","3",".",".","1"],  
 ["7",".",".",".","2",".",".",".","6"],  
 [".","6",".",".",".",".","2","8","."],  
 [".",".",".","4","1","9",".",".","5"],  
 [".",".",".",".","8",".",".","7","9"]  
 ]  
Output:  
[
["5","3","4","6","7","8","9","1","2"],  
 ["6","7","2","1","9","5","3","4","8"],  
 ["1","9","8","3","4","2","5","6","7"],  
 ["8","5","9","7","6","1","4","2","3"],  
 ["4","2","6","8","5","3","7","9","1"],  
 ["7","1","3","9","2","4","8","5","6"],  
 ["9","6","1","5","3","7","2","8","4"],  
 ["2","8","7","4","1","9","6","3","5"],  
 ["3","4","5","2","8","6","1","7","9"]  
]

Explanation:  
The input board is shown above and the only valid solution is shown below:

Constraints:

- board.length == 9
- board[i].length == 9
- board[i][j] is a digit or '.'.
- It is guaranteed that the input board has only one solution.

---

题目看着挺吓人，但是实际上没有多难， 建立 3 个集合来分别保存行、列、方块当中所剩余的可以用的数字。比如 row_set[i] = { 1, 2, 3 }就表示第 i 行还剩下 1, 2 和 3 三个数字可选, col_set[j] = { 4, 5, 6 }则表示第 j 行还剩下 4, 5 和 6 三个数字可选， blk_set[i/3][j/3] = { 7, 8, 9 }则为 board[i][j]这个位置所处的方格里面还剩下 7, 8 和 9 三个数字可选。注意， 我们实际可选的数字实际是三个集合的交集， 因为我们要找到既不在行里出现过的，也不再列里出现过的，还不在方格里出现过的。如果交集是空集的话则证明不能成立，需要前面步骤重新填写。记着每填写一个格子，都要记得在三个集合里面去除掉填写的那个数字。

---

代码实现(Rust):

```rust
use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    fn dp(board: &mut Vec<Vec<char>>, row_set: Vec<HashSet<i32>>, col_set: Vec<HashSet<i32>>, blk_set: Vec<Vec<HashSet<i32>>>) -> bool {
        let mut row = None;
        let mut col = None;
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    row = Some(i);
                    col = Some(j);
                }
            }
        }
        if row.is_none() && col.is_none() {
            return true;
        }
        let row = row.unwrap();
        let col = col.unwrap();
        let inter: HashSet<i32> = row_set[row]
            .intersection(&col_set[col].intersection(&blk_set[row / 3][col / 3]).map(|v| *v).collect())
            .map(|v| *v)
            .collect();
        if inter.is_empty() {
            return false;
        }
        for v in inter {
            let mut r_set = row_set.clone();
            let mut c_set = col_set.clone();
            let mut b_set = blk_set.clone();
            r_set[row].remove(&v);
            c_set[col].remove(&v);
            b_set[row / 3][col / 3].remove(&v);
            board[row][col] = v.to_string().chars().nth(0).unwrap();
            if Solution::dp(board, r_set, c_set, b_set) {
                return true;
            }
        }
        board[row][col] = '.';
        return false;
    }

    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_set: Vec<HashSet<i32>> = vec![HashSet::from_iter(1..=9); 9];
        let mut col_set: Vec<HashSet<i32>> = vec![HashSet::from_iter(1..=9); 9];
        let mut blk_set: Vec<Vec<HashSet<i32>>> = vec![vec![HashSet::from_iter(1..=9); 3]; 3];
        for row in 0..9 {
            for col in 0..9 {
                let val = board[row][col];
                if val != '.' {
                    let v = val.to_string().parse::<i32>().unwrap();
                    row_set[row].remove(&v);
                    col_set[col].remove(&v);
                    blk_set[row / 3][col / 3].remove(&v);
                }
            }
        }
        Solution::dp(board, row_set, col_set, blk_set);
    }
}

```
