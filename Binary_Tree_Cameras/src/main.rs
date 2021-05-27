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
    fn rc(root: &Option<Rc<RefCell<TreeNode>>>, index: i32, set: bool, cache: &mut HashMap<(i32, bool), i32>) -> i32 {
        let rf = root.as_ref().unwrap();
        if rf.borrow().left.is_none() && rf.borrow().right.is_none() {
            if set {
                return 1;
            } else {
                return 0;
            }
        }

        let left_set_count = if rf.borrow().left.is_some() {
            if let Some(c) = cache.get(&(index * 2 + 1, true)) {
                *c
            } else {
                Solution::rc(&rf.borrow().left, index * 2 + 1, true, cache)
            }
        } else {
            0
        };
        let left_unset_count = if rf.borrow().left.is_some() {
            if let Some(c) = cache.get(&(index * 2 + 1, false)) {
                *c
            } else {
                Solution::rc(&rf.borrow().left, index * 2 + 1, false, cache)
            }
        } else {
            0
        };
        let right_set_count = if rf.borrow().right.is_some() {
            if let Some(c) = cache.get(&(index * 2 + 2, true)) {
                *c
            } else {
                Solution::rc(&rf.borrow().right, index * 2 + 2, true, cache)
            }
        } else {
            0
        };
        let right_unset_count = if rf.borrow().right.is_some() {
            if let Some(c) = cache.get(&(index * 2 + 2, false)) {
                *c
            } else {
                Solution::rc(&rf.borrow().right, index * 2 + 2, false, cache)
            }
        } else {
            0
        };
        if rf.borrow().left.is_none() && rf.borrow().right.is_some() {
            if set {
                let ans = right_unset_count + 1;
                cache.insert((index, true), ans);
                return ans;
            } else {
                cache.insert((index, false), right_set_count);
                return right_set_count;
            }
        } else if rf.borrow().left.is_some() && rf.borrow().right.is_none() {
            if set {
                let ans = left_unset_count + 1;
                cache.insert((index, true), ans);
                return ans;
            } else {
                cache.insert((index, false), left_set_count);
                return left_set_count;
            }
        } else {
            if set {
                let ans = left_set_count.min(left_unset_count) + right_set_count.min(right_unset_count) + 1;
                cache.insert((index, true), ans);
                return ans;
            } else {
                let ans = (left_set_count + right_set_count).min(left_set_count + right_unset_count).min(left_unset_count + right_set_count);
                cache.insert((index, false), ans);
                return ans;
            }
        }
    }
    pub fn min_camera_cover(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut cache = HashMap::new();
        let set_count = Solution::rc(&root, 0, true, &mut cache);
        let unset_count = Solution::rc(&root, 0, false, &mut cache);
        if set_count == 0 {
            return unset_count;
        } else if unset_count == 0 {
            return set_count;
        }
        set_count.min(unset_count)
    }
}
fn main() {
    println!("");
}
