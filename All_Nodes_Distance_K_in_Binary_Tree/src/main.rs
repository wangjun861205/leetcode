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

struct Solution;

use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

impl Solution {
    fn connect(
        parent: Option<Rc<RefCell<TreeNode>>>,
        child: Option<Rc<RefCell<TreeNode>>>,
        m: &mut HashMap<i32, Vec<i32>>,
    ) {
        if let Some(p) = parent {
            if let Some(c) = child {
                m.entry((*p).borrow().val)
                    .or_insert(Vec::new())
                    .push((*c).borrow().val);
                m.entry((*c).borrow().val)
                    .or_insert(Vec::new())
                    .push((*p).borrow().val);
                Solution::connect(Some(c.clone()), (*c).borrow().left.clone(), m);
                Solution::connect(Some(c.clone()), (*c).borrow().right.clone(), m);
            }
        } else {
            if let Some(c) = child {
                Solution::connect(Some(c.clone()), (*c).borrow().left.clone(), m);
                Solution::connect(Some(c.clone()), (*c).borrow().right.clone(), m);
            }
        }
    }
    pub fn distance_k(
        root: Option<Rc<RefCell<TreeNode>>>,
        target: Option<Rc<RefCell<TreeNode>>>,
        k: i32,
    ) -> Vec<i32> {
        let mut m = HashMap::new();
        Solution::connect(None, root, &mut m);
        let target_value = (*target.unwrap()).borrow().val;
        let mut curr = vec![target_value];
        let mut visited = HashSet::new();
        visited.insert(target_value);
        for _ in 0..k {
            let mut next = Vec::new();
            for v in &curr {
                if let Some(l) = m.get(v) {
                    for n in l {
                        if !visited.contains(n) {
                            visited.insert(*n);
                            next.push(*n);
                        }
                    }
                }
            }
            curr = next;
        }
        curr
    }
}
fn main() {
    println!("Hello, world!");
}
