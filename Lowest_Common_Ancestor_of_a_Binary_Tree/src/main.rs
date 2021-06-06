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
        TreeNode { val, left: None, right: None }
    }
}

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32, lca: &mut Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(root) = root {
            if root.borrow().val == p || root.borrow().val == q {
                if Solution::dfs(&root.borrow().left, p, q, lca) || Solution::dfs(&root.borrow().right, p, q, lca) {
                    *lca = Some(root.clone());
                }
                return true;
            } else {
                let left = Solution::dfs(&root.borrow().left, p, q, lca);
                let right = Solution::dfs(&root.borrow().right, p, q, lca);
                if left && right {
                    *lca = Some(root.clone());
                    return true;
                }
                if left || right {
                    return true;
                }
                return false;
            }
        } else {
            return false;
        }
    }
    pub fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut lca = None;
        Solution::dfs(&root, p.as_ref().unwrap().borrow().val, q.as_ref().unwrap().borrow().val, &mut lca);
        lca
    }
}
fn main() {
    println!("Hello, world!");
}
