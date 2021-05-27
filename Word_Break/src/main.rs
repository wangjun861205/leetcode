struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(s: String, word_dict: &Vec<String>, cache: &mut HashMap<usize, bool>) -> bool {
        if s.len() == 0 {
            return true;
        }
        for w in word_dict {
            if s.starts_with(w) {
                if let Some(c) = cache.get(&(s.len() - w.len())) {
                    return *c;
                } else {
                    if Solution::rc(s.replacen(w, "", 1), word_dict, cache) {
                        return true;
                    }
                }
            }
        }
        cache.insert(s.len(), false);
        false
    }
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        let mut cache = HashMap::new();
        Solution::rc(s, &word_dict, &mut cache)
    }
}

fn main() {
    println!(
        "{}",
        Solution::word_break(
            "applepenapple".to_owned(),
            vec!["apple".to_owned(), "pen".to_owned()]
        )
    );
}
