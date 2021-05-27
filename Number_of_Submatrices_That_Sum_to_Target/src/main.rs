struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_submatrix_sum_target(matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        let presum: Vec<Vec<i32>> = matrix
            .iter()
            .map(|v| {
                v.into_iter()
                    .scan(0, |stat, v| {
                        *stat += v;
                        Some(*stat)
                    })
                    .collect()
            })
            .collect();
        let mut ans = 0;
        for y1 in 0..presum.len() {
            for x1 in 0..presum[0].len() {
                for y2 in y1..presum.len() {
                    for x2 in x1..presum[0].len() {
                        if x1 == x2 {
                            if presum[y1..=y2].iter().map(|v| v[x1]).sum::<i32>() == target {
                                ans += 1
                            }
                        } else {
                            if presum[y1..=y2].iter().map(|v| v[x2] - v[x1]).sum::<i32>() == target {
                                ans += 1
                            }
                        }
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::num_submatrix_sum_target(vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]], 0));
}
