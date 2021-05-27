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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        match lists.len() {
            0 => return None,
            1 => return lists[0].clone(),
            2 => {
                let (n1, n2) = (lists[0].clone(), lists[1].clone());
                if n1.is_none() && n2.is_none() {
                    return None;
                }
                if n1.is_some() && n2.is_none() {
                    return n1;
                }
                if n1.is_none() && n2.is_some() {
                    return n2;
                }
                if n1.clone().unwrap().val < n2.clone().unwrap().val {
                    let mut node = n1.clone().unwrap();
                    node.next = Solution::merge_k_lists(vec![node.next, n2.clone()]);
                    return Some(node);
                } else {
                    let mut node = n2.clone().unwrap();
                    node.next = Solution::merge_k_lists(vec![n1.clone(), node.next]);
                    return Some(node);
                }
            }
            _ => {
                let half_idx = lists.len() / 2;
                let first_half = Solution::merge_k_lists(lists[..half_idx].to_vec());
                let second_half = Solution::merge_k_lists(lists[half_idx..].to_vec());
                return Solution::merge_k_lists(vec![first_half, second_half]);
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
