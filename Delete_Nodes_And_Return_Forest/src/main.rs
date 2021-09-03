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

use std::collections::HashSet;

impl Solution {
    fn del(root: Option<Rc<RefCell<TreeNode>>>, to_delete: &HashSet<i32>, is_root: bool, ans: &mut Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let is_deleted = to_delete.contains(&node.borrow().val);
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            if is_root {
                if is_deleted {
                    let l = Solution::del(left, to_delete, true, ans);
                    let r = Solution::del(right, to_delete, true, ans);
                    if l.is_some() {
                        ans.push(l);
                    }
                    if r.is_some() {
                        ans.push(r);
                    }
                    return None;
                } else {
                    let l = Solution::del(left, to_delete, false, ans);
                    let r = Solution::del(right, to_delete, false, ans);
                    node.borrow_mut().left = l;
                    node.borrow_mut().right = r;
                    ans.push(Some(node));
                    return None;
                }
            } else {
                if is_deleted {
                    let l = Solution::del(left, to_delete, true, ans);
                    let r = Solution::del(right, to_delete, true, ans);
                    if l.is_some() {
                        ans.push(l);
                    }
                    if r.is_some() {
                        ans.push(r);
                    }
                    return None;
                } else {
                    let l = Solution::del(left, to_delete, false, ans);
                    let r = Solution::del(right, to_delete, false, ans);
                    node.borrow_mut().left = l;
                    node.borrow_mut().right = r;
                    return Some(node);
                }
            }
        }
        None
    }
    pub fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut ans = Vec::new();
        let to_delete: HashSet<i32> = to_delete.into_iter().collect();
        Solution::del(root, &to_delete, true, &mut ans);
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
