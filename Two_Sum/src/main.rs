struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let m = nums
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut s, (i, v)| {
                s.insert(*v, i);
                s
            });
        for (i, v) in nums.into_iter().enumerate() {
            if let Some(j) = m.get(&(target - v)) {
                if i != *j {
                    return vec![i as i32, *j as i32];
                }
            }
        }
        unreachable!()
    }
}
fn main() {
    println!("Hello, world!");
}
