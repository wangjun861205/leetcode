For a binary tree T, we can define a flip operation as follows: choose any node, and swap the left and right child subtrees.

A binary tree X is flip equivalent to a binary tree Y if and only if we can make X equal to Y after some number of flip operations.

Given the roots of two binary trees root1 and root2, return true if the two trees are flip equivelent or false otherwise.

Example 1:

![](https://assets.leetcode.com/uploads/2018/11/29/tree_ex.png)

> Input: root1 = [1,2,3,4,5,6,null,null,null,7,8], root2 = [1,3,2,null,6,4,5,null,null,null,null,8,7]  
> Output: true  
> Explanation: We flipped at nodes with values 1, 3, and 5.

Example 2:

> Input: root1 = [], root2 = []  
> Output: true

Example 3:

> Input: root1 = [], root2 = [1]  
> Output: false

Example 4:

> Input: root1 = [0,null,1], root2 = []  
> Output: false

Example 5:

> Input: root1 = [0,null,1], root2 = [0,1]  
> Output: true

Constraints:

- The number of nodes in each tree is in the range [0, 100].
- Each tree will have unique node values in the range [0, 99].

---

这题的关键是想明白，翻转相等的两个等价条件:

1. 两棵树所包含的元素集合必须一致
2. 相同元素的父节点也必须一致  
   然后就可以自由发挥了， 跟我一样标记所有元素父节点的要注意，要区分出没有父节点(根节点)和元素不存在的状态， 我的代码里-1 表示根节点，-2 表示树中不包含这个元素

---

代码实现(Rust):

```rust
impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>, parent: i32, parents: &mut Vec<i32>) {
        if let Some(node) = root {
            let left = node.borrow_mut().left.take();
            let right = node.borrow_mut().right.take();
            let val = node.borrow().val;
            parents[val as usize] = parent;
            Solution::dfs(left, val, parents);
            Solution::dfs(right, val, parents);
        }
    }
    pub fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
        let mut parents1 = vec![-1; 100];
        let mut parents2 = vec![-1; 100];
        Solution::dfs(root1, -2, &mut parents1);
        Solution::dfs(root2, -2, &mut parents2);
        for (p1, p2) in parents1.into_iter().zip(parents2) {
            if p1 != p2 {
                return false;
            }
        }
        true
    }
}
```
