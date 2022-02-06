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
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, mut prefix: i32, sum: &mut i32) {
        if let Some(node) = root {
            prefix = prefix * 2 + node.borrow().val;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                *sum += prefix;
                return;
            }
            Solution::dfs(&node.borrow().left, prefix, sum);
            Solution::dfs(&node.borrow().right, prefix, sum);
        }
    }
    pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Solution::dfs(&root, 0, &mut sum);
        sum
    }
}

fn main() {
    println!("Hello, world!");
}
