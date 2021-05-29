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
    pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.is_empty() {
            return None;
        }
        let max = nums.iter().max().unwrap();
        let max_position = nums.iter().position(|v| v == max).unwrap();
        let mut node = TreeNode::new(*max);
        node.left = Solution::construct_maximum_binary_tree(nums[..max_position].to_vec());
        node.right = Solution::construct_maximum_binary_tree(nums[max_position + 1..].to_vec());
        Some(Rc::new(RefCell::new(node)))
    }
}

fn main() {
    println!("Hello, world!");
}
