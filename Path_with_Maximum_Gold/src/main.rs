struct Solution;

impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, i: usize, j: usize, mut visited: Vec<Vec<bool>>) -> i32 {
        visited[i][j] = true;
        let up = if i > 0 && grid[i - 1][j] > 0 && !visited[i - 1][j] {
            Solution::dfs(grid, i - 1, j, visited.clone())
        } else {
            0
        };
        let down = if i < grid.len() - 1 && grid[i + 1][j] > 0 && !visited[i + 1][j] {
            Solution::dfs(grid, i + 1, j, visited.clone())
        } else {
            0
        };
        let left = if j > 0 && grid[i][j - 1] > 0 && !visited[i][j - 1] {
            Solution::dfs(grid, i, j - 1, visited.clone())
        } else {
            0
        };
        let right = if j < grid[0].len() - 1 && grid[i][j + 1] > 0 && !visited[i][j + 1] {
            Solution::dfs(grid, i, j + 1, visited.clone())
        } else {
            0
        };
        grid[i][j] + vec![up, down, left, right].into_iter().max().unwrap()
    }

    pub fn get_maximum_gold(grid: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] > 0 {
                    let visited = vec![vec![false; grid[0].len()]; grid.len()];
                    ans = ans.max(Solution::dfs(&grid, i, j, visited));
                }
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::get_maximum_gold(vec![vec![0, 6, 0], vec![5, 8, 7], vec![0, 9, 0]])
    );
}
