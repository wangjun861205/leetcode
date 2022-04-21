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

struct BSTIterator {
    root: Option<Rc<RefCell<TreeNode>>>,
}

fn next(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = &root {
        let (left_val, left) = next(node.borrow_mut().left.take());
        node.borrow_mut().left = left;
        if left_val >= 0 {
            return (left_val, Some(node.clone()));
        }
        let right = node.borrow_mut().right.take();
        return (node.borrow().val, right);
    }
    (-1, None)
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        Self { root }
    }

    fn next(&mut self) -> i32 {
        let (val, root) = next(self.root.take());
        self.root = root;
        val
    }

    fn has_next(&self) -> bool {
        self.root.is_some()
    }
}
fn main() {
    println!("Hello, world!");
}
