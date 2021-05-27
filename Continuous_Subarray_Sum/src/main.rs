struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        if nums.len() == 1 {
            return false;
        }
        let mut cache = HashMap::new();
        let mut sum = 0;
        for (i, n) in nums.into_iter().enumerate() {
            sum += n;
            let remain = sum % k;
            if remain == 0 && i != 0 {
                return true;
            }
            if let Some(index) = cache.get(&remain) {
                if i - index > 1 {
                    return true;
                } else {
                    continue;
                }
            }
            cache.insert(remain, i);
        }
        false
    }
}

fn main() {
    println!("{}", Solution::check_subarray_sum(vec![5, 0, 0, 0], 3));
    // 2, 4, 1, 6, 5
}
