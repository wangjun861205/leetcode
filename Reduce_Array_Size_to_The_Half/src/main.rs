struct Solution;

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn min_set_size(arr: Vec<i32>) -> i32 {
        let len = arr.len() as i32;
        let half = if len % 2 == 0 { len / 2 } else { len / 2 + 1 };
        let map: HashMap<i32, i32> = arr.into_iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(v).or_insert(0) += 1;
            m
        });
        let mut heap: BinaryHeap<i32> = map.values().map(|v| *v).collect();
        let mut ans = 0;
        let mut count = 0;
        while let Some(v) = heap.pop() {
            if count >= half {
                break;
            }
            count += v;
            ans += 1;
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::min_set_size(vec![1, 9]));
}
