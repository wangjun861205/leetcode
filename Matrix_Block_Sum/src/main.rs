struct Solution;

impl Solution {
    pub fn matrix_block_sum(mat: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let row_presum: Vec<Vec<i32>> = mat
            .iter()
            .map(|l| {
                let mut sum: Vec<i32> = l
                    .iter()
                    .scan(0, |s, v| {
                        *s += *v;
                        Some(*s)
                    })
                    .collect();
                sum.insert(0, 0);
                sum
            })
            .collect();
        let mut presum = vec![vec![0; mat[0].len() + 1]; mat.len() + 1];
        for row in 0..row_presum.len() {
            for col in 0..row_presum[0].len() {
                presum[row + 1][col] = presum[row][col] + row_presum[row][col];
            }
        }
        let mut ans = vec![vec![0; mat[0].len()]; mat.len()];
        for row in 0..mat.len() {
            for col in 0..mat[0].len() {
                let top_row = if row as i32 - k >= 0 {
                    row - k as usize
                } else {
                    0
                };
                let bottom_row = if (row + k as usize) < mat.len() {
                    row + k as usize
                } else {
                    mat.len() - 1
                };
                let left_col = if col as i32 - k >= 0 {
                    col - k as usize
                } else {
                    0
                };
                let right_col = if (col + k as usize) < mat[0].len() {
                    col + k as usize
                } else {
                    mat[0].len() - 1
                };
                ans[row][col] = presum[bottom_row + 1][right_col + 1]
                    - presum[top_row][right_col + 1]
                    - presum[bottom_row + 1][left_col]
                    + presum[top_row][left_col];
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::matrix_block_sum(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 1)
    );
}
