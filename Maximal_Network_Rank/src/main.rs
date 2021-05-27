struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let mut counts = vec![0; n as usize];
        let mut sets: Vec<HashSet<i32>> = vec![HashSet::new(); n as usize];
        for r in roads {
            counts[r[0] as usize] += 1;
            counts[r[1] as usize] += 1;
            sets[r[0] as usize].insert(r[1]);
            sets[r[1] as usize].insert(r[0]);
        }
        let mut ans = 0;
        for i in 0..n - 1 {
            for j in i + 1..n {
                let rank = if sets[i as usize].contains(&j) {
                    counts[i as usize] + counts[j as usize] - 1
                } else {
                    counts[i as usize] + counts[j as usize]
                };
                ans = ans.max(rank);
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::maximal_network_rank(
            5,
            vec![
                vec![0, 1],
                vec![0, 3],
                vec![1, 2],
                vec![1, 3],
                vec![2, 3],
                vec![2, 4]
            ]
        )
    );
}
