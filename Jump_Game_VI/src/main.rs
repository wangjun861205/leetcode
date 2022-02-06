struct Solution;

use std::collections::BTreeMap;

impl Solution {
    pub fn max_result(nums: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        let mut heap: BTreeMap<i32, i32> = BinaryHeap::new();
        for i in 1..nums.len() {
            heap.
        }
    }
}
fn main() {
    println!("{}", Solution::max_result(vec![10, -5, -2, 4, 0, 3], 3));
}
