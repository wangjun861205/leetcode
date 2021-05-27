struct Solution;

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    pub fn furthest_building(heights: Vec<i32>, mut bricks: i32, ladders: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        for i in 0..heights.len() - 1 {
            let d = heights[i + 1] - heights[i];
            if d <= 0 {
                continue;
            }
            heap.push(Reverse(d));
            if heap.len() > ladders as usize {
                bricks -= heap.pop().unwrap().0;
                if bricks < 0 {
                    return i as i32;
                }
            }
        }
        return heights.len() as i32 - 1;
    }
}

fn main() {
    println!("{}", Solution::furthest_building(vec![1, 2], 0, 0));
}
