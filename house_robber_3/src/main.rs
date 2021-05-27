use std::cell::RefCell;
use std::rc::Rc;

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

pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => return 0,
        Some(node) => {
            let left = rob(node.borrow().left.clone());
            let right = rob(node.borrow().right.clone());
            let mut left_child_val = 0;
            let mut right_child_val = 0;
            if let Some(left_node) = node.borrow().left.clone() {
                let left_left = rob(left_node.borrow().left.clone());
                let left_right = rob(left_node.borrow().right.clone());
                left_child_val = left_left + left_right;
            }
            if let Some(right_node) = node.borrow().right.clone() {
                let right_left = rob(right_node.borrow().left.clone());
                let right_right = rob(right_node.borrow().right.clone());
                right_child_val = right_left + right_right;
            }
            if left + right > node.borrow().val + left_child_val + right_child_val {
                left + right
            } else {
                node.borrow().val + left_child_val + right_child_val
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
