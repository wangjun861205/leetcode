struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut set = HashSet::new();
        let mut chars: Vec<Vec<char>> = s
            .chars()
            .map(|c| {
                vec![
                    c.to_uppercase().nth(0).unwrap(),
                    c.to_lowercase().nth(0).unwrap(),
                ]
            })
            .collect();
        for mut i in 0..1 << s.len() {
            let mut s = String::new();
            for j in 0..chars.len() {
                s.push(chars[j][i & 1]);
                i >>= 1;
            }
            set.insert(s);
        }
        set.into_iter().collect()
    }
}

fn main() {
    println!("{:?}", Solution::letter_case_permutation("3z4".into()));
}
