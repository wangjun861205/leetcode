struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(nums: Vec<i32>, k: i32, cache: &mut HashMap<(i32, usize), i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        if k == 0 {
            let sum: i32 = nums.iter().sum();
            let max: i32 = *nums.iter().max().unwrap();
            return max * nums.len() as i32 - sum;
        }
        let mut min = i32::MAX;
        for i in 0..nums.len() {
            let sum: i32 = nums[..=i].iter().sum();
            let max: i32 = *nums[..=i].iter().max().unwrap();
            let next = nums[i + 1..].to_vec();
            let w = max * nums[..=i].len() as i32 - sum
                + if let Some(c) = cache.get(&(k - 1, next.len())) {
                    *c
                } else {
                    Solution::dp(next, k - 1, cache)
                };
            min = min.min(w);
        }
        cache.insert((k, nums.len()), min);
        min
    }
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        Solution::dp(nums, k, &mut HashMap::new())
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1)
    );
}
