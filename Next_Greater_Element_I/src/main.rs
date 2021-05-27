struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let m: HashMap<i32, i32> = nums2.windows(2).map(|v| (v[0], v[1])).collect();
        nums1
            .into_iter()
            .map(|v| if let Some(n2) = m.get(&v) { *n2 } else { -1 })
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2])
    );
}
