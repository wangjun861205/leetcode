struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(board: &Vec<Vec<char>>, row: usize, col: usize, visited: &mut HashSet<(usize, usize)>) {
        visited.insert((row, col));
        if row != 0 && !visited.contains(&(row - 1, col)) && board[row - 1][col] == 'O' {
            Solution::rc(board, row - 1, col, visited);
        }
        if row != board.len() - 1
            && !visited.contains(&(row + 1, col))
            && board[row + 1][col] == 'O'
        {
            Solution::rc(board, row + 1, col, visited);
        }
        if col != 0 && !visited.contains(&(row, col - 1)) && board[row][col - 1] == 'O' {
            Solution::rc(board, row, col - 1, visited);
        }
        if col != board[0].len() - 1
            && !visited.contains(&(row, col + 1))
            && board[row][col + 1] == 'O'
        {
            Solution::rc(board, row, col + 1, visited);
        }
    }
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let mut ans = HashSet::new();
        let rows = board.len();
        let cols = board[0].len();
        for row in 0..rows {
            if board[row][0] == 'O' {
                let mut visited = HashSet::new();
                Solution::rc(board, row, 0, &mut visited);
                ans = ans.union(&visited).map(|v| *v).collect();
            }
            if board[row][cols - 1] == 'O' {
                let mut visited = HashSet::new();
                Solution::rc(board, row, cols - 1, &mut visited);
                ans = ans.union(&visited).map(|v| *v).collect();
            }
        }
        for col in 0..cols {
            if board[0][col] == 'O' {
                let mut visited = HashSet::new();
                Solution::rc(board, 0, col, &mut visited);
                ans = ans.union(&visited).map(|v| *v).collect();
            }
            if board[rows - 1][col] == 'O' {
                let mut visited = HashSet::new();
                Solution::rc(board, rows - 1, col, &mut visited);
                ans = ans.union(&visited).map(|v| *v).collect();
            }
        }
        for r in 0..rows {
            for c in 0..cols {
                if !ans.contains(&(r, c)) {
                    board[r][c] = 'X'
                }
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
