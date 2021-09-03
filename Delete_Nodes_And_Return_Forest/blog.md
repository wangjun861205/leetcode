Given the root of a binary tree, each node in the tree has a distinct value.

After deleting all nodes with a value in to_delete, we are left with a forest (a disjoint union of trees).

Return the roots of the trees in the remaining forest. You may return the result in any order.

Example 1:

![](https://assets.leetcode.com/uploads/2019/07/01/screen-shot-2019-07-01-at-53836-pm.png)

> Input: root = [1,2,3,4,5,6,7], to_delete = [3,5]  
> Output: [[1,2,null,4],[6],[7]]

Example 2:

> Input: root = [1,2,4,null,3], to_delete = [3]  
> Output: [[1,2,4]]

Constraints:

- The number of nodes in the given tree is at most 1000.
- Each node has a distinct value between 1 and 1000.
- to_delete.length <= 1000
- to_delete contains distinct values between 1 and 1000.

---

递归的时候向下告知自己这个节点是不是被删掉了， 如果删掉了，那下层的 left 和 right 就是新的 root 节点了

代码写的有点啰嗦

---

代码实现(Rust):

```rust
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
```
