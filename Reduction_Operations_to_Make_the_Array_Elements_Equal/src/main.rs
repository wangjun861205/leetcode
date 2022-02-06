struct Solution;

use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Count(i32, i32);

impl std::cmp::Ord for Count {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl std::cmp::PartialOrd for Count {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Solution {
    pub fn reduction_operations(nums: Vec<i32>) -> i32 {
        let m: HashMap<i32, i32> = nums.into_iter().fold(HashMap::new(), |mut m, v| {
            *m.entry(v).or_insert(0) += 1;
            m
        });
        let mut heap: BinaryHeap<Count> = m.into_iter().map(|(k, v)| Count(k, v)).collect();
        let mut ans = 0;
        let mut count = 0;
        while heap.len() > 1 {
            let c = heap.pop().unwrap();
            count += c.1;
            ans += count;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::reduction_operations(vec![1, 1, 1, 1]));
}
