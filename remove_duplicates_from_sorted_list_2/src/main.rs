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
    pub fn delete(head: Option<Box<ListNode>>, is_dup: bool) -> Option<Box<ListNode>> {
        if is_dup {
            if let Some(node) = head {
                if let Some(next) = node.next {
                    if node.val == next.val {
                        return Solution::delete(Some(next), true);
                    } else {
                        return Solution::delete(Some(next), false);
                    }
                } else {
                    return None;
                }
            } else {
                return None;
            }
        } else {
            if let Some(mut node) = head {
                if let Some(next) = node.next {
                    if node.val == next.val {
                        return Solution::delete(Some(next), true);
                    } else {
                        node.next = Solution::delete(Some(next), false);
                        return Some(node);
                    }
                } else {
                    return Some(node);
                }
            } else {
                return None;
            }
        }
    }

    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Solution::delete(head, false)
    }
}

fn main() {
    println!("Hello, world!");
}
