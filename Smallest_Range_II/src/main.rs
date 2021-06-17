struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn smallest_range_ii(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut max_heap: BinaryHeap<i32> = BinaryHeap::new();
        min_heap.push(Reverse(nums[0] + k));
        max_heap.push(nums[0] + k);
        for n in nums[1..].to_vec() {
            let plus_diff = n + k - min_heap.peek().unwrap().0;
            let sub_diff = *max_heap.peek().unwrap() - n + k;
            if plus_diff < sub_diff {
                min_heap.push(Reverse(n + k));
                max_heap.push(n + k);
            } else {
                min_heap.push(Reverse(n - k));
                max_heap.push(n - k);
            }
        }
        max_heap.pop().unwrap() - min_heap.pop().unwrap().0
    }
}

fn main() {
    println!("{}", Solution::smallest_range_ii(vec![1, 3, 6], 3));
}
