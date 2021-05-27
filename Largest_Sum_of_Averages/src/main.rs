struct Solution;

use std::cmp::Ordering::Equal;

impl Solution {
    pub fn largest_sum_of_averages(a: Vec<i32>, k: i32) -> f64 {
        let mut presum: Vec<i32> = a
            .iter()
            .scan(0, |s, v| {
                *s += v;
                Some(*s)
            })
            .collect();
        presum.insert(0, 0);
        let avg =
            |s: usize, e: usize| -> f64 { (presum[e + 1] - presum[s]) as f64 / (e - s + 1) as f64 };
        let mut boundaries = Vec::new();
        let mut avgs = Vec::new();
        for i in 0..k {
            boundaries.push(i as usize);
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 1)
    );
}
