struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn find_and_replace_pattern(words: Vec<String>, pattern: String) -> Vec<String> {
        words
            .into_iter()
            .filter(|w| {
                if w.len() != pattern.len() {
                    return false;
                }
                let mut map: HashMap<char, char> = HashMap::new();
                for (wc, pc) in w.chars().zip(pattern.chars()) {
                    if let Some(p) = map.get(&pc) {
                        if p != &wc {
                            return false;
                        }
                    } else {
                        if map.values().any(|v| v == &wc) {
                            return false;
                        }
                        map.insert(pc, wc);
                    }
                }
                true
            })
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
