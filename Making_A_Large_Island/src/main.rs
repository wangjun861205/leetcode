struct Solution;

use std::collections::HashSet;
use std::iter::FromIterator;

impl Solution {
    pub fn largest_island(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut counts = vec![0; 250000];
        let mut val = 1;
        grid.iter_mut().for_each(|l| {
            l.iter_mut().for_each(|v| {
                if v == &mut 1 {
                    *v = val;
                    counts[val as usize] = 1;
                    val += 1;
                }
            });
        });
        loop {
            let mut changed = false;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] > 0 {
                        let left = if j > 0 { grid[i][j - 1] } else { 0 };
                        let right = if j < grid[0].len() - 1 { grid[i][j + 1] } else { 0 };
                        let top = if i > 0 { grid[i - 1][j] } else { 0 };
                        let bottom = if i < grid.len() - 1 { grid[i + 1][j] } else { 0 };
                        let max = left.max(right).max(top).max(bottom);
                        if max > grid[i][j] {
                            counts[grid[i][j] as usize] -= 1;
                            counts[max as usize] += 1;
                            grid[i][j] = max;
                            changed = true;
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        let mut ans = *counts.iter().max().unwrap();
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 {
                    let left = if j > 0 { grid[i][j - 1] } else { 0 };
                    let right = if j < grid[0].len() - 1 { grid[i][j + 1] } else { 0 };
                    let top = if i > 0 { grid[i - 1][j] } else { 0 };
                    let bottom = if i < grid.len() - 1 { grid[i + 1][j] } else { 0 };
                    let vals: HashSet<i32> = HashSet::from_iter(vec![left, right, top, bottom].into_iter());
                    let count = vals.into_iter().map(|k| counts[k as usize]).sum::<i32>() + 1;
                    ans = ans.max(count);
                }
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::largest_island(vec![vec![1, 1], vec![1, 1],]));
}
