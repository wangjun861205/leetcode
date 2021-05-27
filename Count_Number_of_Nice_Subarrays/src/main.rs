struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        let len = nums.len();
        let odd_pos: Vec<usize> = nums.into_iter().enumerate().filter(|(_, v)| *v % 2 == 1).map(|(i, _)| i).collect();
        let mut ans = 0;
        for (i, w) in odd_pos.windows(k as usize).enumerate() {
            let left_even_count = if i == 0 { w[0] } else { w[0] - odd_pos[i - 1] - 1 };
            let right_even_count = if i == odd_pos.len() - k as usize {
                len - *w.last().unwrap() - 1
            } else {
                odd_pos[i + k as usize] - *w.last().unwrap() - 1
            };
            ans += (left_even_count as i32 + 1) * (right_even_count as i32 + 1);
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::number_of_subarrays(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2));
}
