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

use std::collections::HashMap;
impl Solution {
    fn find(root: &Option<Rc<RefCell<TreeNode>>>, counts: &mut HashMap<Vec<i32>, i32>, ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut left = Solution::find(&node.borrow().left, counts, ans);
            let right = Solution::find(&node.borrow().right, counts, ans);
            left.extend(right);
            left.push(node.borrow().val);
            *counts.entry(left.clone()).or_insert(0) += 1;
            if counts.get(&left).unwrap() == &2 {
                ans.push(Some(node.clone()));
            }
            return left;
        }
        vec![-201]
    }

    pub fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut counts = HashMap::new();
        let mut ans = Vec::new();
        Solution::find(&root, &mut counts, &mut ans);
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
