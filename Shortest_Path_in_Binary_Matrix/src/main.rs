struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 {
            return -1;
        }
        let mut map = vec![vec![i32::MAX; grid[0].len()]; grid.len()];
        map[0][0] = 1;
        loop {
            let mut changed = false;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    let v = map[i][j];
                    if map[i][j] != i32::MAX {
                        if i > 0 && map[i - 1][j] > v + 1 && grid[i - 1][j] == 0 {
                            map[i - 1][j] = v + 1;
                            changed = true;
                        }
                        if i < grid.len() - 1 && map[i + 1][j] > v + 1 && grid[i + 1][j] == 0 {
                            map[i + 1][j] = v + 1;
                            changed = true;
                        }
                        if j > 0 && map[i][j - 1] > v + 1 && grid[i][j - 1] == 0 {
                            map[i][j - 1] = v + 1;
                            changed = true;
                        }
                        if j < grid[0].len() - 1 && map[i][j + 1] > v + 1 && grid[i][j + 1] == 0 {
                            map[i][j + 1] = v + 1;
                            changed = true;
                        }
                        if i > 0 && j > 0 && map[i - 1][j - 1] > v + 1 && grid[i - 1][j - 1] == 0 {
                            map[i - 1][j - 1] = v + 1;
                            changed = true;
                        }
                        if i > 0 && j < grid[0].len() - 1 && map[i - 1][j + 1] > v + 1 && grid[i - 1][j + 1] == 0 {
                            map[i - 1][j + 1] = v + 1;
                            changed = true;
                        }
                        if i < grid.len() - 1 && j > 0 && map[i + 1][j - 1] > v + 1 && grid[i + 1][j - 1] == 0 {
                            map[i + 1][j - 1] = v + 1;
                            changed = true;
                        }
                        if i < grid.len() - 1 && j < grid[0].len() - 1 && map[i + 1][j + 1] > v + 1 && grid[i + 1][j + 1] == 0 {
                            map[i + 1][j + 1] = v + 1;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        if map[grid.len() - 1][grid[0].len() - 1] == i32::MAX {
            -1
        } else {
            map[grid.len() - 1][grid[0].len() - 1]
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::shortest_path_binary_matrix(vec![
            vec![0, 0, 1, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0, 0, 1],
            vec![0, 0, 1, 0, 1, 0, 0],
            vec![0, 0, 0, 1, 1, 1, 0],
            vec![1, 0, 0, 1, 1, 0, 0],
            vec![1, 1, 1, 1, 1, 0, 1],
            vec![0, 0, 1, 0, 0, 0, 0]
        ])
    );
}
