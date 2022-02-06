struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    fn gen_list(mut heap: BinaryHeap<Reverse<i32>>) -> Option<Box<ListNode>> {
        if let Some(node) = heap.pop() {
            let mut head = Box::new(ListNode::new(node.0));
            let next = Solution::gen_list(heap);
            head.next = next;
            return Some(head);
        }
        None
    }
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        for mut l in lists {
            while let Some(mut n) = l {
                let next = n.next.take();
                heap.push(Reverse(n.val));
                l = next;
            }
        }
        Solution::gen_list(heap)
    }
}

fn main() {
    println!("Hello, world!");
}
