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

use std::rc::Rc;
use std::{cell::RefCell, ops::DerefMut};

struct Solution;

impl Solution {
    pub unsafe fn remove(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut p1: *mut ListNode = head.as_mut().unwrap().as_mut();
        let mut p2 = p1.clone();
        let mut count = 0;
        while p1.as_mut().unwrap().next.as_ref().is_some() {
            if count < n {
                p1 = p1.as_mut().unwrap().next.as_mut().unwrap().as_mut();
            } else {
                p1 = p1.as_mut().unwrap().next.as_mut().unwrap().as_mut();
                p2 = p2.as_mut().unwrap().next.as_mut().unwrap().as_mut();
            }
            count += 1;
        }
        if p1 == p2 {
            return None;
        }
        if count < n {
            return p2.as_ref().unwrap().next.clone();
        }
        p2.as_mut().unwrap().next = p2.as_mut().unwrap().next.as_mut().unwrap().next.clone();
        head
    }
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        unsafe { Solution::remove(head, n) }
    }
}
fn main() {
    unsafe {
        println!(
            "{:?}",
            Solution::remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode { val: 2, next: None }))
                })),
                2
            )
        )
    };
}
