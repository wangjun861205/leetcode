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
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        mut parents: Vec<i32>,
        target_sum: i32,
        ans: &mut i32,
    ) {
        if let Some(root) = root {
            for i in 0..parents.len() {
                if parents[i..].iter().sum::<i32>() + root.borrow().val == target_sum {
                    *ans += 1;
                }
            }
            if root.borrow().val == target_sum {
                *ans += 1;
            }
            parents.push(root.borrow().val);
            Solution::dfs(&root.borrow().left, parents.clone(), target_sum, ans);
            Solution::dfs(&root.borrow().right, parents.clone(), target_sum, ans);
        }
    }
    pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
        let mut ans = 0;
        Solution::dfs(&root, Vec::new(), target_sum, &mut ans);
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
