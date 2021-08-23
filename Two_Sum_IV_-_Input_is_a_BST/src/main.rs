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

use std::collections::HashSet;
impl Solution {
    fn find(root: &Option<Rc<RefCell<TreeNode>>>, k: i32, targets: &mut HashSet<i32>) -> bool {
        if let Some(n) = root {
            if targets.contains(&n.borrow().val) {
                return true;
            }
            targets.insert(k - n.borrow().val);
            if Solution::find(&n.borrow().left, k, targets) {
                return true;
            }
            if Solution::find(&n.borrow().right, k, targets) {
                return true;
            }
        }
        false
    }

    pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
        let mut targets = HashSet::new();
        Solution::find(&root, k, &mut targets)
    }
}
fn main() {
    println!("Hello, world!");
}
