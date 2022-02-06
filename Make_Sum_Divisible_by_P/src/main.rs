struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn min_subarray(nums: Vec<i32>, p: i32) -> i32 {
        let mut curr = 0;
        let length = nums.len();
        let mut ans = length as i32;
        let target = nums.iter().fold(0, |s, v| (s + v) % p);
        let mut m: HashMap<i32, i32> = HashMap::new();
        m.insert(0, -1);
        for (i, v) in nums.into_iter().enumerate() {
            curr = (curr + v) % p;
            m.insert(curr, i as i32);
            if let Some(idx) = m.get(&((curr - target + p) % p)) {
                ans = ans.min(i as i32 - *idx);
            }
        }
        if ans == length as i32 {
            -1
        } else {
            ans
        }
    }
}

fn main() {
    println!("{}", Solution::min_subarray(vec![1000000000, 1000000000, 1000000000], 3));
}
