struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rev(num: i32) -> i32 {
        num.to_string()
            .chars()
            .rev()
            .collect::<String>()
            .parse()
            .unwrap()
    }
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        let mut diffs: HashMap<i32, i32> = HashMap::new();
        let mut count = 0;
        for n in nums.into_iter().rev() {
            let diff = n - Solution::rev(n);
            if let Some(c) = diffs.get(&diff) {
                count += *c;
                count %= 1000000007;
            }
            *diffs.entry(diff).or_insert(0) += 1;
        }

        count
    }
}
fn main() {
    println!("{}", Solution::count_nice_pairs(vec![13, 10, 35, 24, 76]));
}
