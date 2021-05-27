struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        let mut sum = a[0];
        let mut run_max = a[0];
        let mut max = a[0];
        let mut run_min = a[0];
        let mut min = a[0];
        for &v in a[1..].into_iter() {
            sum += v;
            run_max = v.max(run_max + v);
            max = max.max(run_max);
            run_min = v.min(run_min + v);
            min = min.min(run_min);
        }
        if sum == min {
            return max;
        }
        max.max(sum - min)
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_subarray_sum_circular(vec![3, -1, 2, -1])
    );
}
