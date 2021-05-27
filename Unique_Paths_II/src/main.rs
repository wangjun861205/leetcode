struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.last().unwrap().last().unwrap() == &1 {
            return 0;
        }
        let mut matrix: Vec<Vec<i32>> = obstacle_grid
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|col| if col == 1 { -1 } else { 0 })
                    .collect()
            })
            .collect();
        *matrix.last_mut().unwrap().last_mut().unwrap() = 1;
        while matrix[0][0] == 0 {
            for i in 0..matrix.len() {
                for j in 0..matrix[0].len() {
                    if matrix[i][j] == 0 {
                        if i == matrix.len() - 1 {
                            if matrix[i][j + 1] != 0 {
                                matrix[i][j] = matrix[i][j + 1];
                            }
                            continue;
                        }
                        if j == matrix[0].len() - 1 {
                            if matrix[i + 1][j] != 0 {
                                matrix[i][j] = matrix[i + 1][j];
                            }
                            continue;
                        }
                        let right = matrix[i][j + 1];
                        let down = matrix[i + 1][j];
                        if right == 0 || down == 0 {
                            continue;
                        }
                        if right == -1 {
                            matrix[i][j] = down;
                        } else if down == -1 {
                            matrix[i][j] = right;
                        } else {
                            matrix[i][j] = right + down;
                        }
                    }
                }
            }
        }
        if matrix[0][0] == -1 {
            return 0;
        }
        return matrix[0][0];
    }
}
fn main() {
    println!(
        "{}",
        Solution::unique_paths_with_obstacles(vec![vec![0, 1], vec![0, 0]])
    );
}
