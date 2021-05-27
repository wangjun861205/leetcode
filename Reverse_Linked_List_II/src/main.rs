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

use std::ptr::null_mut;

impl Solution {
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut pointers: Vec<Box<ListNode>> = Vec::new();
        while let Some(mut node) = head {
            let remain = node.next.take();
            pointers.push(node);
            head = remain;
        }
        let mut l = left as usize - 1;
        let mut r = right as usize - 1;
        while l < r {
            pointers.swap(l, r);
            l += 1;
            r -= 1;
        }
        let mut h = pointers.pop().unwrap();
        while pointers.len() > 0 {
            let mut p = pointers.pop().unwrap();
            p.next = Some(h);
            h = p;
        }
        Some(h)
    }
}
fn main() {
    let mut l = Some(Box::new(ListNode {
        val: 3,
        next: Some(Box::new(ListNode {
            val: 5,
            next: Some(Box::new(ListNode { val: 7, next: None })),
        })),
    }));

    // unsafe {
    //     let p = &mut l.as_mut().unwrap().next as *mut Option<Box<ListNode>>;
    //     *p = None;
    // }
    // println!("{:?}", l);
    println!("{:?}", Solution::reverse_between(l, 1, 2));
}
