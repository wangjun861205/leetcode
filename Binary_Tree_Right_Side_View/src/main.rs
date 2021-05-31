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
        level: usize,
        index: i32,
        max_indices: &mut Vec<i32>,
        ans: &mut Vec<i32>,
    ) {
        if let Some(root) = root {
            if index > max_indices[level] {
                max_indices[level] = index;
                ans[level] = root.borrow().val;
                Solution::dfs(
                    &root.borrow().left,
                    level + 1,
                    index * 2 + 1,
                    max_indices,
                    ans,
                );
                Solution::dfs(
                    &root.borrow().right,
                    level + 1,
                    index * 2 + 2,
                    max_indices,
                    ans,
                );
            }
        }
    }
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut max_indices = vec![-1; 100];
        let mut ans = vec![-101; 100];
        Solution::dfs(&root, 0, 0, &mut max_indices, &mut ans);
        ans.into_iter().filter(|v| v != &-101).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
