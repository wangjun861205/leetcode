struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(
        n: i32,
        remain: i32,
        bitset: i32,
        prefix_zero: bool,
        cache: &mut HashMap<(i32, i32, bool), i32>,
    ) -> i32 {
        if remain == 0 {
            return 1;
        }
        let mut ans = 0;
        for i in 0..10 {
            if bitset & (1 << i) > 0 {
                if i == 0 && prefix_zero {
                    let count = if let Some(c) = cache.get(&(remain - 1, bitset, true)) {
                        *c
                    } else {
                        Solution::dp(n, remain - 1, bitset, true, cache)
                    };
                    ans += count;
                } else {
                    let count = if let Some(c) = cache.get(&(remain - 1, bitset ^ (1 << i), false))
                    {
                        *c
                    } else {
                        Solution::dp(n, remain - 1, bitset ^ (1 << i), false, cache)
                    };
                    ans += count;
                }
            }
        }
        cache.insert((remain, bitset, prefix_zero), ans);
        ans
    }

    pub fn count_numbers_with_unique_digits(n: i32) -> i32 {
        Solution::dp(n, n, (1 << 10) - 1, true, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::count_numbers_with_unique_digits(8));
}
