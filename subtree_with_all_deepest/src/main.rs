use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

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

impl Solution {
    pub fn max_deepth(root: &Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if root.is_none() {
            return 0;
        }
        let mut left_deepth = 0;
        let mut right_deepth = 0;
        if root.as_ref().unwrap().borrow().left.is_some() {
            left_deepth = Solution::max_deepth(&root.as_ref().unwrap().borrow().left);
        }
        if root.as_ref().unwrap().borrow().right.is_some() {
            right_deepth = Solution::max_deepth(&root.as_ref().unwrap().borrow().right);
        }

        left_deepth.max(right_deepth) + 1
    }
    pub fn subtree_with_all_deepest(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() {
            return root;
        }
        let left_deepth = Solution::max_deepth(&root.as_ref().unwrap().borrow().left);
        let right_deepth = Solution::max_deepth(&root.as_ref().unwrap().borrow().right);
        if left_deepth == right_deepth {
            return root;
        } else {
            if left_deepth > right_deepth {
                return Solution::subtree_with_all_deepest(
                    root.as_ref().unwrap().borrow().left.clone(),
                );
            } else {
                return Solution::subtree_with_all_deepest(
                    root.as_ref().unwrap().borrow().right.clone(),
                );
            }
        }
    }
}
fn main() {
    let mut root = TreeNode::new(3);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let left = &mut root.left;
    left.as_mut().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    left.as_mut().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    let point = Rc::new(RefCell::new(root));
    println!("{}", Solution::max_deepth(&Some(point)));
    // println!("{:?}", root);
}
