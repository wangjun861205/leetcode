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
    fn rc(root: &mut Option<Rc<RefCell<TreeNode>>>, val: i32) {
        if let Some(root) = root {
            let cur_val = root.borrow().val;
            if val < cur_val {
                Solution::rc(&mut root.borrow_mut().left, val);
            } else {
                Solution::rc(&mut root.borrow_mut().right, val);
            }
        } else {
            *root = Some(Rc::new(RefCell::new(TreeNode::new(val))));
        }
    }
    pub fn insert_into_bst(
        mut root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::rc(&mut root, val);
        root
    }
}

fn main() {
    println!("Hello, world!");
}
