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
    fn rc(head: Option<Box<ListNode>>, left: i32, right: i32, cur: i32, stack: &mut Vec<Box<ListNode>>) -> Option<Box<ListNode>> {
        if let Some(mut node) = head {
            let next = node.next.take();
            if cur < left {
                let remain = Solution::rc(next, left, right, cur + 1, stack);
                node.next = remain;
                return Some(node);
            } else if cur >= left && cur <= right {
                stack.push(node);
                let remain = Solution::rc(next, left, right, cur + 1, stack);
                let mut rev = stack.remove(0);
                rev.next = remain;
                return Some(rev);
            } else {
                node.next = next;
                return Some(node);
            }
        }
        None
    }
    pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
        let mut stack = Vec::new();
        Solution::rc(head, left, right, 1, &mut stack)
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
