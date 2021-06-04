struct Solution;
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
impl Solution {
    pub fn bst_from_preorder(mut preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.is_empty() {
            return None;
        }
        let node_val = preorder.remove(0);
        let mut node = TreeNode::new(node_val);
        let left_vals: Vec<i32> = preorder
            .iter()
            .filter(|&v| v < &node_val)
            .map(|v| *v)
            .collect();
        let right_vals: Vec<i32> = preorder
            .iter()
            .filter(|&v| v > &node_val)
            .map(|v| *v)
            .collect();
        node.left = Solution::bst_from_preorder(left_vals);
        node.right = Solution::bst_from_preorder(right_vals);
        Some(Rc::new(RefCell::new(node)))
    }
}
fn main() {
    println!("Hello, world!");
}
