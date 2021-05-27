struct Solution;

use std::cmp::{Ord, Ordering, PartialOrd, Reverse};
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct Dest(i32, i32);

impl Ord for Dest {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Dest {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn car_pooling(mut trips: Vec<Vec<i32>>, mut capacity: i32) -> bool {
        trips.sort_by_key(|v| v[1]);
        if trips.len() == 0 {
            return true;
        }
        let mut stack: BinaryHeap<Reverse<Dest>> = BinaryHeap::new();
        for t in trips {
            while !stack.is_empty() {
                if stack.peek().unwrap().0 .0 <= t[1] {
                    let d = stack.pop().unwrap().0;
                    capacity += d.1;
                } else {
                    break;
                }
            }
            if capacity < t[0] {
                return false;
            }
            capacity -= t[0];
            stack.push(Reverse(Dest(t[2], t[0])));
        }
        true
    }
}
fn main() {
    println!(
        "{}",
        Solution::car_pooling(vec![vec![3, 2, 7], vec![3, 7, 9], vec![8, 3, 9]], 11)
    );
}

// [9, 1, 7], [4, 2, 4], [9, 3, 4], [7, 4, 5]
