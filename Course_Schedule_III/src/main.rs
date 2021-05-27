struct Solution;

use std::collections::BinaryHeap;

impl Solution {
    pub fn schedule_course(mut courses: Vec<Vec<i32>>) -> i32 {
        courses.sort_by_key(|v| v[1]);
        let mut heap = BinaryHeap::new();
        let mut sum = 0;
        for v in courses {
            let (d, e) = (v[0], v[1]);
            sum += d;
            heap.push(d);
            if sum > e {
                sum -= heap.peek().unwrap();
                heap.pop();
            }
        }
        heap.len() as i32
    }
}
fn main() {
    println!("Hello, world!");
}
