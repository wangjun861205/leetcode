struct Solution;

use std::cmp::{Ord, PartialOrd};

#[derive(PartialEq, Eq)]
struct Team(char, Vec<i32>);

impl Ord for Team {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        for (a, b) in self.1.iter().zip(other.1.iter()) {
            if a > b {
                return std::cmp::Ordering::Greater;
            } else if a < b {
                return std::cmp::Ordering::Less;
            }
        }
        if self.0 > other.0 {
            return std::cmp::Ordering::Less;
        }
        return std::cmp::Ordering::Greater;
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn rank_teams(votes: Vec<String>) -> String {
        let length = votes[0].len();
        let m = votes.into_iter().fold(HashMap::new(), |mut m, v| {
            for (i, c) in v.chars().enumerate() {
                m.entry(c).or_insert(vec![0; length])[i] += 1;
            }
            m
        });
        let mut heap = m.into_iter().fold(BinaryHeap::new(), |mut h, (c, l)| {
            h.push(Team(c, l));
            h
        });
        let mut ans = String::new();
        while !heap.is_empty() {
            ans.push(heap.pop().unwrap().0);
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::rank_teams(
            vec!["WXYZ", "XYZW"]
                .into_iter()
                .map(str::to_owned)
                .collect()
        )
    );
}
