struct Solution;

use std::collections::HashSet;

impl Solution {
    fn dfs(id: &mut i32, graph: &Vec<Vec<i32>>, ids: &mut Vec<i32>, low: &mut Vec<i32>, cur: i32, parent: i32, bridges: &mut Vec<Vec<i32>>) {
        ids[cur as usize] = *id;
        low[cur as usize] = *id;
        *id += 1;
        for n in &graph[cur as usize] {
            if n == &parent {
                continue;
            }
            if ids[*n as usize] == -1 {
                Solution::dfs(id, graph, ids, low, *n, cur, bridges);
                low[cur as usize] = low[cur as usize].min(low[*n as usize]);
                if ids[cur as usize] < low[*n as usize] {
                    bridges.push(vec![cur, *n]);
                }
            } else {
                low[cur as usize] = low[cur as usize].min(ids[*n as usize]);
            }
        }
    }
    pub fn critical_connections(n: i32, connections: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut graph = vec![Vec::new(); n as usize];
        for conn in connections {
            graph[conn[0] as usize].push(conn[1]);
            graph[conn[1] as usize].push(conn[0]);
        }
        let mut id = 0;
        let mut ids = vec![-1; n as usize];
        let mut low = vec![-1; n as usize];
        let mut bridges = Vec::new();
        for i in 0..graph.len() {
            Solution::dfs(&mut id, &graph, &mut ids, &mut low, i as i32, -1, &mut bridges);
        }
        bridges
    }
}
fn main() {
    println!("{:?}", Solution::critical_connections(4, vec![vec![0, 1], vec![1, 2], vec![2, 0], vec![1, 3]]));
}

// 4->0->1->4!
//    |
//    |->2->4!
//    |
//    |->3
