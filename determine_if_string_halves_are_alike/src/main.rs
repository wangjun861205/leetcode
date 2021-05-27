struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut start = 0_usize;
        let mut end = s.len() - 1;
        let mut balance = 0;
        let chars: Vec<char> = s.chars().collect();
        let set: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U']
            .into_iter()
            .collect();
        while start < end {
            if set.contains(&chars[start]) {
                balance += 1;
            }
            if set.contains(&chars[end]) {
                balance -= 1;
            }
            start += 1;
            end -= 1;
        }
        balance == 0
    }
}
fn main() {
    println!("Hello, world!");
}
