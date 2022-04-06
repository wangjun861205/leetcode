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

use std::collections::BinaryHeap;

impl Solution {
    pub fn sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        while let Some(mut h) = head {
            let next = h.next.take();
            heap.push(h.val);
            head = next;
        }
        let mut head = None;
        while let Some(n) = heap.pop() {
            let mut node = Box::new(ListNode::new(n));
            node.next = head;
            head = Some(node);
        }
        head
    }
}

fn main() {
    println!("Hello, world!");
}
