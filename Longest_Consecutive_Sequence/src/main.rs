struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let set: HashSet<i32> = nums.into_iter().collect();
        let mut ans = 0;
        for n in &set {
            if !set.contains(&(*n - 1)) {
                let mut count = 1;
                let mut next = *n + 1;
                while set.contains(&next) {
                    count += 1;
                    next += 1;
                }
                ans = ans.max(count);
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
