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
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        if let Some(n) = root {
            let left = Solution::range_sum_bst(n.borrow_mut().left.take(), low, high);
            let right = Solution::range_sum_bst(n.borrow_mut().right.take(), low, high);
            if n.borrow().val >= low && n.borrow().val <= high {
                return left + right + n.borrow().val;
            }
            return left + right;
        }
        0
    }
}
fn main() {
    println!("Hello, world!");
}
