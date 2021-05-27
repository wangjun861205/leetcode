// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
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

struct Solution;
use std::rc::Rc;
use std::{cell::RefCell, ops::Deref};
impl Solution {
    fn rc(
        root: Option<Rc<RefCell<TreeNode>>>,
        deepth: i32,
    ) -> (Option<Rc<RefCell<TreeNode>>>, i32) {
        let node = root.as_ref().unwrap().borrow();
        if node.left.is_none() && node.right.is_none() {
            return (root.clone(), deepth + 1);
        }
        if node.left.is_some() && node.right.is_none() {
            return Solution::rc(node.left.clone(), deepth + 1);
        }
        if node.left.is_none() && node.right.is_some() {
            return Solution::rc(node.right.clone(), deepth + 1);
        }
        let (lv, ld) = Solution::rc(node.left.clone(), deepth + 1);
        let (rv, rd) = Solution::rc(node.right.clone(), deepth + 1);
        if ld == rd {
            return (root.clone(), ld);
        } else {
            if ld < rd {
                return (rv, rd);
            } else {
                return (lv, ld);
            }
        }
    }

    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (v, _) = Solution::rc(root.clone(), 0);
        if v.is_none() {
            root
        } else {
            v
        }
    }
}
fn main() {
    println!("Hello, world!");
}
