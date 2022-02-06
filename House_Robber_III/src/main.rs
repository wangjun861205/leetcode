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
    fn dp(root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(node) = root {
            let (left_rob, left_pass) = Solution::dp(&node.borrow().left);
            let (right_rob, right_pass) = Solution::dp(&node.borrow().right);
            let curr_val = node.borrow().val;
            let rob = curr_val + left_pass + right_pass;
            let pass = left_rob.max(left_pass) + right_rob.max(right_pass);
            return (rob, pass);
        }
        (0, 0)
    }
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (rob, pass) = Solution::dp(&root);
        rob.max(pass)
    }
}
fn main() {
    println!("Hello, world!");
}
