struct Solution;

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut matrix = vec![vec![-1; grid[0].len()]; grid.len()];
        *matrix.last_mut().unwrap().last_mut().unwrap() = *grid.last().unwrap().last().unwrap();
        while matrix[0][0] == -1 {
            for i in 0..matrix.len() {
                for j in 0..matrix[0].len() {
                    if matrix[i][j] == -1 {
                        match (i, j) {
                            (i, j) if i == matrix.len() - 1 => {
                                if matrix[i][j + 1] != -1 {
                                    matrix[i][j] = grid[i][j] + matrix[i][j + 1];
                                }
                            }
                            (i, j) if j == matrix[0].len() - 1 => {
                                if matrix[i + 1][j] != -1 {
                                    matrix[i][j] = grid[i][j] + matrix[i + 1][j];
                                }
                            }
                            _ => {
                                let right = matrix[i][j + 1];
                                let down = matrix[i + 1][j];
                                if right != -1 && down != -1 {
                                    matrix[i][j] = grid[i][j] + right.min(down);
                                }
                            }
                        }
                    }
                }
            }
        }
        matrix[0][0]
    }
}
fn main() {
    println!("Hello, world!");
}
