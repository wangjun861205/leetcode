struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_alien_sorted(words: Vec<String>, order: String) -> bool {
        let map: HashMap<char, usize> = order.chars().enumerate().map(|(i, v)| (v, i)).collect();
        if words.len() == 1 {
            return true;
        }
        for w in words.windows(2).into_iter() {
            let mut cmp = 0;
            for (c1, c2) in w[0].chars().zip(w[1].chars()) {
                let o1 = map.get(&c1).unwrap();
                let o2 = map.get(&c2).unwrap();
                if o1 < o2 {
                    cmp = -1;
                    break;
                } else if o1 == o2 {
                    continue;
                } else {
                    return false;
                }
            }
            if cmp == 0 {
                if w[0].len() > w[1].len() {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {
    println!(
        "{}",
        Solution::is_alien_sorted(
            vec!["hello".to_owned(), "leetcode".to_owned()],
            "hlabcdefgijkmnopqrstuvwxyz".to_owned()
        )
    );
}
