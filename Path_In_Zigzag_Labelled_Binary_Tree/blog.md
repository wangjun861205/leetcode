In an infinite binary tree where every node has two children, the nodes are labelled in row order.

In the odd numbered rows (ie., the first, third, fifth,...), the labelling is left to right, while in the even numbered rows (second, fourth, sixth,...), the labelling is right to left.

Given the label of a node in this tree, return the labels in the path from the root of the tree to the node with that label.

Example 1:

> Input: label = 14  
> Output: [1,3,4,14]

Example 2:

> Input: label = 26  
> Output: [1,2,6,10,26]

Constraints:

- 1 <= label <= 10^6

---

明天要踏上新的征程了，可能不会有这么多时间来做题写博客了，尽可能做到每天一道题吧。

这题我不知道大家怎么看，反正我觉得这就是个纯数学问题。根据例题推导出几个公式就算大功告成了。

1. 计算理论父节点的公式:  
   1.1 如果 child % 2 == 1, 则 parent = (child - 1) / 2.  
   1.2 如果 child % 2 == 0, 则 parent = child / 2.

2. 计算层级值区间的公式(假设层级为 level):  
   min = 2^level, max = 2^(level+1) - 1, 此区间为闭合区间。

3. 从理论父节点映射到实际父节点的公式(假设理论父节点为 tp, 实际父节点为 pp):  
   pp = min + (max - tp)

把这三个公式串成一个流程，这问题就解决了:

1. 计算理论父节点
2. 判断理论父节点所处的区间
3. 根据区间的 min 和 max 计算实际父节点， 然后以实际父节点作为参数，重复如上过程，直到父节点为 1，也就是根节点为止

---

代码实现(Rust):

```rust
impl Solution {
    pub fn path_in_zig_zag_tree(label: i32) -> Vec<i32> {
        if label == 1 {
            return vec![1];
        }
        let parent;
        if label % 2 == 0 {
            parent = label / 2;
        } else {
            parent = (label - 1) / 2;
        }
        let mut exp = 0;
        while 2_i32.pow(exp) > parent || 2_i32.pow(exp + 1) - 1 < parent {
            exp += 1;
        }
        let start = 2_i32.pow(exp);
        let end = 2_i32.pow(exp + 1) - 1;
        let true_parent = start + (end - parent);
        let mut path = Solution::path_in_zig_zag_tree(true_parent);
        path.push(label);
        path
    }
}
```
