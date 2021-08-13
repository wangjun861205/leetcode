struct Solution;

use std::collections::HashMap;

impl Solution {
    fn predict(nums: &Vec<i32>, left: usize, right: usize, is_p1: bool, cache: &mut HashMap<(usize, usize, bool), i32>) -> i32 {
        if left == right {
            if is_p1 {
                return nums[left];
            } else {
                return 0;
            }
        }
        let mut pick_left = 0;
        if is_p1 {
            pick_left = nums[left]
                + if let Some(c) = cache.get(&(left + 1, right, !is_p1)) {
                    *c
                } else {
                    Solution::predict(nums, left + 1, right, !is_p1, cache)
                };
        } else {
            pick_left = if let Some(c) = cache.get(&(left + 1, right, !is_p1)) {
                *c
            } else {
                Solution::predict(nums, left + 1, right, !is_p1, cache)
            };
        }
        let mut pick_right = 0;
        if is_p1 {
            pick_right = nums[right]
                + if let Some(c) = cache.get(&(left, right - 1, !is_p1)) {
                    *c
                } else {
                    Solution::predict(nums, left, right - 1, !is_p1, cache)
                };
        } else {
            pick_right = if let Some(c) = cache.get(&(left, right - 1, !is_p1)) {
                *c
            } else {
                Solution::predict(nums, left, right - 1, !is_p1, cache)
            }
        }
        let ans = if is_p1 { pick_left.max(pick_right) } else { pick_left.min(pick_right) };
        cache.insert((left, right, is_p1), ans);
        ans
    }

    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let total: i32 = nums.iter().sum();
        let mut cache = HashMap::new();
        let p1 = Solution::predict(&nums, 0, nums.len() - 1, true, &mut cache);
        p1 * 2 >= total
    }
}
fn main() {
    println!("{}", Solution::predict_the_winner(vec![1, 3, 1]));
}
