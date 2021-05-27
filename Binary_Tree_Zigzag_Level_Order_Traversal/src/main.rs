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

struct Solution;

use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traversal(root: Option<&Rc<RefCell<TreeNode>>>, level: usize, ans: &mut Vec<Vec<i32>>) {
        if let Some(rf) = root {
            if level % 2 == 0 {
                if let Some(v) = ans.get_mut(level) {
                    v.push(rf.borrow().val);
                } else {
                    ans.push(vec![rf.borrow().val]);
                }
            } else {
                if let Some(v) = ans.get_mut(level) {
                    v.insert(0, rf.borrow().val);
                } else {
                    ans.push(vec![rf.borrow().val]);
                }
            }
            Solution::traversal(rf.borrow().left.as_ref(), level + 1, ans);
            Solution::traversal(rf.borrow().right.as_ref(), level + 1, ans);
        }
    }
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Solution::traversal(root.as_ref(), 0, &mut ans);
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
