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
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        if let Some(mut n) = head {
            let next = n.next.take();
            let remain_next = Solution::remove_elements(next, val);
            if n.val == val {
                return remain_next;
            } else {
                n.next = remain_next;
                return Some(n);
            }
        }
        None
    }
}
fn main() {
    println!("Hello, world!");
}
