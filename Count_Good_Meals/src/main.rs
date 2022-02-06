struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_pairs(deliciousness: Vec<i32>) -> i32 {
        let counts: HashMap<i32, i32> = deliciousness.iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(*v).or_insert(0) += 1;
            m
        });
        let mut ans = 0_i128;
        for d in deliciousness {
            for i in (0..=21).rev() {
                let p = 2_i32.pow(i);
                if p >= d {
                    let c = p - d;
                    let count = *counts.get(&c).unwrap_or(&0);
                    if c == d {
                        ans += count as i128 - 1;
                    } else {
                        ans += count as i128;
                    }
                } else {
                    break;
                }
            }
        }
        (ans / 2 % 1000000007) as i32
    }
}
fn main() {
    println!("{}", Solution::count_pairs(vec![1048576, 1048576]));
}
