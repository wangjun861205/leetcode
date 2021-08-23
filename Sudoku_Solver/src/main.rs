struct Solution;

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

fn main() {
    let mut board = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    Solution::solve_sudoku(&mut board);
    println!("{:?}", board);
}
