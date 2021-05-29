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
    fn get_left_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            Solution::get_left_height(&root.borrow().left) + 1
        } else {
            0
        }
    }

    fn get_right_height(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            Solution::get_right_height(&root.borrow().right) + 1
        } else {
            0
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let left_height = Solution::get_left_height(&root.borrow().left) + 1;
            let right_height = Solution::get_right_height(&root.borrow().right) + 1;
            if left_height == right_height {
                return 2_i32.pow(left_height as u32) - 1;
            } else {
                let left_count = Solution::count_nodes(root.borrow().left.clone());
                let right_count = Solution::count_nodes(root.borrow().right.clone());
                return left_count + right_count + 1;
            }
        } else {
            return 0;
        }
    }
}
fn main() {
    println!("Hello, world!");
}
