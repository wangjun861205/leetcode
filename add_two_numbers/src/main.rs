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
    pub fn add(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
        addition: i32,
    ) -> Option<Box<ListNode>> {
        if l1.is_none() {
            if let Some(node) = l2 {
                return Solution::add(Some(Box::new(ListNode::new(addition))), Some(node), 0);
            } else {
                if addition > 0 {
                    return Some(Box::new(ListNode::new(addition)));
                } else {
                    return None;
                }
            }
        }
        if l2.is_none() {
            return Solution::add(l1, Some(Box::new(ListNode::new(addition))), 0);
        }
        let node1 = l1.unwrap();
        let node2 = l2.unwrap();
        let sum = node1.as_ref().val + node2.as_ref().val + addition;
        let add = sum / 10;
        let num = sum % 10;
        let next = Solution::add(node1.next.clone(), node2.next.clone(), add);
        let mut node = ListNode::new(num);
        node.next = next;
        Some(Box::new(node))
    }
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        Solution::add(l1, l2, 0)
    }
}
fn main() {
    println!("Hello, world!");
}
