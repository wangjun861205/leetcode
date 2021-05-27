struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        if x > y {
            return x - y;
        }
        if x == y {
            return 0;
        }
        if y % 2 == 1 {
            return Solution::broken_calc(x, y + 1) + 1;
        } else {
            return Solution::broken_calc(x, y / 2) + 1;
        }
    }
}
fn main() {
    println!("{}", Solution::broken_calc(3, 10));
}
