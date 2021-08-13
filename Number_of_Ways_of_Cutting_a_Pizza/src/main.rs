struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(presum: &Vec<Vec<i32>>, row: usize, col: usize, rows: usize, cols: usize, k: i32, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        if k == 1 {
            return 1;
        }
        // if row == rows || col == cols {
        //     return 0;
        // }

        let mut ans = 0;
        for i in row..rows - 1 {
            if Solution::is_horizontal_ok(presum, row, col, i) {
                let next = if let Some(c) = cache.get(&(col, i + 1)) {
                    *c
                } else {
                    Solution::rc(presum, i + 1, col, rows, cols, k - 1, cache)
                };
                ans += next;
            }
        }
        for j in col..cols - 1 {
            if Solution::is_vertical_ok(presum, row, col, j) {
                let next = if let Some(c) = cache.get(&(j + 1, row)) {
                    *c
                } else {
                    Solution::rc(presum, row, j + 1, rows, cols, k - 1, cache)
                };
                ans += next;
            }
        }
        cache.insert((col, row), ans);
        ans
    }

    fn is_horizontal_ok(presum: &Vec<Vec<i32>>, top: usize, left: usize, i: usize) -> bool {
        Solution::get_apple_count(presum, top, left, i, presum[0].len() - 2) > 0 && Solution::get_apple_count(presum, i + 1, left, presum.len() - 2, presum[0].len() - 2) > 0
    }

    fn is_vertical_ok(presum: &Vec<Vec<i32>>, top: usize, left: usize, j: usize) -> bool {
        Solution::get_apple_count(presum, top, left, presum.len() - 2, j) > 0 && Solution::get_apple_count(presum, top, j + 1, presum.len() - 2, presum[0].len() - 2) > 0
    }

    fn get_apple_count(presum: &Vec<Vec<i32>>, top: usize, left: usize, bottom: usize, right: usize) -> i32 {
        presum[bottom + 1][right + 1] - presum[top][right + 1] - presum[bottom + 1][left] + presum[top][left]
    }

    pub fn ways(pizza: Vec<String>, k: i32) -> i32 {
        let mut presum: Vec<Vec<i32>> = pizza
            .iter()
            .map(|l| {
                l.chars()
                    .scan(0, |v, c| {
                        if c == 'A' {
                            *v += 1;
                        }
                        Some(*v)
                    })
                    .collect()
            })
            .collect();
        if presum.len() > 1 {
            for i in 1..presum.len() {
                for j in 0..presum[0].len() {
                    presum[i][j] += presum[i - 1][j];
                }
            }
        }
        presum.iter_mut().for_each(|l| l.insert(0, 0));
        presum.insert(0, vec![0; pizza[0].len() + 1]);
        let mut cache = HashMap::new();
        Solution::rc(&presum, 0, 0, pizza.len(), pizza[0].len(), k, &mut cache)
    }
}
fn main() {
    println!("{}", Solution::ways(vec![".A..A", "A.A..", "A.AA.", "AAAA.", "A.AA."].into_iter().map(str::to_owned).collect(), 5));
}
