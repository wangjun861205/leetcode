struct Solution;

use std::cmp::{Ordering, Reverse};
use std::collections::BinaryHeap;

#[derive(Eq, PartialEq)]
struct Node(i32, usize, usize);

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, mut k: i32) -> i32 {
        let mut heap: BinaryHeap<Reverse<Node>> = BinaryHeap::new();
        for i in 0..matrix.len() {
            heap.push(Reverse(Node(matrix[i][0], i, 0)));
        }
        let mut count = 0;
        let mut result = 0;
        while !heap.is_empty() {
            let mut node = heap.pop().unwrap().0;
            result = node.0;
            count += 1;
            if count == k {
                break;
            }
            if node.2 < matrix[0].len() - 1 {
                node.2 += 1;
                node.0 = matrix[node.1][node.2];
                heap.push(Reverse(node));
            }
        }
        result
    }
}

fn main() {
    println!("Hello, world!");
}
