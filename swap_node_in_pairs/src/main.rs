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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match head.clone() {
            None => head,
            Some(mut node) => match node.next.clone() {
                None => head,
                Some(mut next) => {
                    let next_next = next.next.clone();
                    node.next = Solution::swap_pairs(next_next);
                    next.next = Some(node.clone());
                    Some(next)
                }
            },
        }
    }
}

struct node {
    val: i32,
}

fn main() {
    let mut n1 = Box::new(ListNode::new(1));
    let mut n2 = Box::new(ListNode::new(2));
    let mut n3 = Box::new(ListNode::new(3));
    let mut n4 = Box::new(ListNode::new(4));
    n3.next = Some(n4);
    n2.next = Some(n3);
    n1.next = Some(n2);
    println!("{:?}", Solution::swap_pairs(Some(n1)));
}
