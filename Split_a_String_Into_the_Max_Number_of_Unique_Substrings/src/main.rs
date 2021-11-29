struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn dp(chars: &Vec<char>, idx: usize, existed: HashSet<String>) -> i32 {
        if idx == chars.len() {
            return 0;
        }
        let mut ans = Vec::new();
        for i in idx..chars.len() {
            let cur = chars[idx..=i].into_iter().collect::<String>();
            if !existed.contains(&cur) {
                let mut ex = existed.clone();
                ex.insert(cur.clone());
                let next = Solution::dp(chars, i + 1, ex);
                ans.push(next + 1);
            }
        }
        if ans.is_empty() {
            return 0;
        }
        let max = ans.into_iter().max().unwrap();
        max
    }

    pub fn max_unique_split(s: String) -> i32 {
        Solution::dp(&s.chars().collect(), 0, HashSet::new())
    }
}

fn main() {
    println!("{}", Solution::max_unique_split("wwwzfvedwfvhsww".to_owned()));
}
