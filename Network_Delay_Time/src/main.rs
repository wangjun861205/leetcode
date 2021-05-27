struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn network_delay_time(times: Vec<Vec<i32>>, n: i32, k: i32) -> i32 {
        let mut ts: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
        for t in times {
            ts[(t[0] - 1) as usize].push((t[1], t[2]));
        }
        let mut counts: Vec<i32> = vec![-1; n as usize];
        let mut visited: HashSet<i32> = HashSet::new();
        let mut stack: Vec<i32> = Vec::new();
        counts[(k - 1) as usize] = 0;
        visited.insert(k);
        stack.push(k);
        while !stack.is_empty() {
            let node = stack.remove(0);
            for t in &ts[(node - 1) as usize] {
                let time = counts[(node - 1) as usize] + t.1;
                if counts[(t.0 - 1) as usize] == -1 {
                    counts[(t.0 - 1) as usize] = time;
                } else {
                    if counts[(t.0 - 1) as usize] > time {
                        counts[(t.0 - 1) as usize] = time;
                        visited.insert(t.0);
                        stack.push(t.0);
                        continue;
                    }
                }
                if !visited.contains(&t.0) {
                    visited.insert(t.0);
                    stack.push(t.0);
                }
            }
        }
        if counts.iter().any(|v| v == &-1) {
            return -1;
        }
        counts.into_iter().max().unwrap()
    }
}
fn main() {
    println!(
        "{}",
        Solution::network_delay_time(vec![vec![2, 1, 1], vec![2, 3, 1], vec![3, 4, 1]], 4, 2)
    );
}
