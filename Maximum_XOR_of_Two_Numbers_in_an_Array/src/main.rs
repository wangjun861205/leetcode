struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn find_maximum_xor(nums: Vec<i32>) -> i32 {
        let mut mask = 0;
        let mut ans = 0;
        for i in (0..32).rev() {
            mask |= 1 << i;
            let set: HashSet<i32> = nums.iter().map(|&v| v & mask).collect();
            let best = ans | (1 << i);
            for &p in &set {
                if set.contains(&(p ^ best)) {
                    ans = best;
                }
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
