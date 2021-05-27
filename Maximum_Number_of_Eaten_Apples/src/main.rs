struct Solution;

use std::cmp::Reverse;
use std::cmp::{Ord, Ordering, PartialOrd};
use std::collections::BinaryHeap;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Pair(usize, i32);

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
        let mut d = 0_usize;
        'outer: for i in 0..apples.len() {
            d = i;
            let apple_num = apples[i];
            let expire_at = i + days[i] as usize - 1;
            if apple_num == 0 {
                while let Some(mut p) = heap.pop() {
                    if p.0 .0 >= i {
                        p.0 .1 -= 1;
                        ans += 1;
                        if p.0 .1 > 0 {
                            heap.push(p);
                        }
                        break;
                    }
                }
            } else {
                while let Some(mut p) = heap.pop() {
                    if p.0 .0 >= i {
                        if p.0 .0 <= expire_at {
                            p.0 .1 -= 1;
                            ans += 1;
                            if p.0 .1 > 0 {
                                heap.push(p);
                            }
                            heap.push(Reverse(Pair(expire_at, apple_num)));
                        } else {
                            ans += 1;
                            if apple_num > 1 && expire_at > 1 {
                                heap.push(Reverse(Pair(expire_at, apple_num - 1)));
                            }
                        }
                        continue 'outer;
                    }
                }
                ans += 1;
                if apple_num > 1 && expire_at > 1 {
                    heap.push(Reverse(Pair(expire_at, apple_num - 1)));
                }
            }
        }
        while let Some(mut p) = heap.pop() {
            while p.0 .0 >= d && p.0 .1 > 0 {
                p.0 .1 -= 1;
                d += 1;
                ans += 1;
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::eaten_apples(vec![1, 2, 3, 5, 2], vec![3, 2, 1, 4, 2])
    );
}
