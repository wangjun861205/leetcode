struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(
        nums: &Vec<i32>,
        idx: usize,
        a: i32,
        b: i32,
        cache: &mut HashMap<(usize, i32, i32), bool>,
    ) -> bool {
        if idx == nums.len() {
            return a == b;
        }
        let add_to_a = if let Some(c) = cache.get(&(idx + 1, a + nums[idx], b)) {
            *c
        } else {
            Solution::dp(nums, idx + 1, a + nums[idx], b, cache)
        };
        if add_to_a {
            return true;
        }
        let add_to_b = if let Some(c) = cache.get(&(idx + 1, a, b + nums[idx])) {
            *c
        } else {
            Solution::dp(nums, idx + 1, a, b + nums[idx], cache)
        };
        if add_to_b {
            return true;
        }
        cache.insert((idx, a, b), false);
        false
    }
    pub fn can_partition(mut nums: Vec<i32>) -> bool {
        Solution::dp(&nums, 0, 0, 0, &mut HashMap::new())
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_partition(vec![
            34, 16, 5, 72, 28, 99, 90, 67, 81, 27, 22, 48, 85, 35, 41, 37, 59, 94, 60, 60, 49, 38,
            80, 63, 26, 96, 3, 40, 82, 59, 3, 12, 94, 82, 9, 56, 33, 65, 26, 73, 52, 65, 54, 55,
            84, 9, 80, 42, 16, 51, 24, 16, 96, 33, 45, 11, 39, 57, 9, 89, 21, 82, 43, 93, 31, 43,
            95, 28, 40, 100, 67, 21, 32, 55, 36, 77, 3, 2, 95, 73, 56, 84, 9, 56, 91, 54, 28, 6,
            90, 22, 42, 2, 94, 47, 82, 91, 86, 77, 40, 93
        ])
    );
}
