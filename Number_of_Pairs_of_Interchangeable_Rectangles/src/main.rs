struct Solution;

use std::collections::HashMap;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if a == 0 {
            return b;
        }
        if b == 0 {
            return a;
        }
        if a > b {
            let r = a % b;
            return Solution::gcd(b, r);
        } else {
            let r = b % a;
            return Solution::gcd(a, r);
        }
    }
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut counts = HashMap::new();
        let mut sums = HashMap::new();
        for r in rectangles {
            let g = Solution::gcd(r[0], r[1]);
            let count = counts.entry((r[0] / g, r[1] / g)).or_insert(0);
            let sum = sums.entry((r[0] / g, r[1] / g)).or_insert(0);
            *sum += *count;
            *count += 1;
        }
        sums.into_values().sum()
    }
}

fn main() {
    println!("Hello, world!");
}
