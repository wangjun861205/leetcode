struct Solution;

use std::collections::HashSet;

impl Solution {
    fn dfs(
        board: &Vec<Vec<char>>,
        mut chars: Vec<char>,
        row: usize,
        col: usize,
        mut visited: HashSet<(usize, usize)>,
    ) -> bool {
        if visited.contains(&(row, col)) {
            return false;
        }
        if chars.len() == 0 {
            return true;
        }
        let c = chars.remove(0);
        visited.insert((row, col));
        if row > 0 && board[row - 1][col] == c {
            if Solution::dfs(board, chars.clone(), row - 1, col, visited.clone()) {
                return true;
            }
        }
        if row < board.len() - 1 && board[row + 1][col] == c {
            if Solution::dfs(board, chars.clone(), row + 1, col, visited.clone()) {
                return true;
            }
        }
        if col > 0 && board[row][col - 1] == c {
            if Solution::dfs(board, chars.clone(), row, col - 1, visited.clone()) {
                return true;
            }
        }
        if col < board[0].len() - 1 && board[row][col + 1] == c {
            if Solution::dfs(board, chars.clone(), row, col + 1, visited.clone()) {
                return true;
            }
        }
        false
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for row in 0..board.len() {
            for col in 0..board[0].len() {
                if board[row][col] == word.chars().collect::<Vec<char>>()[0] {
                    let visited = HashSet::new();
                    if Solution::dfs(&board, word.chars().skip(1).collect(), row, col, visited) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
fn main() {
    println!("Hello, world!");
}
