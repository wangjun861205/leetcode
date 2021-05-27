struct NumMatrix {
    dp: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {
    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut dp: Vec<Vec<i32>> = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for r in 0..matrix.len() {
            for c in 0..matrix[0].len() {
                dp[r + 1][c + 1] = dp[r][c + 1] + dp[r + 1][c] + matrix[r][c] - dp[r][c];
            }
        }
        Self { dp: dp }
    }

    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        self.dp[row2 as usize + 1][col2 as usize + 1]
            - self.dp[row1 as usize][col2 as usize + 1]
            - self.dp[row2 as usize + 1][col1 as usize]
            + self.dp[row1 as usize][col1 as usize]
    }
}
fn main() {
    println!(
        "{}",
        NumMatrix::new(vec![
            vec![3, 0, 1, 4, 2],
            vec![5, 6, 3, 2, 1],
            vec![1, 2, 0, 1, 5],
            vec![4, 1, 0, 1, 7],
            vec![1, 0, 3, 0, 5]
        ])
        .sum_region(2, 1, 4, 3)
    );
}
