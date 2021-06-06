struct Solution;

use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap};

impl Solution {
    pub fn max_performance(n: i32, speed: Vec<i32>, efficiency: Vec<i32>, k: i32) -> i32 {
        let modulor = 10_i64.pow(9) + 7;
        let mut pairs: Vec<(i32, i32)> = efficiency.into_iter().zip(speed).collect();
        pairs.sort_by_key(|v| Reverse(v.0));
        let mut speed_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        let mut speed_sum: i64 = 0;
        let mut performance: i64 = 0;
        for (efc, spd) in pairs {
            if speed_heap.len() > k as usize - 1 {
                speed_sum -= speed_heap.pop().unwrap().0 as i64;
            }
            speed_heap.push(Reverse(spd));
            speed_sum += spd as i64;
            performance = performance.max(speed_sum * efc as i64);
        }
        (performance % modulor) as i32
    }
}

fn main() {
    println!("{}", Solution::max_performance(6, vec![10, 5, 1, 7, 4, 2], vec![2, 1, 1, 1, 7, 3], 6))
}
