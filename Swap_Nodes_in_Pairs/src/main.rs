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

impl Solution {
    fn rc(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            if let Some(mut p) = prev {
                let next = node.next.take();
                let swapped = Solution::rc(next, None);
                p.next = swapped;
                node.next = Some(p);
                return Some(node);
            } else {
                let next = node.next.take();
                return Solution::rc(next, Some(node));
            }
        }
        prev
    }
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::rc(head, None)
    }
}
fn main() {
    println!("Hello, world!");
}
