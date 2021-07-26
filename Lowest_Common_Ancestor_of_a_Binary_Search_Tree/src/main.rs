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
    fn find(
        root: &Option<Rc<RefCell<TreeNode>>>,
        p: i32,
        q: i32,
    ) -> (bool, bool, Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            if node.borrow().val == p {
                let (_, left_q, _) = Solution::find(&node.borrow().left, p, q);
                if left_q {
                    return (true, true, Some(node.clone()));
                }
                let (_, right_q, _) = Solution::find(&node.borrow().right, p, q);
                if right_q {
                    return (true, true, Some(node.clone()));
                }
                return (true, false, None);
            }
            if node.borrow().val == q {
                let (left_p, _, _) = Solution::find(&node.borrow().left, p, q);
                if left_p {
                    return (true, true, Some(node.clone()));
                }
                let (right_p, _, _) = Solution::find(&node.borrow().right, p, q);
                if right_p {
                    return (true, true, Some(node.clone()));
                }
                return (false, true, None);
            }
            let (left_p, left_q, left_ancestor) = Solution::find(&node.borrow().left, p, q);
            if left_ancestor.is_some() {
                return (true, true, left_ancestor);
            }
            let (right_p, right_q, right_ancestor) = Solution::find(&node.borrow().right, p, q);
            if right_ancestor.is_some() {
                return (true, true, right_ancestor);
            }
            if (left_p && right_q) || (right_p && left_q) {
                return (true, true, Some(node.clone()));
            } else {
                if left_p || right_p {
                    return (true, false, None);
                } else if left_q || right_q {
                    return (false, true, None);
                } else {
                    return (false, false, None);
                }
            }
        }
        return (false, false, None);
    }
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let (_, _, node) = Solution::find(&root, p.unwrap().borrow().val, q.unwrap().borrow().val);
        return node;
    }
}
fn main() {
    println!("Hello, world!");
}
