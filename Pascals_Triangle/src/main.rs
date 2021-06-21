struct Solution;

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut dp = vec![vec![1; num_rows as usize]; num_rows as usize];
        for i in 1..num_rows as usize {
            for j in 1..i {
                dp[i][j] = dp[i - 1][j - 1] + dp[i - 1][j];
            }
        }
        dp.into_iter()
            .enumerate()
            .map(|(i, l)| l[..i + 1].to_vec())
            .collect()
    }
}
fn main() {
    println!("{:?}", Solution::generate(5));
}
