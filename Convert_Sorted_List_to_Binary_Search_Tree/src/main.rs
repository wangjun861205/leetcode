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
// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;
struct Solution;

impl Solution {
    fn to_vec(head: Option<Box<ListNode>>, l: &mut Vec<i32>) {
        if let Some(node) = head {
            l.push(node.val);
            Solution::to_vec(node.next, l);
        }
    }
    fn gen_tree(l: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        match l.len() {
            0 => return None,
            1 => return Some(Rc::new(RefCell::new(TreeNode::new(l[0])))),
            _ => {
                let mid = l.len() / 2;
                let mut root = TreeNode::new(l[mid]);
                root.left = Solution::gen_tree(l[..mid].to_vec());
                if mid == l.len() - 1 {
                    root.right = None;
                } else {
                    root.right = Solution::gen_tree(l[mid + 1..].to_vec())
                }
                return Some(Rc::new(RefCell::new(root)));
            }
        }
    }
    pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut l: Vec<i32> = Vec::new();
        Solution::to_vec(head, &mut l);
        if l.len() == 0 {
            return None;
        }
        Solution::gen_tree(l)
    }
}
fn main() {
    println!("Hello, world!");
}
