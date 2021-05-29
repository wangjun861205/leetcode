struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>, limit: i32) -> i32 {
        let mut max_heap: BinaryHeap<(i32, usize)> = BinaryHeap::new();
        let mut min_heap: BinaryHeap<Reverse<(i32, usize)>> = BinaryHeap::new();
        max_heap.push((nums[0], 0));
        min_heap.push(Reverse((nums[0], 0)));
        let mut ans = 1;
        let mut left: usize = 0;
        for i in 1..nums.len() {
            max_heap.push((nums[i], i));
            min_heap.push(Reverse((nums[i], i)));
            while max_heap.peek().unwrap().0 - min_heap.peek().unwrap().0 .0 > limit {
                left = max_heap.peek().unwrap().1.min(min_heap.peek().unwrap().0 .1) + 1;
                while max_heap.peek().unwrap().1 < left {
                    max_heap.pop();
                }
                while min_heap.peek().unwrap().0 .1 < left {
                    min_heap.pop();
                }
            }
            ans = ans.max(i - left + 1);
        }
        ans as i32
    }
}
fn main() {
    println!("{}", Solution::longest_subarray(vec![2, 2, 2, 4, 4, 2, 5, 5, 5, 5, 5, 2], 2));
}
