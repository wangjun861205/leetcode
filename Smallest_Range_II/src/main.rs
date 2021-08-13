struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 1 {
            return 0;
        }
        nums.sort();
        let mut ans = 10000;
        for i in 0..nums.len() {
            let min_left = nums[0] + k;
            let min_right = nums[i] + k;
            if i == nums.len() - 1 {
                ans = ans.min(min_right - min_left);
                break;
            }
            let add_left = nums[i + 1] - k;
            let add_right = nums[nums.len() - 1] - k;
            let min = min_left.min(min_right).min(add_left).min(add_right);
            let max = min_left.max(min_right).max(add_left).max(add_right);
            ans = ans.min(max - min);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::smallest_range_ii(vec![7, 8, 8], 5));
}
