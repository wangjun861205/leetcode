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

use std::ptr::null_mut;

impl Solution {
    fn reverse(head: Option<Box<ListNode>>, prev: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut h) = head {
            let next = h.next.take();
            h.next = prev;
            return Solution::reverse(next, Some(h));
        }
        prev
    }
    fn merge(head: &mut Option<Box<ListNode>>, other: Option<Box<ListNode>>) {
        if let Some(mut o) = other {
            let mut h = head.as_mut().unwrap();
            let next = h.next.take();
            let other_next = o.next;
            o.next = next;
            h.next = Some(o);
            Solution::merge(&mut h.next.as_mut().unwrap().next, other_next);
        }
    }
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        if head.is_none() {
            return;
        }
        let mut slow: *mut Option<Box<ListNode>> = head;
        let mut fast: *mut Option<Box<ListNode>> = head;
        unsafe {
            loop {
                let f = fast.as_mut().unwrap().as_mut().unwrap();
                if let Some(next) = f.next.as_mut() {
                    if next.next.is_some() {
                        slow = &mut slow.as_mut().unwrap().as_mut().unwrap().next
                            as *mut Option<Box<ListNode>>;
                        fast = &mut next.next as *mut Option<Box<ListNode>>;
                        continue;
                    }
                }
                break;
            }
            let mut second_half = (*slow).as_mut().unwrap().next.take();
            second_half = Solution::reverse(second_half, None);
            Solution::merge(head, second_half)
        }
    }
}
fn main() {
    let mut l = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode { val: 4, next: None })),
            })),
        })),
    }));
    Solution::reorder_list(&mut l);
    println!("{:?}", l);
}
