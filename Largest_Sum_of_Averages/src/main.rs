struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(a: &Vec<i32>, k: i32, idx: usize, cache: &mut HashMap<(i32, usize), f64>) -> f64 {
        if a.is_empty() {
            return f64::MIN;
        }
        if k == 1 {
            let slice = a[idx..].to_vec();
            let length = slice.len();
            let avg = slice.into_iter().sum::<i32>() as f64 / length as f64;
            cache.insert((k, idx), avg);
            return avg;
        }
        let mut ans = 0_f64;
        for i in idx + 1..a.len() {
            let slice = a[idx..i].to_vec();
            let length = slice.len();
            let avg = slice.into_iter().sum::<i32>() as f64 / length as f64;
            let next = if let Some(c) = cache.get(&(k - 1, i)) {
                *c
            } else {
                Solution::dp(a, k - 1, i, cache)
            };
            ans = ans.max(avg + next);
        }
        cache.insert((k, idx), ans);
        ans
    }
    pub fn largest_sum_of_averages(a: Vec<i32>, k: i32) -> f64 {
        Solution::dp(&a, k, 0, &mut HashMap::new())
    }
}
fn main() {
    println!(
        "{}",
        Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3)
    );
}
