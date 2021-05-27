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
    pub fn rotate_right(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if k == 0 {
            return head;
        }
        let mut count = 0;
        let mut h = &head;
        while let Some(node) = h {
            count += 1;
            h = &node.next;
        }
        if count == 0 || count == 1 || k % count == 0 {
            return head;
        }
        let mut h = &mut head;
        let mut e = None;
        let mut n = count - k % count;
        while n > 0 {
            if n == 1 {
                let node = h.as_mut().unwrap();
                e = node.next.take();
                break;
            }
            h = &mut h.as_mut().unwrap().next;
            n -= 1;
        }
        let mut new_head = e;
        let mut h = &mut new_head;
        while let Some(node) = h {
            if node.next.is_none() {
                node.next = head;
                break;
            }
            h = &mut node.next;
        }
        new_head
    }
}
fn main() {
    println!("Hello, world!");
}
