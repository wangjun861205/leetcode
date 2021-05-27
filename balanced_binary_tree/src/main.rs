use std::cell::RefCell;
use std::rc::Rc;

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

struct Solution;

impl Solution {
    pub fn height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let left_height = Solution::height(root.as_ref().unwrap().borrow().left.clone());
        let right_height = Solution::height(root.as_ref().unwrap().borrow().right.clone());
        left_height.max(right_height) + 1
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if root.is_none() {
            return true;
        }
        let is_left_balance = Solution::is_balanced(root.as_ref().unwrap().borrow().left.clone());
        let is_right_balance = Solution::is_balanced(root.as_ref().unwrap().borrow().right.clone());
        if !(is_left_balance && is_right_balance) {
            return false;
        }
        let left_height = Solution::height(root.as_ref().unwrap().borrow().left.clone());
        let right_height = Solution::height(root.as_ref().unwrap().borrow().right.clone());
        (left_height - right_height).abs() <= 1
    }
}
fn main() {}
