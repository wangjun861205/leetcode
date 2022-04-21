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
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val < low {
                return Solution::trim_bst(node.borrow_mut().right.take(), low, high);
            }
            if node.borrow().val > high {
                return Solution::trim_bst(node.borrow_mut().left.take(), low, high);
            }
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Solution::trim_bst(left, low, high);
            node.borrow_mut().right = Solution::trim_bst(right, low, high);
            return Some(node);
        }
        None
    }
}

fn main() {
    println!("Hello, world!");
}
