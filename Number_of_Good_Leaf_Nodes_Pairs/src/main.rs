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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32, ans: &mut i32) -> Vec<i32> {
        if let Some(mut n) = root {
            let left = Solution::dfs(n.borrow_mut().left.take(), distance, ans);
            let right = Solution::dfs(n.borrow_mut().right.take(), distance, ans);
            if left.is_empty() && right.is_empty() {
                return vec![1];
            }
            for l in &left {
                for r in &right {
                    if *l + *r <= distance {
                        *ans += 1;
                    }
                }
            }
            return left
                .into_iter()
                .chain(right.into_iter())
                .map(|d| d + 1)
                .collect();
        }
        return vec![];
    }
    pub fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
        let mut ans = 0;
        Solution::dfs(root, distance, &mut ans);
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
