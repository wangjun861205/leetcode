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
    fn rc(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val == 1 {
                let mut left = node.borrow_mut().left.take();
                let mut right = node.borrow_mut().right.take();
                left = Solution::rc(left);
                right = Solution::rc(right);
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
                return Some(node);
            } else {
                let left = Solution::rc(node.borrow_mut().left.take());
                let right = Solution::rc(node.borrow_mut().right.take());
                if left.is_none() && right.is_none() {
                    return None;
                }
                node.borrow_mut().left = left;
                node.borrow_mut().right = right;
                return Some(node);
            }
        } else {
            return None;
        }
    }

    pub fn prune_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::rc(root)
    }
}
fn main() {
    println!("Hello, world!");
}
