struct Solution;

impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let row_presum: Vec<Vec<i32>> = matrix
            .iter()
            .map(|row| {
                row.iter().fold(vec![0], |mut l, v| {
                    let last = l.last().unwrap();
                    l.push(*v + *last);
                    l
                })
            })
            .collect();
        let mut presum = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for col in 1..matrix[0].len() + 1 {
            let mut sum = 0;
            for row in 1..matrix.len() + 1 {
                sum += row_presum[row - 1][col];
                presum[row][col] = sum;
            }
        }
        let area = |srow: usize, scol: usize, erow: usize, ecol: usize| -> i32 {
            presum[erow + 1][ecol + 1] - presum[erow + 1][scol] - presum[srow][ecol + 1]
                + presum[srow][scol]
        };
        let mut max = -10001;
        for end_row in 0..matrix.len() {
            for end_col in 0..matrix[0].len() {
                for start_row in 0..=end_row {
                    for start_col in 0..=end_col {
                        let a = area(start_row, start_col, end_row, end_col);
                        if a > k {
                            continue;
                        } else {
                            max = max.max(a);
                        }
                    }
                }
            }
        }
        max
    }
}
fn main() {
    println!("{}", Solution::max_sum_submatrix(vec![vec![2, 2, -1]], 0));
}
