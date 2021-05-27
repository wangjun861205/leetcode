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
    fn rc(
        root: Option<&Rc<RefCell<TreeNode>>>,
        target_sum: i32,
        mut path: Vec<i32>,
        ans: &mut Vec<Vec<i32>>,
    ) {
        if let Some(rf) = root {
            let val = rf.borrow().val;
            path.push(val);
            if rf.borrow().left.is_none() && rf.borrow().right.is_none() {
                if val == target_sum {
                    ans.push(path);
                }
            } else {
                if rf.borrow().left.is_some() {
                    Solution::rc(
                        rf.borrow().left.as_ref(),
                        target_sum - val,
                        path.clone(),
                        ans,
                    );
                }
                if rf.borrow().right.is_some() {
                    Solution::rc(
                        rf.borrow().right.as_ref(),
                        target_sum - val,
                        path.clone(),
                        ans,
                    );
                }
            }
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let path = Vec::new();
        Solution::rc(root.as_ref(), target_sum, path, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
