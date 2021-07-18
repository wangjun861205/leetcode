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
    fn reverse(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return None;
        }
        let remain = head.as_mut().unwrap().next.take();
        let next = Solution::reverse(remain);
        Solution::append(next, head)
    }

    fn append(mut head: Option<Box<ListNode>>, node: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return node;
        }
        unsafe {
            let mut p: *mut Option<Box<ListNode>> = &mut head;
            while (*p).as_mut().unwrap().next.is_some() {
                p = &mut (*p).as_mut().unwrap().next;
            }
            (*p).as_mut().unwrap().next = node;
        }
        head
    }

    fn rc(mut head: Option<Box<ListNode>>, k: i32, mut prev: Option<Box<ListNode>>, curr: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return Solution::reverse(prev);
        } else {
            let remain = head.as_mut().unwrap().next.take();
            prev = Solution::append(head, prev);
            if curr == k {
                let next = Solution::rc(remain, k, None, 1);
                return Solution::append(prev, next);
            } else {
                return Solution::rc(remain, k, prev, curr + 1);
            }
        }
    }
    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        Solution::rc(head, k, None, 1)
    }
}
fn main() {
    let list = Some(Box::new(ListNode {
        val: 1,
        next: Some(Box::new(ListNode {
            val: 2,
            next: Some(Box::new(ListNode {
                val: 3,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode { val: 5, next: None })),
                })),
            })),
        })),
    }));
    // println!("{:?}", Solution::reverse(list));
    println!("{:?}", Solution::reverse_k_group(list, 3));
}
