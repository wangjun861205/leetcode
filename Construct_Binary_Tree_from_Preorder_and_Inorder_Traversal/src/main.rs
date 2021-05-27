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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn rc(
        preorder: &Vec<i32>,
        preorder_index: &mut usize,
        inorder: &Vec<i32>,
        inorder_left_index: usize,
        inorder_right_index: usize,
        inorder_map: &HashMap<i32, usize>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if inorder_left_index > inorder_right_index {
            return None;
        }
        let root_val = preorder.get(*preorder_index).unwrap();
        let mut node = TreeNode::new(*root_val);
        let inorder_index = inorder_map.get(root_val).unwrap();
        *preorder_index += 1;
        let left = if inorder_index > &0 {
            Solution::rc(preorder, preorder_index, inorder, inorder_left_index, inorder_index - 1, inorder_map)
        } else {
            None
        };
        let right = if inorder_index < &(inorder.len() - 1) {
            Solution::rc(preorder, preorder_index, inorder, inorder_index + 1, inorder_right_index, inorder_map)
        } else {
            None
        };
        node.left = left;
        node.right = right;
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let inorder_map: HashMap<i32, usize> = inorder.iter().enumerate().map(|(i, v)| (*v, i)).collect();
        let mut preorder_index = 0_usize;
        Solution::rc(&preorder, &mut preorder_index, &inorder, 0, inorder.len() - 1, &inorder_map)
    }
}
fn main() {
    println!("{:?}", Solution::build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]));
}
