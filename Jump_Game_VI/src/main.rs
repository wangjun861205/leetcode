#![feature(map_first_last)]
struct Solution;

use std::collections::BTreeSet;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut set: BTreeSet<i32> = BTreeSet::new();
        let mut queue: Vec<i32> = Vec::new();
        let mut dp: Vec<i32> = vec![-10001; nums.len()];
        dp[0] = nums[0];
        dp[nums.len() - 1] = nums[nums.len() - 1];
        for i in (1..nums.len() - 1).rev() {
            if set.len() < k as usize {
                dp[i] = nums[i];
                queue.insert(0, nums[i]);
                set.insert(nums[i]);
            } else {
                let max = *set.last().unwrap();
                dp[i] = nums[i] + max;
                let last = queue.pop().unwrap();
                set.remove(&last);
                set.insert(dp[i]);
                queue.insert(0, dp[i]);
            }
        }
        dp[1..=k as usize].into_iter().max().unwrap() + dp[0] + dp[dp.len() - 1]
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_result(vec![1, -5, -20, 4, -1, 3, -6, -3], 2)
    );
}
