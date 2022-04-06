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
    fn clone(root: &Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let mut copy = TreeNode::new(node.borrow().val);
            copy.left = Solution::clone(&node.borrow().left);
            copy.right = Solution::clone(&node.borrow().right);
            return Some(Rc::new(RefCell::new(copy)));
        }
        None
    }
    pub fn all_possible_fbt(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if n % 2 == 0 {
            return vec![];
        }
        let mut m: HashMap<i32, Vec<Option<Rc<RefCell<TreeNode>>>>> =
            vec![(1, vec![Some(Rc::new(RefCell::new(TreeNode::new(0))))])]
                .into_iter()
                .collect();
        for i in (3..21).step_by(2) {
            if i > n {
                break;
            }

            let mut nodes = Vec::new();
            for l in (1..i).step_by(2) {
                let lefts = m.get(&l).unwrap().clone();
                let rights = m.get(&(i - 1 - l)).unwrap().clone();
                for left in &lefts {
                    for right in &rights {
                        nodes.push(Some(Rc::new(RefCell::new(TreeNode {
                            val: 0,
                            left: Solution::clone(left),
                            right: Solution::clone(right),
                        }))));
                    }
                }
            }
            m.insert(i, nodes);
        }
        m.get(&n).unwrap().clone()
    }
}

fn main() {
    println!("Hello, world!");
}
