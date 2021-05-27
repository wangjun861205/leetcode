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
use std::collections::HashMap;
use std::rc::Rc;
impl Solution {
    fn rc(
        in_map: &HashMap<i32, usize>,
        left_index: usize,
        right_index: usize,
        postorder: &Vec<i32>,
        post_index: &mut usize,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let val = postorder[*post_index];
        let mut node = TreeNode::new(val);
        *post_index -= 1;
        let in_index = in_map.get(&val).unwrap();
        let right = if in_index < &right_index {
            Solution::rc(in_map, *in_index + 1, right_index, postorder, post_index)
        } else {
            None
        };
        let left = if in_index > &left_index {
            Solution::rc(in_map, left_index, *in_index - 1, postorder, post_index)
        } else {
            None
        };
        node.left = left;
        node.right = right;
        Some(Rc::new(RefCell::new(node)))
    }
    pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        let in_map = inorder
            .into_iter()
            .enumerate()
            .map(|(i, v)| (v, i))
            .collect();
        let mut post_index = postorder.len() - 1;
        Solution::rc(&in_map, 0, postorder.len() - 1, &postorder, &mut post_index)
    }
}

fn main() {
    println!("Hello, world!");
}
