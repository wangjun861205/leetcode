struct Solution;

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

fn main() {
    println!("Hello, world!");
}
