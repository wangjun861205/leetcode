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
    fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, list: &mut Vec<i32>) {
        if let Some(node) = root {
            Solution::inorder_traversal(&node.borrow().left, list);
            list.push(node.borrow().val);
            Solution::inorder_traversal(&node.borrow().right, list);
        }
    }
    pub fn get_all_elements(
        root1: Option<Rc<RefCell<TreeNode>>>,
        root2: Option<Rc<RefCell<TreeNode>>>,
    ) -> Vec<i32> {
        let mut list1 = Vec::new();
        let mut list2 = Vec::new();
        Solution::inorder_traversal(&root1, &mut list1);
        Solution::inorder_traversal(&root2, &mut list2);
        let mut ans = Vec::new();
        while !list1.is_empty() || !list2.is_empty() {
            if list1.is_empty() {
                ans.push(list2.remove(0));
                continue;
            }
            if list2.is_empty() {
                ans.push(list1.remove(0));
                continue;
            }
            if list1[0] < list2[0] {
                ans.push(list1.remove(0));
            } else {
                ans.push(list2.remove(0));
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
