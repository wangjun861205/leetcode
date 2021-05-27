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
    fn rc(root: Option<Rc<RefCell<TreeNode>>>, level: usize, table: &mut Vec<Vec<i32>>) -> bool {
        if let Some(r) = root {
            if r.borrow().val % 2 == (level as i32) % 2 {
                return false;
            } else {
                if let Some(l) = table.get_mut(level) {
                    if level % 2 == 0 {
                        if l.last().unwrap() >= &r.borrow().val {
                            return false;
                        }
                    } else {
                        if l.last().unwrap() <= &r.borrow().val {
                            return false;
                        }
                    }
                    l.push(r.borrow().val);
                } else {
                    table.push(vec![r.borrow().val]);
                }
            }
            if !Solution::rc(r.borrow().left.clone(), level + 1, table) {
                return false;
            }
            if !Solution::rc(r.borrow().right.clone(), level + 1, table) {
                return false;
            }
            return true;
        } else {
            return true;
        }
    }
    pub fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut table: Vec<Vec<i32>> = Vec::new();
        Solution::rc(root, 0, &mut table)
    }
}
fn main() {
    println!("Hello, world!");
}
