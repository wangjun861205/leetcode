struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let mut remains: Vec<Vec<i32>> = s
            .chars()
            .rev()
            .scan(vec![0; 26], |l, c| {
                l[c as usize - 97] += 1;
                Some(l.clone())
            })
            .collect();
        remains.reverse();
        remains.remove(0);
        remains.push(vec![0; 26]);
        let chars: Vec<char> = s.chars().collect();
        let mut set = HashSet::new();
        let mut length = 0;
        let mut ans = Vec::new();
        'outer: for (i, c) in chars.into_iter().enumerate() {
            set.insert(c);
            length += 1;
            for &k in &set {
                if remains[i][k as usize - 97] != 0 {
                    continue 'outer;
                }
            }
            ans.push(length);
            set.clear();
            length = 0;
        }
        ans
    }
}

fn main() {
    println!("{:?}", Solution::partition_labels("eccbbbbdec".into()));
}
