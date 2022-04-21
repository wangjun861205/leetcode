Given the root of a binary search tree and the lowest and highest boundaries as low and high, trim the tree so that all its elements lies in [low, high]. Trimming the tree should not change the relative structure of the elements that will remain in the tree (i.e., any node's descendant should remain a descendant). It can be proven that there is a unique answer.

Return the root of the trimmed binary search tree. Note that the root may change depending on the given bounds.

Example 1:

![](https://assets.leetcode.com/uploads/2020/09/09/trim1.jpg)

> Input: root = [1,0,2], low = 1, high = 2  
> Output: [1,null,2]

Example 2:

![](https://assets.leetcode.com/uploads/2020/09/09/trim2.jpg)

> Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3  
> Output: [3,2,null,1]

Constraints:

- The number of nodes in the tree in the range [1, 104].
- 0 <= Node.val <= 104
- The value of each node in the tree is unique.
- root is guaranteed to be a valid binary search tree.
- 0 <= low <= high <= 104

---

二叉搜索树的特性是， 左边所有的子节点都小于当前节点， 右边所有子节点都大于当前节点。如果`current_value < low`则左侧所有子节点的值肯定也都`< low`, 这时候只需要返回右侧子节点。如果`current_value > high`则右侧所有子节点的值肯定也都`> high`, 这时只需要返回左侧子节点。如果`low <= current_value <= high`则递归调用处理左右两侧子节点

---

```rust
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn trim_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        low: i32,
        high: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            if node.borrow().val < low {
                return Solution::trim_bst(node.borrow_mut().right.take(), low, high);
            }
            if node.borrow().val > high {
                return Solution::trim_bst(node.borrow_mut().left.take(), low, high);
            }
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            node.borrow_mut().left = Solution::trim_bst(left, low, high);
            node.borrow_mut().right = Solution::trim_bst(right, low, high);
            return Some(node);
        }
        None
    }
}
```
