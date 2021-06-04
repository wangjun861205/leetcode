struct Solution;

impl Solution {
    fn dfs(grid: &mut Vec<Vec<i32>>, i: usize, j: usize, id: i32) -> i32 {
        grid[i][j] = -1;
        let up = if i > 0 && grid[i - 1][j] == 1 {
            Solution::dfs(grid, i - 1, j, id)
        } else {
            0
        };
        let down = if i < grid.len() - 1 && grid[i + 1][j] == 1 {
            Solution::dfs(grid, i + 1, j, id)
        } else {
            0
        };
        let left = if j > 0 && grid[i][j - 1] == 1 {
            Solution::dfs(grid, i, j - 1, id)
        } else {
            0
        };
        let right = if j < grid[0].len() - 1 && grid[i][j + 1] == 1 {
            Solution::dfs(grid, i, j + 1, id)
        } else {
            0
        };
        grid[i][j] = id;
        up + down + left + right + 1
    }
    pub fn max_area_of_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut id = 2;
        let mut max = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    let count = Solution::dfs(&mut grid, i, j, id);
                    id += 1;
                    max = max.max(count);
                }
            }
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}
