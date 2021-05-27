struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn rc(words: &HashSet<String>, word: String, cache: &mut HashMap<String, i32>) -> i32 {
        let mut max = 0;
        for i in 0..word.len() {
            let mut w = word.clone();
            w.remove(i);
            if !words.contains(&w) {
                continue;
            }
            if let Some(c) = cache.get(&w) {
                max = max.max(*c);
            } else {
                max = max.max(Solution::rc(words, w, cache));
            }
        }
        cache.insert(word, max + 1);
        max + 1
    }
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let word_set: HashSet<String> = words.iter().map(|v| v.clone()).collect();
        let mut cache = HashMap::new();
        let mut ans = 0;
        for word in words {
            ans = ans.max(Solution::rc(&word_set, word, &mut cache));
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_str_chain(vec![
            "a".to_owned(),
            "b".to_owned(),
            "ba".to_owned(),
            "bca".to_owned(),
            "bda".to_owned(),
            "bdca".to_owned()
        ])
    );
}
