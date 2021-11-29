struct Solution;

use std::collections::HashMap;

impl Solution {
    fn partition(nums: &Vec<i32>, k: i32, target: i32, i: usize, sum: i32, status: &mut Vec<bool>) -> bool {
        if k == 1 {
            return true;
        }
        if sum == target {
            return Solution::partition(nums, k - 1, target, 0, 0, status);
        }
        for j in i..nums.len() {
            if status[j] && sum + nums[j] <= target {
                status[j] = false;
                if Solution::partition(nums, k, target, i + 1, sum + nums[j], status) {
                    return true;
                }
                status[j] = true;
            }
        }
        false
    }

    pub fn can_partition_k_subsets(nums: Vec<i32>, k: i32) -> bool {
        let sum = nums.iter().sum::<i32>();
        if sum % k != 0 {
            return false;
        }
        let target = sum / k;
        let mut status = vec![true; nums.len()];
        Solution::partition(&nums, k, target, 0, 0, &mut status)
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_partition_k_subsets(vec![3522, 181, 521, 515, 304, 123, 2512, 312, 922, 407, 146, 1932, 4037, 2646, 3871, 269], 5)
    );
}
