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
    pub fn split(head: Option<Box<ListNode>>) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        match head {
            None => return (None, None),
            Some(mut odd) => match odd.next.take() {
                None => return (Some(odd), None),
                Some(mut even) => {
                    let next_head = even.next.take();
                    let (next_odd, next_even) = Solution::split(next_head);
                    odd.next = next_odd;
                    even.next = next_even;
                    return (Some(odd), Some(even));
                }
            },
        }
    }

    pub fn append(
        head: Option<Box<ListNode>>,
        next: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match head {
            None => None,
            Some(mut node) => {
                if node.next.is_some() {
                    node.next = Solution::append(node.next, next);
                    return Some(node);
                } else {
                    node.next = next;
                    return Some(node);
                }
            }
        }
    }

    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let (mut odd, even) = Solution::split(head);
        odd = Solution::append(odd, even);
        odd
    }
}
// 1 -> 2 -> 3 -> 4 -> 5
fn main() {
    let mut first = ListNode::new(1);
    let mut second = ListNode::new(2);
    let mut third = ListNode::new(3);
    let mut forth = ListNode::new(4);
    let mut fifth = ListNode::new(5);
    forth.next = Some(Box::new(fifth));
    third.next = Some(Box::new(forth));
    second.next = Some(Box::new(third));
    first.next = Some(Box::new(second));
    let l = Solution::odd_even_list(Some(Box::new(first)));
    println!("{:?}", l);
}
