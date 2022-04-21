struct Solution;

use std::collections::HashSet;

impl Solution {
    fn color(grid: &mut Vec<Vec<i32>>, row: i32, col: i32, color: i32, visited: &mut HashSet<(i32, i32)>) {
        if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32 || grid[row as usize][col as usize] == 0 || visited.contains(&(row, col)) {
            return;
        }
        visited.insert((row, col));
        grid[row as usize][col as usize] = color;
        Solution::color(grid, row - 1, col, color, visited);
        Solution::color(grid, row + 1, col, color, visited);
        Solution::color(grid, row, col - 1, color, visited);
        Solution::color(grid, row, col + 1, color, visited);
    }
    pub fn count_sub_islands(grid1: Vec<Vec<i32>>, mut grid2: Vec<Vec<i32>>) -> i32 {
        let mut color = 2;
        for row in 0..grid2.len() {
            for col in 0..grid2[0].len() {
                if grid2[row][col] == 1 {
                    Solution::color(&mut grid2, row as i32, col as i32, color, &mut HashSet::new());
                    color += 1;
                }
            }
        }
        if color == 2 {
            return 0;
        }
        let mut stats = vec![true; color as usize - 2];
        for (row1, row2) in grid1.into_iter().zip(grid2) {
            for (v1, v2) in row1.into_iter().zip(row2) {
                if v2 != 0 && v1 == 0 {
                    stats[v2 as usize - 2] = false;
                }
            }
        }
        stats.into_iter().filter(|v| *v).count() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
