struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut left_most = HashMap::new();
        let prexor: Vec<i32> = s
            .chars()
            .enumerate()
            .scan(0, |s, (i, c)| {
                match c {
                    'a' => *s ^= 1,
                    'e' => *s ^= 1 << 1,
                    'i' => *s ^= 1 << 2,
                    'o' => *s ^= 1 << 3,
                    'u' => *s ^= 1 << 4,
                    _ => {}
                }
                if !left_most.contains_key(s) {
                    left_most.insert(*s, i);
                }
                Some(*s)
            })
            .collect();
        if prexor.last().unwrap() == &0 {
            return s.len() as i32;
        }
        let mut ans = 0;
        for i in 0..s.len() {
            if prexor[i] == 0 {
                ans = ans.max(i as i32 + 1);
                continue;
            }
            if let Some(&li) = left_most.get(&prexor[i]) {
                ans = ans.max((li as i32 - i as i32).abs());
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::find_the_longest_substring("leetcodeisgreat".into())
    );
}
