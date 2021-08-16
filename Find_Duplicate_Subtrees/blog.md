Given the root of a binary tree, return all duplicate subtrees.

For each kind of duplicate subtrees, you only need to return the root node of any one of them.

Two trees are duplicate if they have the same structure with the same node values.

Example 1:

![](https://assets.leetcode.com/uploads/2020/08/16/e1.jpg)  
Input: root = [1,2,3,4,null,2,4,null,null,4]  
Output: [[2,4],[4]]

Example 2:

![](https://assets.leetcode.com/uploads/2020/08/16/e2.jpg)

Input: root = [2,1,1]  
Output: [[1]]

Example 3:

![](https://assets.leetcode.com/uploads/2020/08/16/e33.jpg)

Input: root = [2,2,2,3,null,3,null]  
Output: [[2,3],[3]]

Constraints:

- The number of the nodes in the tree will be in the range [1, 10^4]
- -200 <= Node.val <= 200

---

traversal 整个 tree, 同时记录已经出现的 subtree, 如果当前的 subtree 已经出现过且出现次数为**_1 次_**, 则把当前的这个 subtree 放到答案数组中。

严格来说这题对于不同语言可能复杂程度会有所差异，因为用数组或者切片直接作为 map 的 key, 不是所有语言都直接支持。

---

代码实现(Rust):

```rust
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
```
