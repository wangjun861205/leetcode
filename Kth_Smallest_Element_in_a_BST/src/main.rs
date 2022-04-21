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
    fn inorder(root: Option<Rc<RefCell<TreeNode>>>, k: &mut i32, ans: &mut i32) {
        if let Some(node) = root {
            if *k == 0 {
                return;
            }
            Solution::inorder(node.borrow_mut().left.take(), k, ans);
            if *k == 0 {
                return;
            }
            *k -= 1;
            if *k == 0 {
                *ans = node.borrow().val;
            }
            Solution::inorder(node.borrow_mut().right.take(), k, ans);
        }
    }
    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, mut k: i32) -> i32 {
        let mut ans = 0;
        Solution::inorder(root, &mut k, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
