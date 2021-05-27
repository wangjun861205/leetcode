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
    pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        let mut h: Vec<Box<ListNode>> = Vec::new();
        let mut t: Vec<Box<ListNode>> = Vec::new();
        while let Some(mut n) = head {
            head = n.next;
            n.next = None;
            if n.val < x {
                h.push(n);
            } else {
                t.push(n);
            }
        }
        h.append(&mut t);
        if h.len() == 0 {
            return None;
        }
        let mut last = h.pop().unwrap();
        while h.len() > 0 {
            let mut n = h.pop().unwrap();
            n.next = Some(last);
            last = n;
        }
        Some(last)
    }
}
fn main() {
    println!("Hello, world!");
}
