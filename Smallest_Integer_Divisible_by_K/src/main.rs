struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        if k == 1 {
            return 1;
        }
        let mut remainder = 1;
        let mut digits = 1;
        let mut tried = HashSet::new();
        tried.insert(1);
        loop {
            remainder = (remainder * 10 + 1) % k;
            digits += 1;
            if remainder == 0 {
                return digits;
            }
            if tried.contains(&remainder) {
                return -1;
            }
            tried.insert(remainder);
        }
        unreachable!()
    }
}
fn main() {
    println!("{}", Solution::smallest_repunit_div_by_k(4));
}
