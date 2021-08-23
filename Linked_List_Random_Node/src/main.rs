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

use rand;
use std::ptr::null;

struct Solution {
    list: Option<Box<ListNode>>,
    pointer: *const ListNode,
}

impl Solution {
    fn new(head: Option<Box<ListNode>>) -> Self {
        let mut s = Self {
            list: head,
            pointer: null(),
        };
        s.pointer = &**s.list.as_ref().unwrap() as *const ListNode;
        s
    }

    /** Returns a random node's value. */
    fn get_random(&mut self) -> i32 {
        let mut jumps: u8 = rand::random();
        unsafe {
            while jumps > 0 {
                if let Some(next) = &(*self.pointer).next {
                    self.pointer = &**next as *const ListNode;
                } else {
                    self.pointer = &**self.list.as_ref().unwrap() as *const ListNode;
                }
                jumps -= 1;
            }
            return (*self.pointer).val;
        }
    }
}

fn main() {
    let mut s = Solution::new(Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode { val: 3, next: None })),
        })),
    })));
    println!("{}", s.get_random());
    println!("{}", s.get_random());
    println!("{}", s.get_random());
}
