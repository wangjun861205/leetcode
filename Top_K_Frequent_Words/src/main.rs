struct Solution;

use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Pair(String, i32);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 != other.1 {
            return self.1.partial_cmp(&other.1);
        }
        return other.0.partial_cmp(&self.0);
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        let mut heap: BinaryHeap<Reverse<Pair>> = BinaryHeap::new();
        let mut counts: HashMap<String, i32> = HashMap::new();
        for w in words {
            *counts.entry(w).or_insert(0) += 1;
        }
        for (s, c) in counts {
            heap.push(Reverse(Pair(s, c)));
            if heap.len() > k as usize {
                heap.pop();
            }
        }
        let mut ans = Vec::new();
        while let Some(Reverse(Pair(k, _))) = heap.pop() {
            ans.insert(0, k)
        }
        ans
    }
}

fn main() {
    println!("{}", "i" < "love");
    println!("Hello, world!");
}
