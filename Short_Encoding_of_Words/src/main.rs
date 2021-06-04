struct Solution;

use std::cmp::Reverse;
use std::collections::HashSet;

impl Solution {
    pub fn minimum_length_encoding(mut words: Vec<String>) -> i32 {
        words.sort_by_key(|v| Reverse(v.len()));
        let mut set: HashSet<String> = HashSet::new();
        let mut encoding: Vec<String> = Vec::new();
        for word in words {
            if set.contains(&word) {
                continue;
            }
            let chars: Vec<char> = word.chars().collect();
            for i in 0..chars.len() {
                set.insert(chars[i..].to_vec().into_iter().collect::<String>());
            }
            encoding.push(word);
        }
        let word_count = encoding.len();
        let total_len: usize = encoding.iter().map(|v| v.len()).sum();
        (total_len + word_count) as i32
    }
}
fn main() {
    println!(
        "{}",
        Solution::minimum_length_encoding(vec![
            "time".to_owned(),
            "me".to_owned(),
            "bell".to_owned()
        ])
    );
}
