struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut sum: i32 = card_points[..k as usize].iter().sum();
        ans = sum;
        for i in 1..=k as usize {
            sum -= card_points[k as usize - i];
            sum += card_points[card_points.len() - i];
            ans = ans.max(sum);
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::max_score(vec![1, 2, 3, 4, 5, 6, 1], 3));
}
