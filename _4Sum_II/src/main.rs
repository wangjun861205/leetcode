struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn four_sum_count(
        nums1: Vec<i32>,
        nums2: Vec<i32>,
        nums3: Vec<i32>,
        nums4: Vec<i32>,
    ) -> i32 {
        let mut m1 = HashMap::new();
        for n1 in &nums1 {
            for n2 in &nums2 {
                *m1.entry(n1 + n2).or_insert(0) += 1;
            }
        }
        let mut m2 = HashMap::new();
        for n3 in &nums3 {
            for n4 in &nums4 {
                *m2.entry(n3 + n4).or_insert(0) += 1;
            }
        }
        let mut count = 0;
        for (n1, c1) in m1 {
            count += m2.get(&(-n1)).unwrap_or(&0) * c1;
        }
        count
    }
}
fn main() {
    println!("Hello, world!");
}
