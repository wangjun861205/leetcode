struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn last_stone_weight(stones: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<i32> = stones.into_iter().collect();
        while let Some(s1) = heap.pop() {
            if let Some(s2) = heap.pop() {
                heap.push(s1 - s2);
            } else {
                return s1;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
