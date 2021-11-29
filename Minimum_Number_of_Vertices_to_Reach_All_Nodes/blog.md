Given a directed acyclic graph, with n vertices numbered from 0 to n-1, and an array edges where edges[i] = [fromi, toi] represents a directed edge from node fromi to node toi.

Find the smallest set of vertices from which all nodes in the graph are reachable. It's guaranteed that a unique solution exists.

Notice that you can return the vertices in any order.

Example 1:

![](https://assets.leetcode.com/uploads/2020/07/07/untitled22.png)

> Input: n = 6, edges = [[0,1],[0,2],[2,5],[3,4],[4,2]]  
> Output: [0,3]

Explanation: It's not possible to reach all the nodes from a single vertex. From 0 we can reach [0,1,2,5]. From 3 we can reach [3,4,2,5]. So we output [0,3].

Example 2:

![](https://assets.leetcode.com/uploads/2020/07/07/untitled.png)

> Input: n = 5, edges = [[0,1],[2,1],[3,1],[1,4],[2,4]]  
> Output: [0,2,3]

Explanation: Notice that vertices 0, 3 and 2 are not reachable from any other node, so we must include them. Also any of these vertices can reach nodes 1 and 4.

Constraints:

- 2 <= n <= 10^5
- 1 <= edges.length <= min(10^5, n \* (n - 1) / 2)
- edges[i].length == 2
- 0 <= fromi, toi < n
- All pairs (fromi, toi) are distinct.

---

union find, 注意有多个 from 指向同一个 to 的时候， to 的 parent 为遍历的第一个 from

---

```rust
impl Solution {
    fn find(parents: &mut Vec<usize>, node: usize) -> usize {
        if parents[node] == node {
            return node;
        }
        let parent = Solution::find(parents, parents[node]);
        parents[node] = parent;
        return parent;
    }
    fn rc(parents: &mut Vec<usize>, from: usize, to: usize) {
        let parent = Solution::find(parents, from);
        if parents[to] == to {
            parents[to] = parent;
        }
    }
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut parents: Vec<usize> = (0..n as usize).into_iter().collect();
        for e in edges {
            Solution::rc(&mut parents, e[0] as usize, e[1] as usize);
        }
        for i in 0..n as usize {
            Solution::find(&mut parents, i);
        }
        parents
            .into_iter()
            .enumerate()
            .filter(|(i, v)| i == v)
            .map(|(i, v)| v as i32)
            .collect()
    }
}
```
