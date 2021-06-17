struct Solution;

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    fn dfs(
        m: i32,
        n: i32,
        max_move: i32,
        start_row: i32,
        start_column: i32,
        target_row: i32,
        target_column: i32,
        mut visited: HashSet<(i32, i32)>,
        cache: &mut HashMap<(i32, i32, i32, i32, i32), i32>,
    ) -> i32 {
        if start_row == target_row && start_column == target_column {
            return 1;
        }
        if max_move == 1 {
            return 0;
        }
        if start_row < 0 || start_row == m || start_column < 0 || start_column == n {
            return 0;
        }
        // if visited.contains(&(start_row, start_column)) {
        //     return 0;
        // }
        // visited.insert((start_row, start_column));
        let top = if let Some(c) = cache.get(&(start_row - 1, start_column, target_row, target_column, max_move - 1)) {
            *c
        } else {
            Solution::dfs(m, n, max_move - 1, start_row - 1, start_column, target_row, target_column, visited.clone(), cache)
        };
        let bottom = if let Some(c) = cache.get(&(start_row + 1, start_column, target_row, target_column, max_move - 1)) {
            *c
        } else {
            Solution::dfs(m, n, max_move - 1, start_row + 1, start_column, target_row, target_column, visited.clone(), cache)
        };
        let left = if let Some(c) = cache.get(&(start_row, start_column - 1, target_row, target_column, max_move - 1)) {
            *c
        } else {
            Solution::dfs(m, n, max_move - 1, start_row, start_column - 1, target_row, target_column, visited.clone(), cache)
        };
        let right = if let Some(c) = cache.get(&(start_row, start_column + 1, target_row, target_column, max_move - 1)) {
            *c
        } else {
            Solution::dfs(m, n, max_move - 1, start_row, start_column + 1, target_row, target_column, visited.clone(), cache)
        };
        let ans = top + bottom + left + right;
        cache.insert((start_row, start_column, target_row, target_column, max_move), ans);
        ans
    }
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut cache = HashMap::new();
        let mut sum: i64 = 0;
        for row in 0..m {
            for col in 0..n {
                let visited = HashSet::new();
            }
        }
        (sum % (10_i64.pow(9) + 7) as i64) as i32
    }
}

fn main() {
    println!("{}", Solution::find_paths(1, 3, 3, 0, 1));
}
