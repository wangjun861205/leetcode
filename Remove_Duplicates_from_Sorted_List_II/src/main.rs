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

// 1, 2, 3, 3, 4, 4, 5

use std::collections::HashMap;

impl Solution {
    fn dedup(head: Option<Box<ListNode>>, m: &mut HashMap<i32, i32>) -> Option<Box<ListNode>> {
        if let Some(mut h) = head {
            *m.entry(h.val).or_insert(0) += 1;
            let next = h.next.take();
            let n = Solution::dedup(next, m);
            if m.get(&h.val).unwrap() == &1 {
                h.next = n;
                return Some(h);
            } else {
                return n;
            }
        }
        None
    }
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::dedup(head, &mut HashMap::new())
    }
}
fn main() {
    println!("Hello, world!");
}
