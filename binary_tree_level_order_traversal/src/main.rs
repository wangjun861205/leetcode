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

struct Solution;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        if root.is_none() {
            return ans;
        }
        queue.push(root);
        while queue.len() > 0 {
            let mut q: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();
            let mut vals: Vec<i32> = Vec::new();
            queue.into_iter().for_each(|v| {
                if let Some(node) = v {
                    q.push(node.borrow().left.clone());
                    q.push(node.borrow().right.clone());
                    vals.push(node.borrow().val);
                }
            });
            queue = q;
            if vals.len() > 0 {
                ans.push(vals);
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
