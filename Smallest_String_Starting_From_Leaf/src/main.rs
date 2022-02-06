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
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, mut curr: String) -> String {
        if let Some(node) = root {
            curr.insert(0, (node.borrow().val as u8 + 97) as char);
            if node.borrow().left.is_none() && node.borrow().right.is_none() {
                return curr;
            }
            if node.borrow().left.is_some() && node.borrow().right.is_some() {
                let l = Solution::dfs(node.borrow_mut().left.take(), curr.clone());
                let r = Solution::dfs(node.borrow_mut().right.take(), curr.clone());
                return l.min(r);
            }
            if node.borrow().left.is_some() {
                return Solution::dfs(node.borrow_mut().left.take(), curr.clone());
            }
            return Solution::dfs(node.borrow_mut().right.take(), curr.clone());
        }
        unreachable!()
    }
    pub fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
        Solution::dfs(root, String::new())
    }
}
fn main() {
    println!("Hello, world!");
}
