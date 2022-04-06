struct Solution;

use std::cmp::Reverse;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair(i32, i32);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Solution {
    pub fn eaten_apples(apples: Vec<i32>, days: Vec<i32>) -> i32 {
        let mut heap: BinaryHeap<Reverse<Pair>> = BinaryHeap::new();
        let mut ans = 0;
        let mut day = 0;
        for (a, d) in apples.into_iter().zip(days) {
            if a > 0 {
                heap.push(Reverse(Pair(day + d, a)));
            }
            while let Some(Reverse(Pair(expire, mut count))) = heap.pop() {
                if expire <= day {
                    continue;
                }
                count -= 1;
                ans += 1;
                if count == 0 {
                    break;
                }
                heap.push(Reverse(Pair(expire, count)));
                break;
            }
            day += 1;
        }
        while let Some(Reverse(Pair(expire, mut count))) = heap.pop() {
            if expire <= day {
                continue;
            }
            count -= 1;
            ans += 1;
            if count == 0 {
                day += 1;
                continue;
            }
            heap.push(Reverse(Pair(expire, count)));
            day += 1;
        }

        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::eaten_apples(vec![3, 0, 0, 0, 0, 2], vec![3, 0, 0, 0, 0, 2])
    );
}
