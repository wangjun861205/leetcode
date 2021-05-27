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
    pub fn proc(root: Option<Rc<RefCell<TreeNode>>>, parent_val: i32) -> i32 {
        match root {
            None => return 0,
            Some(node) => {
                let right = Solution::proc(node.clone().borrow().right.clone(), parent_val);
                let val = node.clone().borrow().val;
                node.borrow_mut().val = val + right + parent_val;
                let left =
                    Solution::proc(node.clone().borrow().left.clone(), val + right + parent_val);
                val + right + left
            }
        }
    }
    pub fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::proc(root.clone(), 0);
        root
    }
}
fn main() {
    println!("Hello, world!");
}
