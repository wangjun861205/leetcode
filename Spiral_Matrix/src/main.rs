struct Solution;

use std::collections::HashSet;
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut row = 0_usize;
        let mut col = 0_usize;
        let mut dir = Direction::Right;
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut ans: Vec<i32> = Vec::new();
        loop {
            if visited.contains(&(row, col)) {
                break;
            }
            ans.push(matrix[row][col]);
            visited.insert((row, col));
            match dir {
                Direction::Right => {
                    if col == matrix[0].len() - 1 || visited.contains(&(row, col + 1)) {
                        dir = Direction::Down;
                        if row == matrix.len() - 1 {
                            break;
                        }
                        row += 1;
                    } else {
                        col += 1;
                    }
                }
                Direction::Down => {
                    if row == matrix.len() - 1 || visited.contains(&(row + 1, col)) {
                        dir = Direction::Left;
                        if col == 0 {
                            break;
                        }
                        col -= 1;
                    } else {
                        row += 1;
                    }
                }
                Direction::Left => {
                    if col == 0 || visited.contains(&(row, col - 1)) {
                        dir = Direction::Up;
                        if row == 0 {
                            break;
                        }
                        row -= 1;
                    } else {
                        col -= 1;
                    }
                }
                Direction::Up => {
                    if row == 0 || visited.contains(&(row - 1, col)) {
                        dir = Direction::Right;
                        if col == matrix[0].len() - 1 {
                            break;
                        }
                        col += 1;
                    } else {
                        row -= 1;
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::spiral_order(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]])
    );
}
