struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let left_max: Vec<i32> = nums
            .iter()
            .scan(BinaryHeap::new(), |h, v| {
                h.push(*v);
                Some(*h.peek().unwrap())
            })
            .collect();
        let mut right_min: Vec<i32> = nums
            .iter()
            .rev()
            .scan(BinaryHeap::new(), |h, v| {
                h.push(Reverse(*v));
                Some(h.peek().unwrap().0)
            })
            .collect();
        right_min.reverse();
        for i in 0..nums.len() - 1 {
            if left_max[i] <= right_min[i + 1] {
                return i as i32 + 1;
            }
        }
        unreachable!();
    }
}

fn main() {
    println!("{}", Solution::partition_disjoint(vec![5, 0, 3, 8, 6]));
}
