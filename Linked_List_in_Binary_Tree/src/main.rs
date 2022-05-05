struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}
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
    fn matching(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(list_node) = head {
            if let Some(tree_node) = root {
                if list_node.val == tree_node.borrow().val {
                    if Solution::matching(&list_node.next, &tree_node.borrow().left) {
                        return true;
                    }
                    if Solution::matching(&list_node.next, &tree_node.borrow().right) {
                        return true;
                    }
                    return false;
                }
                return false;
            }
            return false;
        }
        return true;
    }

    fn search(head: &Option<Box<ListNode>>, root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(tree_node) = root {
            if Solution::matching(head, &Some(tree_node.clone())) {
                return true;
            }
            if Solution::search(head, &tree_node.borrow().left) {
                return true;
            }
            if Solution::search(head, &tree_node.borrow().right) {
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::search(&head, &root)
    }
}

// [1,4,2,6,8]
// [1,4,4,null,2,2,null,1,null,6,8,null,null,null,null,1,3]

fn main() {}
