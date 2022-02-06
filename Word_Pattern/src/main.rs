struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        let words: Vec<&str> = s.split(" ").collect();
        if pattern.len() != words.len() {
            return false;
        }
        let mut char_to_word = HashMap::new();
        let mut word_to_char = HashMap::new();
        for (p, w) in pattern.chars().zip(words) {
            if char_to_word.contains_key(&p) && word_to_char.contains_key(w) {
                if char_to_word.get(&p).unwrap() != &w || word_to_char.get(w).unwrap() != &p {
                    return false;
                }
            } else if !(char_to_word.contains_key(&p) || word_to_char.contains_key(w)) {
                char_to_word.insert(p, w);
                word_to_char.insert(w, p);
            } else {
                return false;
            }
        }
        true
    }
}

fn main() {
    println!(
        "{}",
        Solution::word_pattern("abba".into(), "dog cat cat dog".into())
    );
}
