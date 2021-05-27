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
    pub fn construct(pre: &mut Vec<i32>, post: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if pre.len() == 0 {
            return None;
        }
        let root_val = pre.remove(0);
        let mut root = TreeNode::new(root_val);
        let pos = post.iter().position(|v| v == &root_val);
        match pos {
            None => {
                return Some(Rc::new(RefCell::new(root)));
            }
            Some(p) => {
                let mut l: Vec<i32> = post.drain(..p).collect();
                post.remove(0);
                match l.len() {
                    0 => {
                        return Some(Rc::new(RefCell::new(root)));
                    }
                    1 => {
                        pre.remove(0);
                        root.left = Some(Rc::new(RefCell::new(TreeNode::new(l[0]))));
                        return Some(Rc::new(RefCell::new(root)));
                    }
                    // 2 => {
                    //     pre.remove(0);
                    //     pre.remove(0);
                    //     root.left = Some(Rc::new(RefCell::new(TreeNode::new(l[0]))));
                    //     root.right = Some(Rc::new(RefCell::new(TreeNode::new(l[1]))));
                    //     return Some(Rc::new(RefCell::new(root)));
                    // }
                    _ => {
                        root.left = Solution::construct(pre, &mut l);
                        root.right = Solution::construct(pre, post);
                        return Some(Rc::new(RefCell::new(root)));
                    }
                }
            }
        }
    }
    pub fn construct_from_pre_post(pre: Vec<i32>, post: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut pre = pre.clone();
        let mut post = post.clone();
        let mut root = TreeNode::new(pre.remove(0));
        post.pop();
        root.left = Solution::construct(&mut pre, &mut post);
        root.right = Solution::construct(&mut pre, &mut post);
        Some(Rc::new(RefCell::new(root)))
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::construct_from_pre_post(vec![4, 2, 1, 3], vec![3, 1, 2, 4])
    );
}
