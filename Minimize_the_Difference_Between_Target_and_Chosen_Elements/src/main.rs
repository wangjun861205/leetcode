struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(
        mat: &Vec<Vec<i32>>,
        target: i32,
        row: usize,
        cache: &mut HashMap<(i32, usize), i32>,
    ) -> i32 {
        if row == mat.len() {
            return target.abs();
        }
        if target <= 0 {
            return Solution::dp(mat, target - mat[row][0], row + 1, cache);
        }
        let mut ans = i32::MAX;
        for v in &mat[row] {
            let next = if let Some(c) = cache.get(&(target - *v, row + 1)) {
                *c
            } else {
                Solution::dp(mat, target - *v, row + 1, cache)
            };
            ans = ans.min(next);
        }
        cache.insert((target, row), ans);
        ans
    }
    pub fn minimize_the_difference(mut mat: Vec<Vec<i32>>, target: i32) -> i32 {
        for l in &mut mat {
            l.sort();
        }
        Solution::dp(&mat, target, 0, &mut HashMap::new())
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100)
    );
}
