struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let mut l: Vec<char> = s.chars().collect();
        let mut buffer: Vec<char> = Vec::new();
        while l.len() > 0 {
            buffer.push(l.remove(0));
            if buffer.len() >= k as usize {
                if buffer[buffer.len() - k as usize..].iter().all(|v| v == &buffer[buffer.len() - k as usize]) {
                    buffer = buffer[..buffer.len() - k as usize].to_vec();
                }
            }
        }
        buffer.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::remove_duplicates("pbbcggttciiippooaais".to_owned(), 2));
}
