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
    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut l: Vec<i32> = Vec::new();
        match root {
            None => return l,
            Some(n) => {
                l.append(&mut Solution::postorder_traversal(n.borrow().left.clone()));
                l.append(&mut Solution::postorder_traversal(n.borrow().right.clone()));
                l.push(n.borrow().val);
            }
        }
        l
    }
}
fn main() {
    println!("Hello, world!");
}
