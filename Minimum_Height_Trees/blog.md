A tree is an undirected graph in which any two vertices are connected by exactly one path. In other words, any connected graph without simple cycles is a tree.

Given a tree of n nodes labelled from 0 to n - 1, and an array of n - 1 edges where edges[i] = [ai, bi] indicates that there is an undirected edge between the two nodes ai and bi in the tree, you can choose any node of the tree as the root. When you select a node x as the root, the result tree has height h. Among all possible rooted trees, those with minimum height (i.e. min(h)) are called minimum height trees (MHTs).

Return a list of all MHTs' root labels. You can return the answer in any order.

The height of a rooted tree is the number of edges on the longest downward path between the root and a leaf.

Example 1:

![](https://assets.leetcode.com/uploads/2020/09/01/e1.jpg)

> Input: n = 4, edges = [[1,0],[1,2],[1,3]]  
> Output: [1]

Explanation: As shown, the height of the tree is 1 when the root is the node with label 1 which is the only MHT.

Example 2:

![](https://assets.leetcode.com/uploads/2020/09/01/e2.jpg)

> Input: n = 6, edges = [[3,0],[3,1],[3,2],[3,4],[5,4]]  
> Output: [3,4]

Example 3:

> Input: n = 1, edges = []  
> Output: [0]

Example 4:

> Input: n = 2, edges = [[0,1]]  
> Output: [0,1]

Constraints:

- 1 <= n <= 2 \* 104
- edges.length == n - 1
- 0 <= ai, bi < n
- ai != bi
- All the pairs (ai, bi) are distinct.
- The given input is guaranteed to be a tree and there will be no repeated edges.

---

这题我就不再赘述了，网站上有官方的 Solution。讲的很清楚，而且 Topological Sorting 确实很精妙。代码实现方面我没参考 Solution 里面的例子，了解了原理之后自己写的，运行的效率不怎么高。

---

```rust
use std::collections::HashSet;

impl Solution {
    pub fn find_min_height_trees(n: i32, mut edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut counts = vec![0; n as usize];
        let mut total = 0;
        for e in &edges {
            counts[e[0] as usize] += 1;
            counts[e[1] as usize] += 1;
            total += 2;
        }
        loop {
            let set: HashSet<i32> = counts
                .iter()
                .enumerate()
                .filter_map(|(i, v)| if *v == 1 { Some(i as i32) } else { None })
                .collect();
            if set.len() * 2 == total {
                let idx = counts
                    .iter()
                    .enumerate()
                    .max_by_key(|(_, &v)| v)
                    .map(|(i, _)| i)
                    .unwrap();
                return vec![idx as i32];
            }
            if total == 2 {
                return set.into_iter().collect();
            }
            let (cur, remain): (Vec<Vec<i32>>, Vec<Vec<i32>>) = edges
                .into_iter()
                .partition(|v| set.contains(&v[0]) || set.contains(&v[1]));
            edges = remain;
            for e in cur {
                counts[e[0] as usize] -= 1;
                counts[e[1] as usize] -= 1;
                total -= 2;
            }
        }
    }
}
```
