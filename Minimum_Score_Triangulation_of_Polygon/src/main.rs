struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(values: &Vec<i32>, i: usize, j: usize, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        let mut min = i32::MAX;
        for k in 0..values.len() {
            if k < i || k > j {
                let p = values[i] * values[j] * values[k];
                let o1 = if let Some(c) = cache.get(&(i, k)) {
                    *c
                } else {
                    Solution::dp(values, i, k, cache)
                };
                let o2 = if let Some(c) = cache.get(&(j, k)) {
                    *c
                } else {
                    Solution::dp(values, j, k, cache)
                };
                min = min.min(p + o1 + o2);
            }
        }
        cache.insert((i, j), min);
        return min;
    }

    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        let mut ans = i32::MAX;
        for i in 0..values.len() {
            let j = if i == values.len() - 1 { 0 } else { i + 1 };
            ans = ans.min(Solution::dp(&values, i, j, &mut cache));
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::min_score_triangulation(vec![4, 3, 1, 3]));
}
