struct Solution;

impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut dp = vec![vec![0; 1001]; 1000];
        for i in (0..nums.len() - 1).rev() {
            for j in (i + 1..nums.len()).rev() {
                dp[i][(nums[j] - nums[i] + 500) as usize] =
                    dp[j][(nums[j] - nums[i] + 500) as usize] + 1;
                ans = ans.max(dp[i][(nums[j] - nums[i] + 500) as usize]);
            }
        }
        ans + 1
    }
}

fn main() {
    println!("{}", Solution::longest_arith_seq_length(vec![3, 6, 9, 12]));
}
