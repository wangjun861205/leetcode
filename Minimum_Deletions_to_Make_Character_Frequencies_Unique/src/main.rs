struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut counts = vec![0; 26];
        for c in s.chars() {
            counts[c as usize - 97] += 1;
        }
        counts = counts.into_iter().filter(|v| v > &0).collect();
        counts.sort();
        counts.reverse();
        let mut set = HashSet::new();
        let mut count = 0;
        set.insert(counts[0]);
        for i in 1..counts.len() {
            if counts[i] == 0 {
                break;
            }
            while set.contains(&counts[i]) {
                if counts[i] == 0 {
                    break;
                }
                counts[i] -= 1;
                count += 1;
                if counts[i] == 0 {
                    break;
                }
            }
            set.insert(counts[i]);
        }
        count
    }
}

fn main() {
    println!("{}", Solution::min_deletions("bbcebab".to_owned()));
}
