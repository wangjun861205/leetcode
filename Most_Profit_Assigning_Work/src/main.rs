struct Solution;

use std::cmp::{Ord, PartialOrd, Reverse};
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq)]
struct Pair(i32, i32);

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.0 < other.0 {
            return std::cmp::Ordering::Less;
        } else if self.0 > other.0 {
            return std::cmp::Ordering::Greater;
        } else {
            return Reverse(self.1).cmp(&Reverse(other.1));
        }
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl Solution {
    pub fn max_profit_assignment(
        difficulty: Vec<i32>,
        profit: Vec<i32>,
        mut worker: Vec<i32>,
    ) -> i32 {
        let mut heap: BinaryHeap<Pair> = difficulty
            .into_iter()
            .zip(profit)
            .map(|(d, p)| Pair(p, d))
            .collect();
        worker.sort();
        worker.reverse();
        let mut ans = 0;
        let mut pair = heap.pop();
        for w in worker {
            while let Some(p) = pair.take() {
                if p.1 > w {
                    pair = heap.pop();
                    continue;
                }
                ans += p.0;
                pair = Some(p);
                break;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_profit_assignment(
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7]
        )
    );
}
