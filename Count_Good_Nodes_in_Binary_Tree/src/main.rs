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
    fn rc(root: Option<Rc<RefCell<TreeNode>>>, max: i32) -> i32 {
        if let Some(node) = root {
            if node.borrow().val < max {
                let left = Solution::rc(node.borrow().left.clone(), max);
                let right = Solution::rc(node.borrow().right.clone(), max);
                return left + right;
            } else {
                let left = Solution::rc(node.borrow().left.clone(), node.borrow().val);
                let right = Solution::rc(node.borrow().right.clone(), node.borrow().val);
                return left + right + 1;
            }
        } else {
            return 0;
        }
    }
    pub fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let val = root.as_ref().unwrap().borrow().val;
        Solution::rc(root, val)
    }
}
fn main() {
    println!("Hello, world!");
}
