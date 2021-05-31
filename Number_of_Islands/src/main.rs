struct Solution;

use std::collections::HashSet;

impl Solution {
    fn search(grid: &mut Vec<Vec<String>>, i: usize, j: usize, num: i32) {
        grid[i][j] = num.to_string();
        if i > 0 && &grid[i - 1][j] == "1" {
            Solution::search(grid, i - 1, j, num);
        }
        if i < grid.len() - 1 && &grid[i + 1][j] == "1" {
            Solution::search(grid, i + 1, j, num);
        }
        if j > 0 && &grid[i][j - 1] == "1" {
            Solution::search(grid, i, j - 1, num);
        }
        if j < grid[0].len() - 1 && &grid[i][j + 1] == "1" {
            Solution::search(grid, i, j + 1, num);
        }
    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut grid: Vec<Vec<String>> = grid
            .into_iter()
            .map(|v| v.into_iter().map(|c| c.to_string()).collect())
            .collect();
        let mut count = 1;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if &grid[i][j] == "1" {
                    count += 1;
                    Solution::search(&mut grid, i, j, count);
                }
            }
        }
        count - 1
    }
}
fn main() {
    println!("Hello, world!");
}
