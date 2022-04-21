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
    pub fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut stack = vec![root];
        let mut level = 0;
        let mut max = i32::MIN;
        let mut max_level = 0;
        while !stack.is_empty() {
            level += 1;
            let mut sum = 0;
            let mut new_stack = Vec::new();
            let mut valid = false;
            for node in stack {
                if let Some(n) = node {
                    valid = true;
                    sum += n.borrow().val;
                    new_stack.push(n.borrow_mut().left.take());
                    new_stack.push(n.borrow_mut().right.take());
                }
            }
            if valid {
                stack = new_stack;
                if sum > max {
                    max = sum;
                    max_level = level;
                }
                continue;
            }
            break;
        }
        max_level
    }
}

fn main() {
    println!("Hello, world!");
}
