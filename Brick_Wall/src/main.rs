struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn least_bricks(wall: Vec<Vec<i32>>) -> i32 {
        let len = wall.len();
        let l: Vec<Vec<i32>> = wall
            .into_iter()
            .map(|l| {
                l[..l.len() - 1]
                    .into_iter()
                    .scan(0, |s, e| {
                        *s += *e;
                        Some(*s)
                    })
                    .collect()
            })
            .collect();
        let mut m = HashMap::new();
        l.into_iter().for_each(|l| {
            l.into_iter().for_each(|v| {
                *m.entry(v).or_insert(0) += 1;
            })
        });
        if let Some(&v) = m.values().max() {
            len as i32 - v
        } else {
            len as i32
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::least_bricks(vec![vec![7, 1, 2], vec![3, 5, 1, 1], vec![10]])
    );
}
