struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        let counts = s.chars().fold(vec![0; 26], |mut l, c| {
            l[c as usize - 97] += 1;
            l
        });
        let invalid_chars: HashSet<char> = counts
            .into_iter()
            .enumerate()
            .filter(|(_, count)| count != &0 && count < &k)
            .map(|(i, _)| (i as u8 + 97) as char)
            .collect();
        if invalid_chars.is_empty() {
            return s.len() as i32;
        }
        let mut ans = 0;
        let mut ss = String::new();
        for c in s.chars() {
            if !invalid_chars.contains(&c) {
                ss.push(c);
                continue;
            }
            if !ss.is_empty() {
                ans = ans.max(Solution::longest_substring(ss, k));
            }
            ss = String::new();
        }
        if !ss.is_empty() {
            ans = ans.max(Solution::longest_substring(ss, k));
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::longest_substring("ababacb".into(), 3));
}
