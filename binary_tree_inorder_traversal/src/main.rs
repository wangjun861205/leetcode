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

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut l: Vec<i32> = Vec::new();
        match root {
            None => return l,
            Some(n) => {
                l.append(&mut Solution::inorder_traversal(n.borrow().left.clone()));
                l.push(n.borrow().val);
                l.append(&mut Solution::inorder_traversal(n.borrow().right.clone()));
            }
        }
        l
    }
}
fn main() {
    println!("Hello, world!");
}
