struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut dp = vec![(0, 0); nums.len()];
        dp[0] = if nums[0] < 0 {
            (nums[0], 0)
        } else {
            (0, nums[0])
        };
        let mut min_negative = dp[0].0;
        let mut max_positive = dp[0].1;
        for (i, n) in nums.into_iter().enumerate().skip(1) {
            dp[i].0 = dp[i - 1].0 + n;
            dp[i].1 = dp[i - 1].1 + n;
            if dp[i].0 > 0 {
                dp[i].0 = 0;
            }
            if dp[i].1 < 0 {
                dp[i].1 = 0;
            }
            min_negative = min_negative.min(dp[i].0);
            max_positive = max_positive.max(dp[i].1);
        }
        max_positive.max(min_negative.abs())
    }
}

fn main() {
    println!("{}", Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]));
}
