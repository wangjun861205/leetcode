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

use std::rc::Rc;
use std::{cell::RefCell, ops::Deref};

impl Solution {
    fn rc(root: Option<Rc<RefCell<TreeNode>>>, deepth: i32) -> (i32, i32) {
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_none() && node.right.is_none() {
            return (node.val, deepth + 1);
        }
        if node.left.is_some() && node.right.is_none() {
            return Solution::rc(node.left.clone(), deepth + 1);
        }
        if node.left.is_none() && node.right.is_some() {
            return Solution::rc(node.right.clone(), deepth + 1);
        }
        let left = Solution::rc(node.left.clone(), deepth + 1);
        let right = Solution::rc(node.right.clone(), deepth + 1);
        if left.1 >= right.1 {
            left
        } else {
            right
        }
    }
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (val, _) = Solution::rc(root, 1);
        val
    }
}
fn main() {
    println!("Hello, world!");
}
