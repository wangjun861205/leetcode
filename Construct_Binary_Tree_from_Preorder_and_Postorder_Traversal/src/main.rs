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
    pub fn construct_from_pre_post(
        mut preorder: Vec<i32>,
        mut postorder: Vec<i32>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let root_val = preorder.remove(0);
        postorder.pop();
        let mut node = TreeNode::new(root_val);
        if !preorder.is_empty() {
            let mut pre_vals = Vec::new();
            let mut pos_vals = Vec::new();
            let first_pre_v = preorder[0];
            loop {
                let pre_v = preorder.remove(0);
                let pos_v = postorder.remove(0);
                pre_vals.push(pre_v);
                pos_vals.push(pos_v);
                if pos_v == first_pre_v {
                    break;
                }
            }
            node.left = Solution::construct_from_pre_post(pre_vals, pos_vals);
        }
        if !preorder.is_empty() {
            let mut pre_vals = Vec::new();
            let mut pos_vals = Vec::new();
            let first_pre_v = preorder[0];
            loop {
                let pre_v = preorder.remove(0);
                let pos_v = postorder.remove(0);
                pre_vals.push(pre_v);
                pos_vals.push(pos_v);
                if pos_v == first_pre_v {
                    break;
                }
            }
            node.right = Solution::construct_from_pre_post(pre_vals, pos_vals);
        }
        Some(Rc::new(RefCell::new(node)))
    }
}
fn main() {
    println!("Hello, world!");
}
