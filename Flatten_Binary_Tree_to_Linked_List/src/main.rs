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
    fn rc(root: &mut Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(rf) = root {
            Solution::rc(&mut rf.borrow_mut().right, prev);
            Solution::rc(&mut rf.borrow_mut().left, prev);
            rf.borrow_mut().right = prev.clone();
            rf.borrow_mut().left = None;
            *prev = Some(rf.clone());
        }
    }
    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut prev = None;
        Solution::rc(root, &mut prev);
    }
}
fn main() {
    println!("Hello, world!");
}
