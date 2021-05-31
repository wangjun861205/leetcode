struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            return false;
        }
        if s == goal {
            let map: HashMap<char, usize> = s.chars().fold(HashMap::new(), |mut m, c| {
                *m.entry(c).or_insert(0) += 1;
                m
            });
            return map.values().any(|v| v > &1);
        }
        let diffs: Vec<(char, char)> = s
            .chars()
            .zip(goal.chars())
            .filter(|(s, g)| s != g)
            .collect();
        if diffs.len() != 2 {
            return false;
        }
        return diffs[0].0 == diffs[1].1 && diffs[0].1 == diffs[1].0;
    }
}

fn main() {
    println!("Hello, world!");
}
