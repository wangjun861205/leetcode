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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, parent: i32, parents: &mut Vec<i32>) {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let val = node.borrow().val;
            parents[val as usize] = parent;
            Solution::dfs(left, val, parents);
            Solution::dfs(right, val, parents);
        }
    }
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut parents1 = vec![-1; 100];
        let mut parents2 = vec![-1; 100];
        Solution::dfs(root1, -2, &mut parents1);
        Solution::dfs(root2, -2, &mut parents2);
        for (p1, p2) in parents1.into_iter().zip(parents2) {
            if p1 != p2 {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
