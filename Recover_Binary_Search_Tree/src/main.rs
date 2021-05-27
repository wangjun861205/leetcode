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
    fn find(root: &Option<Rc<RefCell<TreeNode>>>, first: &mut Option<Rc<RefCell<TreeNode>>>, second: &mut Option<Rc<RefCell<TreeNode>>>, prev: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(root) = root {
            Solution::find(&root.borrow().left, first, second, prev);
            if let Some(prev) = prev {
                if first.is_none() && root.borrow().val < prev.borrow().val {
                    *first = Some(prev.clone());
                }
                if first.is_some() && root.borrow().val < prev.borrow().val {
                    *second = Some(root.clone());
                }
            }
            *prev = Some(root.clone());
            Solution::find(&root.borrow().right, first, second, prev);
        }
    }
    pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        let mut first = None;
        let mut second = None;
        let mut prev = None;
        Solution::find(root, &mut first, &mut second, &mut prev);
        let temp = first.as_ref().unwrap().borrow().val;
        first.as_ref().unwrap().borrow_mut().val = second.as_ref().unwrap().borrow().val;
        second.as_ref().unwrap().borrow_mut().val = temp;
    }
}
fn main() {
    let mut root = Some(Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode { val: 1, left: None, right: None }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode { val: 2, left: None, right: None }))),
            right: None,
        }))),
    })));
    Solution::recover_tree(&mut root);
}
