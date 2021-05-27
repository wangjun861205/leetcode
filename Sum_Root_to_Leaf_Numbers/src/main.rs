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
    fn rc(root: Option<Rc<RefCell<TreeNode>>>, prev: i32, sum: &mut i32) {
        if let Some(r) = root {
            let cur = prev * 10 + r.borrow().val;
            if r.borrow().left.is_none() && r.borrow().right.is_none() {
                *sum += cur;
                return;
            }
            Solution::rc(r.borrow().left.clone(), cur, sum);
            Solution::rc(r.borrow().right.clone(), cur, sum);
        }
    }
    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut sum = 0;
        Solution::rc(root, 0, &mut sum);
        sum
    }
}
fn main() {
    println!("");
}
