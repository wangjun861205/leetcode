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
impl Solution {
    fn insert(head: Option<Box<ListNode>>, node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if node.is_none() {
            return head;
        }
        if head.is_none() {
            return node;
        }
        let mut h = head.unwrap();
        let mut n = node.unwrap();
        if n.val > h.val {
            let next = Solution::insert(h.next.take(), Some(n));
            h.next = next;
            return Some(h);
        } else {
            n.next = Some(h);
            return Some(n);
        }
    }
    pub fn insertion_sort_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut sorted: Option<Box<ListNode>> = None;
        while let Some(mut n) = head {
            let next = n.next.take();
            sorted = Solution::insert(sorted, Some(n));
            head = next;
        }
        sorted
    }
}
fn main() {
    println!("Hello, world!");
}
