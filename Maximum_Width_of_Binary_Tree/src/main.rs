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

use std::collections::HashMap;

impl Solution {
    fn dfs(
        root: &Option<Rc<RefCell<TreeNode>>>,
        level: usize,
        index: i32,
        widths: &mut HashMap<usize, (i32, i32)>,
    ) {
        if let Some(root) = root {
            let entry = widths.entry(level).or_insert((index, index));
            if index < entry.0 {
                entry.0 = index;
            }
            if index > entry.1 {
                entry.1 = index;
            }
            Solution::dfs(&root.borrow().left, level + 1, index * 2 + 1, widths);
            Solution::dfs(&root.borrow().right, level + 1, index * 2 + 2, widths);
        }
    }
    pub fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut widths: HashMap<usize, (i32, i32)> = HashMap::new();
        Solution::dfs(&root, 0, 0, &mut widths);
        let (s, e) = widths.values().max_by_key(|(s, e)| e - s).unwrap();
        e - s + 1
    }
}
fn main() {
    println!("Hello, world!");
}
