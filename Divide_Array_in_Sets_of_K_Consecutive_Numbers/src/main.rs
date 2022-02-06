struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_possible_divide(mut nums: Vec<i32>, k: i32) -> bool {
        if nums.len() % k as usize != 0 {
            return false;
        }
        nums.sort();
        let mut m = nums.iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(*v).or_insert(0) += 1;
            m
        });
        for n in nums {
            if *m.get(&n).unwrap() > 0 {
                for i in 0..k {
                    if let Some(v) = m.get_mut(&(n + i)) {
                        if *v == 0 {
                            return false;
                        }
                        *v -= 1;
                        continue;
                    }
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
