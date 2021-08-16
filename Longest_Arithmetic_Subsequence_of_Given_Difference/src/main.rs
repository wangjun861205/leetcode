struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        let mut counts = HashMap::new();
        let mut ans = 1;
        for v in arr {
            if let Some(&c) = counts.get(&(v - difference)) {
                *counts.entry(v).or_insert(0) = c + 1;
                ans = ans.max(c + 1);
            } else {
                counts.insert(v, 1);
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2)
    );
}
