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
    fn rc(root: &Option<Rc<RefCell<TreeNode>>>, val: i32, depth: i32, cur_depth: i32) {
        if let Some(root) = root {
            if cur_depth == depth - 1 {
                let mut new_left = TreeNode::new(val);
                let mut new_right = TreeNode::new(val);
                let ori_left = root.borrow_mut().left.take();
                let ori_right = root.borrow_mut().right.take();
                new_left.left = ori_left;
                new_right.right = ori_right;
                root.borrow_mut().left = Some(Rc::new(RefCell::new(new_left)));
                root.borrow_mut().right = Some(Rc::new(RefCell::new(new_right)));
                return;
            } else {
                Solution::rc(&root.borrow().left, val, depth, cur_depth + 1);
                Solution::rc(&root.borrow().right, val, depth, cur_depth + 1);
            }
        } else {
            return;
        }
    }
    pub fn add_one_row(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
        depth: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if depth == 1 {
            let mut new_root = TreeNode::new(val);
            new_root.left = root;
            return Some(Rc::new(RefCell::new(new_root)));
        } else {
            Solution::rc(&root, val, depth, 1);
            return root;
        }
    }
}
fn main() {
    println!("Hello, world!");
}
