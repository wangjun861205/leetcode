struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let pre_sums: Vec<i32> = nums
            .into_iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect();
        let mut ans = 0;
        let mut counts: HashMap<i32, i32> = vec![(0, 1)].into_iter().collect();
        for i in 0..pre_sums.len() {
            let target = pre_sums[i] - goal;
            ans += counts.get(&target).unwrap_or(&0);
            *counts.entry(pre_sums[i]).or_insert(0) += 1;
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::num_subarrays_with_sum(vec![1, 0, 1, 0, 1], 2)
    );
}
