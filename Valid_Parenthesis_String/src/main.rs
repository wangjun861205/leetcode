struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(
        chars: &Vec<char>,
        idx: usize,
        left_count: i32,
        cache: &mut HashMap<(usize, i32), bool>,
    ) -> bool {
        if idx == chars.len() {
            if left_count == 0 {
                return true;
            }
            return false;
        }
        if left_count < 0 {
            return false;
        }
        match chars[idx] {
            '(' => {
                let ans = if let Some(c) = cache.get(&(idx + 1, left_count + 1)) {
                    *c
                } else {
                    Solution::dp(chars, idx + 1, left_count + 1, cache)
                };
                cache.insert((idx, left_count), ans);
                return ans;
            }
            ')' => {
                let ans = if let Some(c) = cache.get(&(idx + 1, left_count - 1)) {
                    *c
                } else {
                    Solution::dp(chars, idx + 1, left_count - 1, cache)
                };
                cache.insert((idx, left_count), ans);
                return ans;
            }
            _ => {
                let left = if let Some(c) = cache.get(&(idx + 1, left_count + 1)) {
                    *c
                } else {
                    Solution::dp(chars, idx + 1, left_count + 1, cache)
                };
                let right = if let Some(c) = cache.get(&(idx + 1, left_count - 1)) {
                    *c
                } else {
                    Solution::dp(chars, idx + 1, left_count - 1, cache)
                };
                let empty = if let Some(c) = cache.get(&(idx + 1, left_count)) {
                    *c
                } else {
                    Solution::dp(chars, idx + 1, left_count, cache)
                };
                let ans = left || right || empty;
                cache.insert((idx, left_count), ans);
                return ans;
            }
        }
    }
    pub fn check_valid_string(s: String) -> bool {
        Solution::dp(&s.chars().collect(), 0, 0, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::check_valid_string("**************************************************))))))))))))))))))))))))))))))))))))))))))))))))))".into()));
}
