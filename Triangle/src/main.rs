struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(
        triangle: &Vec<Vec<i32>>,
        level: usize,
        index: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if level == triangle.len() - 1 {
            return triangle[level][index];
        }
        let mut left = triangle[level][index];
        let mut right = left;
        if let Some(l) = cache.get(&(level + 1, index)) {
            left += *l;
        } else {
            left += Solution::rc(triangle, level + 1, index, cache);
        }
        if let Some(r) = cache.get(&(level + 1, index + 1)) {
            right += *r;
        } else {
            right += Solution::rc(triangle, level + 1, index + 1, cache);
        }
        let ans = left.min(right);
        cache.insert((level, index), ans);
        ans
    }
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let mut cache = HashMap::new();
        Solution::rc(&triangle, 0, 0, &mut cache)
    }
}
fn main() {
    println!(
        "{}",
        Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]])
    );
}
