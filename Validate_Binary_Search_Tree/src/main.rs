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
    fn rc(root: &Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(r) = root {
            if !Solution::rc(&r.borrow().left, prev) {
                return false;
            }
            if let Some(p) = prev {
                if p.borrow().val >= r.borrow().val {
                    return false;
                }
            }
            *prev = Some(r.clone());
            if !Solution::rc(&r.borrow().right, prev) {
                return false;
            }
        }
        true
    }
    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut prev = None;
        Solution::rc(&root, &mut prev)
    }
}
fn main() {
    println!("Hello, world!");
}
