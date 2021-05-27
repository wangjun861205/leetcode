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
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            None => true,
            Some(node) => {
                if node.borrow().left.is_some() {
                    if node.borrow().val < node.borrow().left.as_ref().unwrap().borrow().val {
                        return false;
                    }
                }
                if node.borrow().right.is_some() {
                    if node.borrow().val > node.borrow().right.as_ref().unwrap().borrow().val {
                        return false;
                    }
                }
                if node.borrow().left.is_some() {
                    if !Solution::is_valid_bst(node.borrow().left.clone()) {
                        return false;
                    }
                }
                if node.borrow().right.is_some() {
                    if !Solution::is_valid_bst(node.borrow().right.clone()) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
