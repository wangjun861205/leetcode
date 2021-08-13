Given the root of a binary search tree, return a balanced binary search tree with the same node values. If there is more than one answer, return any of them.

A binary search tree is balanced if the depth of the two subtrees of every node never differs by more than 1.

Example 1:

![](https://assets.leetcode.com/uploads/2021/08/10/balance1-tree.jpg)

> Input: root = [1,null,2,null,3,null,4,null,null]  
> Output: [2,1,3,null,null,null,4]  
> Explanation:  
> This is not the only correct answer, [3,1,4,null,2] is also correct.

Example 2:

![](https://assets.leetcode.com/uploads/2021/08/10/balanced2-tree.jpg)

> Input: root = [2,1,3]  
> Output: [2,1,3]

Constraints:

- The number of nodes in the tree is in the range [1, 104].
- 1 <= Node.val <= 105

---

忘了谁说过，如果笨办法有效，那这个办法就不笨。写代码得时刻把这条记心里，往往笨办法的尝试成本也是最低的，先试试，笨办法不能满足需要再想其他高招。

这题要是上来就往旋转调整上想估计一时半会儿是出不来了，我试了几次，对于我的能力这个方法太复杂了，于是找答案，没想到答案出奇的简单:

1. 先用 inorder traversal 把节点的值都放到一个数组里，因为这是个 BST, 而我们又用的 inorder 的顺序，所以我们得到的数组一定是有序的。
2. 然后用前面得到的数组组建 balanced BST， 具体方法就是每次找中间元素， 以中间元素把数组分成左右两部分，再分别以左右两部分为参数递归调用，得到的结果分别作为中间元素的 left 和 right, 这样组建的 BST 不仅是 balanced 的, 而且是高度最低的。

---

代码实现(Rust):

```rust

impl Solution {
    fn inorder_traversal(root: &Option<Rc<RefCell<TreeNode>>>, vals: &mut Vec<i32>) {
        if let Some(node) = root {
            Solution::inorder_traversal(&node.borrow().left, vals);
            vals.push(node.borrow().val);
            Solution::inorder_traversal(&node.borrow().right, vals);
        }
    }

    fn construct(vals: &Vec<i32>, start: usize, end: usize) -> Option<Rc<RefCell<TreeNode>>> {
        if start == end {
            return None;
        }
        let mid = start + (end - start) / 2;
        let mut root = TreeNode::new(vals[mid]);
        let left = Solution::construct(vals, start, mid);
        let right = Solution::construct(vals, mid + 1, end);
        root.left = left;
        root.right = right;
        Some(Rc::new(RefCell::new(root)))
    }

    pub fn balance_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        let mut vals = Vec::new();
        Solution::inorder_traversal(&root, &mut vals);
        Solution::construct(&vals, 0, vals.len())
    }
}

```
