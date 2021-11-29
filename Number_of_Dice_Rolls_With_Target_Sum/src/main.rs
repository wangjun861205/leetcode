struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(d: i32, f: i32, target: i32, cache: &mut HashMap<(i32, i32, i32), i64>) -> i64 {
        if d == 1 {
            if target > f {
                return 0;
            } else {
                return 1;
            }
        }
        const M: i64 = 1000000007;
        let mut sum = 0_i64;
        for v in 1..=f {
            if target - v > 0 {
                let next = if let Some(c) = cache.get(&(d - 1, f, target - v)) {
                    *c
                } else {
                    Solution::dp(d - 1, f, target - v, cache)
                };
                sum += next % M;
                sum %= M;
            }
        }
        sum %= M;
        cache.insert((d, f, target), sum);
        sum
    }
    pub fn num_rolls_to_target(d: i32, f: i32, target: i32) -> i32 {
        Solution::dp(d, f, target, &mut HashMap::new()) as i32
    }
}

fn main() {
    println!("{}", Solution::num_rolls_to_target(30, 30, 500));
}
