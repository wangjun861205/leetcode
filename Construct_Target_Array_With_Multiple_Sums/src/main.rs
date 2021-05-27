struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn is_possible(target: Vec<i32>) -> bool {
        let mut sum = 0;
        let mut heap: BinaryHeap<i32> = BinaryHeap::new();
        for t in target {
            sum += t;
            heap.push(t);
        }
        while let Some(v) = heap.pop() {
            if v == 1 {
                return true;
            }
            let remain = sum - v;
            if remain == 1 {
                return true;
            }
            if v - remain <= 0 {
                return false;
            }
            sum -= remain;
            heap.push(v - remain);
        }
        unreachable!();
    }
}
fn main() {
    println!("{}", Solution::is_possible(vec![1, 1, 1, 2]));
}
