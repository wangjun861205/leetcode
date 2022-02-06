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
    fn dp(root: Option<Rc<RefCell<TreeNode>>>, pe: bool, gpe: bool) -> i32 {
        let mut ans = 0;
        if let Some(n) = root {
            let val = n.borrow().val;
            if gpe {
                ans += n.borrow().val;
            }
            ans += Solution::dp(n.borrow_mut().left.take(), val % 2 == 0, pe);
            ans += Solution::dp(n.borrow_mut().right.take(), val % 2 == 0, pe);
        }
        ans
    }
    pub fn sum_even_grandparent(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::dp(root, false, false)
    }
}

fn main() {
    println!("Hello, world!");
}
