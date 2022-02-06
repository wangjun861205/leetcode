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
    fn count(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(n) = root {
            let left = Solution::count(&n.borrow().left);
            let right = Solution::count(&n.borrow().right);
            return left + right + 1;
        }
        0
    }
    fn count_x(root: &Option<Rc<RefCell<TreeNode>>>, x: i32) -> Option<(i32, i32, i32)> {
        if let Some(n) = root {
            if n.borrow().val == x {
                let left = Solution::count(&n.borrow().left);
                let right = Solution::count(&n.borrow().right);
                return Some((left + right + 1, left, right));
            } else {
                let left = Solution::count_x(&n.borrow().left, x);
                if left.is_some() {
                    return left;
                }
                let right = Solution::count_x(&n.borrow().right, x);
                return right;
            }
        }
        None
    }

    pub fn btree_game_winning_move(root: Option<Rc<RefCell<TreeNode>>>, n: i32, x: i32) -> bool {
        let total = Solution::count(&root);
        if let Some((x, left, right)) = Solution::count_x(&root, x) {
            return total - x > x || total - left < left || total - right < right;
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
