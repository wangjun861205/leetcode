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
    fn rc(l: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        if l.len() == 0 {
            return vec![None];
        }
        let mut ans = Vec::new();
        for i in 0..l.len() {
            let cur = l[i];
            let left = l[..i].to_vec();
            let right = l[i + 1..].to_vec();
            let l = Solution::rc(left);
            let r = Solution::rc(right);
            for lv in &l {
                for rv in &r {
                    let mut root = TreeNode::new(cur);
                    root.left = lv.clone();
                    root.right = rv.clone();
                    ans.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }
        ans
    }
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let l: Vec<i32> = (1..=n).into_iter().collect();
        Solution::rc(l)
    }
}

fn main() {
    println!("{:?}", Solution::generate_trees(3));
}
