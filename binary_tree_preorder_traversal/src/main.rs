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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut l: Vec<i32> = Vec::new();
        match root {
            None => return l,
            Some(node) => {
                l.push(node.clone().borrow().val);
                l.append(&mut Solution::preorder_traversal(
                    node.borrow().left.clone(),
                ));
                l.append(&mut Solution::preorder_traversal(
                    node.borrow().right.clone(),
                ));
            }
        }
        l
    }
}
fn main() {
    println!("Hello, world!");
}
