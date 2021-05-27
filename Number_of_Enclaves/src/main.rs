struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(grid: &Vec<Vec<i32>>, r: usize, c: usize, visited: &mut HashSet<(usize, usize)>) -> i32 {
        if visited.contains(&(r, c)) {
            return 0;
        }
        visited.insert((r, c));
        let left = if c > 0 && grid[r][c - 1] == 1 {
            Solution::rc(grid, r, c - 1, visited)
        } else {
            0
        };
        let top = if r > 0 && grid[r - 1][c] == 1 {
            Solution::rc(grid, r - 1, c, visited)
        } else {
            0
        };
        let right = if c < grid[0].len() - 1 && grid[r][c + 1] == 1 {
            Solution::rc(grid, r, c + 1, visited)
        } else {
            0
        };
        let bottom = if r < grid.len() - 1 && grid[r + 1][c] == 1 {
            Solution::rc(grid, r + 1, c, visited)
        } else {
            0
        };
        left + top + right + bottom + 1
    }
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let total: i32 = grid
            .iter()
            .map(|r| r.iter().filter(|&c| c == &1).count() as i32)
            .sum();
        let mut count = 0;
        let mut visited = HashSet::new();
        for c in 0..grid[0].len() {
            if grid[0][c] == 1 {
                count += Solution::rc(&grid, 0, c, &mut visited);
            }
            if grid[grid.len() - 1][c] == 1 {
                count += Solution::rc(&grid, grid.len() - 1, c, &mut visited);
            }
        }
        for r in 0..grid.len() {
            if grid[r][0] == 1 {
                count += Solution::rc(&grid, r, 0, &mut visited);
            }
            if grid[r][grid[0].len() - 1] == 1 {
                count += Solution::rc(&grid, r, grid[0].len() - 1, &mut visited);
            }
        }
        total - count
    }
}
fn main() {
    println!(
        "{}",
        Solution::num_enclaves(vec![
            vec![0, 1, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 0]
        ])
    );
}
