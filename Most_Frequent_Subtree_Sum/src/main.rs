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
use std::cmp::Reverse;
use std::rc::Rc;

use std::collections::HashMap;
impl Solution {
    fn dfs(root: &Option<Rc<RefCell<TreeNode>>>, cache: &mut HashMap<i32, i32>) -> i32 {
        if let Some(n) = root {
            let left = Solution::dfs(&n.borrow().left, cache);
            let right = Solution::dfs(&n.borrow().right, cache);
            let v = left + right + n.borrow().val;
            *cache.entry(v).or_insert(0) += 1;
            return v;
        }
        0
    }
    pub fn find_frequent_tree_sum(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut cache = HashMap::new();
        Solution::dfs(&root, &mut cache);
        let mut l: Vec<(i32, i32)> = cache.into_iter().collect();
        l.sort_by_key(|v| Reverse(v.1));
        let freq = l[0].1;
        l.into_iter()
            .take_while(|v| v.1 == freq)
            .map(|v| v.0)
            .collect()
    }
}
fn main() {
    println!("Hello, world!");
}
