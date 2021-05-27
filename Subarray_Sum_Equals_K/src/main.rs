struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cache = HashMap::new();
        cache.insert(0, 1);
        let mut sum = 0;
        let mut count = 0;
        for n in nums {
            sum += n;
            count += if let Some(v) = cache.get(&(sum - k)) { *v } else { 0 };
            *cache.entry(sum).or_insert(0) += 1;
        }
        count
    }
}

// 0, 1, 2, 3
fn main() {
    println!("{}", Solution::subarray_sum(vec![1, 2, 3], 3));
}
