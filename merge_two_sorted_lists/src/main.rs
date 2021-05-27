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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() && l2.is_none() {
            return None;
        }
        if l1.is_none() && l2.is_some() {
            return l2;
        }
        if l1.is_some() && l2.is_none() {
            return l1;
        }
        let mut n1 = l1.clone().unwrap();
        let mut n2 = l2.clone().unwrap();
        let r1 = l1.clone().unwrap().next;
        let r2 = l2.clone().unwrap().next;
        if n1.val < n2.val {
            n1.next = Solution::merge_two_lists(r1, Some(n2));
            return Some(n1);
        } else {
            n2.next = Solution::merge_two_lists(Some(n1), r2);
            return Some(n2);
        }
    }
}
fn main() {
    println!("Hello, world!");
}
