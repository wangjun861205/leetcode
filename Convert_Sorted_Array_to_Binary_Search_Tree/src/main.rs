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
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        let mut root = TreeNode::new(nums[nums.len() / 2]);
        let left = Solution::sorted_array_to_bst(nums[..nums.len() / 2].to_vec());
        let right = Solution::sorted_array_to_bst(nums[nums.len() / 2 + 1..].to_vec());
        root.left = left;
        root.right = right;
        Some(Rc::new(RefCell::new(root)))
    }
}
fn main() {
    println!("Hello, world!");
}
