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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &head;
        let mut slow = &head;
        while let Some(f) = fast {
            let one = &f.next;
            if one.is_none() {
                return slow.clone();
            }
            slow = &slow.as_ref().unwrap().next;
            fast = &one.as_ref().unwrap().next;
        }
        slow.clone()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::middle_node(Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode { val: 4, next: None }))
                }))
            }))
        })))
    );
}
