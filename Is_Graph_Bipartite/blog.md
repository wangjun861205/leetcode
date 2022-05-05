There is an undirected graph with n nodes, where each node is numbered between 0 and n - 1. You are given a 2D array graph, where graph[u] is an array of nodes that node u is adjacent to. More formally, for each v in graph[u], there is an undirected edge between node u and node v. The graph has the following properties:

There are no self-edges (graph[u] does not contain u).
There are no parallel edges (graph[u] does not contain duplicate values).
If v is in graph[u], then u is in graph[v] (the graph is undirected).
The graph may not be connected, meaning there may be two nodes u and v such that there is no path between them.
A graph is bipartite if the nodes can be partitioned into two independent sets A and B such that every edge in the graph connects a node in set A and a node in set B.

Return true if and only if it is bipartite.

Example 1:

![](https://assets.leetcode.com/uploads/2020/10/21/bi2.jpg)

> Input: graph = [[1,2,3],[0,2],[0,1,3],[0,2]]  
> Output: false

Explanation: There is no way to partition the nodes into two independent sets such that every edge connects a node in one and a node in the other.

Example 2:

![](https://assets.leetcode.com/uploads/2020/10/21/bi1.jpg)

> Input: graph = [[1,3],[0,2],[1,3],[0,2]]  
> Output: true

Explanation: We can partition the nodes into two sets: {0, 2} and {1, 3}.

Constraints:

- graph.length == n
- 1 <= n <= 100
- 0 <= graph[u].length < n
- 0 <= graph[u][i] <= n - 1
- graph[u] does not contain u.
- All the values of graph[u] are unique.
- If graph[u] contains v, then graph[v] contains u.

---

BFS 对 node 进行染色， 每个 edge 两端的颜色必须不同， 如果两端是相同颜色的， 则证明整个图无法进行二分。要注意的是 graph 里面可能会有不相连的 node， 所以整体还需要进行遍历查找是否还有未被染色的 node

---

```rust
impl Solution {
    fn bfs(graph: &Vec<Vec<i32>>, group: &mut Vec<i32>, i: usize) -> bool {
        let mut color = 1;
        group[i] = color;
        let mut stack = vec![i];
        loop {
            let mut next = Vec::new();
            while let Some(curr) = stack.pop() {
                for &n in &graph[curr] {
                    if group[n as usize] == -color {
                        continue;
                    }
                    if group[n as usize] == color {
                        return false;
                    }
                    group[n as usize] = -color;
                    next.push(n as usize)
                }
            }
            color = -color;
            if next.is_empty() {
                break;
            }
            stack = next;
        }
        true
    }
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        let mut group = vec![0; graph.len()];
        loop {
            let mut updated = false;
            for i in 0..group.len() {
                if group[i] == 0 {
                    if !Solution::bfs(&graph, &mut group, i) {
                        return false;
                    }
                    updated = true;
                }
            }
            if !updated {
                break;
            }
        }
        true
    }
}
```
