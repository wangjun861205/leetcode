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
    fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            Solution::inorder_traversal(&node.borrow().left, vals);
            vals.push(node.borrow().val);
            Solution::inorder_traversal(&node.borrow().right, vals);
        }
    }

    fn construct(vals: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return None;
        }
        let mid = start + (end - start) / 2;
        let mut root = TreeNode::new(vals[mid]);
        let left = Solution::construct(vals, start, mid);
        let right = Solution::construct(vals, mid + 1, end);
        root.left = left;
        root.right = right;
        Some(Rc::new(RefCell::new(root)))
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = Vec::new();
        Solution::inorder_traversal(&root, &mut vals);
        Solution::construct(&vals, 0, vals.len())
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::balance_bst(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode { val: 4, left: None, right: None })))
                })))
            })))
        }))))
    );
}
