struct Solution;

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Pair(i32, i32, i32);

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.1 == other.1 {
            return self.2.partial_cmp(&other.2);
        }
        return self.1.partial_cmp(&other.1);
    }
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 == other.1 {
            return self.2.cmp(&other.2);
        }
        return self.1.cmp(&other.1);
    }
}

struct FreqStack {
    index: i32,
    heap: BinaryHeap<Pair>,
    counts: HashMap<i32, i32>,
}

impl FreqStack {
    fn new() -> Self {
        Self {
            index: 0,
            heap: BinaryHeap::new(),
            counts: HashMap::new(),
        }
    }

    fn push(&mut self, val: i32) {
        let entry = self.counts.entry(val).or_insert(0);
        *entry += 1;
        self.heap.push(Pair(val, *entry, self.index));
        self.index += 1;
    }

    fn pop(&mut self) -> i32 {
        let Pair(val, _, _) = self.heap.pop().unwrap();
        *self.counts.get_mut(&val).unwrap() -= 1;
        return val;
    }
}

fn main() {
    println!("Hello, world!");
}
