struct Solution;

use std::collections::HashMap;

impl Solution {
    fn bfs(
        graph: &Vec<Vec<i32>>,
        idx: i32,
        cache: &mut HashMap<i32, Vec<Vec<i32>>>,
    ) -> Vec<Vec<i32>> {
        if idx == graph.len() as i32 - 1 {
            return vec![vec![graph.len() as i32 - 1]];
        }
        let mut ans = Vec::new();
        for next in &graph[idx as usize] {
            let mut paths = if let Some(c) = cache.get(next) {
                c.clone()
            } else {
                Solution::bfs(graph, *next, cache)
            };
            for p in &mut paths {
                p.insert(0, idx);
            }
            ans.append(&mut paths);
        }
        cache.insert(idx, ans.clone());
        ans
    }
    pub fn all_paths_source_target(graph: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        Solution::bfs(&graph, 0, &mut HashMap::new())
    }
}

fn main() {
    println!("Hello, world!");
}
