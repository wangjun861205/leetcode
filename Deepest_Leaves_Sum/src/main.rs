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
    fn rc(root: Option<Rc<RefCell<TreeNode>>>, max_depth: &mut i32, mut depth: i32, sum: &mut i32) {
        if let Some(node) = root {
            depth += 1;
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                if depth == *max_depth {
                    *sum += node.borrow().val;
                } else if depth > *max_depth {
                    *max_depth = depth;
                    *sum = node.borrow().val;
                }
            } else {
                Solution::rc(node.borrow().left.clone(), max_depth, depth, sum);
                Solution::rc(node.borrow().right.clone(), max_depth, depth, sum);
            }
        }
    }
    pub fn deepest_leaves_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_depth = 0;
        let mut sum = 0;
        Solution::rc(root, &mut max_depth, 0, &mut sum);
        sum
    }
}
fn main() {
    println!("Hello, world!");
}
