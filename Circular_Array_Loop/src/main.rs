struct Solution;

use std::collections::HashSet;

impl Solution {
    fn proc_idx(mut idx: i32, length: i32) -> i32 {
        if idx >= length {
            return idx % length;
        }
        if idx < 0 {
            idx = length - (-idx) % length;
        }
        return idx;
    }
    fn dfs(
        nums: &Vec<i32>,
        prev: i32,
        idx: i32,
        visited: &mut Vec<bool>,
        mut path: HashSet<i32>,
    ) -> bool {
        if nums.len() == 1 {
            return false;
        }
        if path.contains(&idx) {
            return true;
        }
        if visited[idx as usize] {
            return false;
        }
        let val = nums[idx as usize];
        if prev < 1001 && val * prev < 0 {
            return false;
        }
        path.insert(idx);
        visited[idx as usize] = true;
        let next_idx = Solution::proc_idx(idx + val, nums.len() as i32);
        if next_idx == idx {
            return false;
        }
        return Solution::dfs(nums, val, next_idx, visited, path);
    }
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let mut visited = vec![false; nums.len()];
        for i in 0..nums.len() {
            if !visited[i] {
                if Solution::dfs(&nums, 1001, i as i32, &mut visited, HashSet::new()) {
                    return true;
                }
            }
        }
        false
    }
}
fn main() {
    println!("{}", Solution::circular_array_loop(vec![-2, -3, -9]));
}
