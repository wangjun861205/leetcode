struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut dp = vec![vec![0, 0]; nums.len()];
        dp[0][0] = nums[0] as i64;
        for i in 1..nums.len() {
            dp[i][0] = (dp[i - 1][1] + nums[i] as i64).max(dp[i - 1][0]);
            dp[i][1] = (dp[i - 1][0] - nums[i] as i64).max(dp[i - 1][1]);
        }
        dp.last().unwrap()[0]
    }
}

fn main() {
    println!("{}", Solution::max_alternating_sum(vec![6, 2, 1, 2, 4, 5]))
}
