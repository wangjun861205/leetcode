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
    fn rc(root: Option<&Rc<RefCell<TreeNode>>>, level: usize, ans: &mut Vec<Vec<i32>>) {
        if let Some(rf) = root {
            if level == ans.len() {
                ans.insert(0, Vec::new());
            }
            Solution::rc(rf.borrow().left.as_ref(), level + 1, ans);
            Solution::rc(rf.borrow().right.as_ref(), level + 1, ans);
            let len = ans.len();
            ans[len - level - 1].push(rf.borrow().val);
        }
    }
    pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        Solution::rc(root.as_ref(), 0, &mut ans);
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
