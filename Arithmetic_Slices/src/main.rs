struct Solution;

impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp = vec![(0, 0, 0); nums.len()];
        for i in 1..nums.len() {
            let diff = nums[i] - nums[i - 1];
            let (d, c, t) = dp[i - 1];
            if diff == d {
                dp[i] = (diff, c + 1, c + t);
                continue;
            }
            ans += dp[i - 1].2;
            dp[i] = (diff, 1, 0);
        }
        ans += dp.last().unwrap().2;
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::number_of_arithmetic_slices(vec![1, 2, 3, 8, 9, 10])
    );
}
