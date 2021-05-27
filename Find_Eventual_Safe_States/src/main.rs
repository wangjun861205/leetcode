struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn is_terminal(
        graph: &Vec<Vec<usize>>,
        index: usize,
        visited: &mut HashSet<usize>,
        results: &mut HashMap<usize, bool>,
    ) -> bool {
        if visited.contains(&index) {
            return false;
        }
        visited.insert(index);
        let rs: Vec<bool> = graph[index]
            .iter()
            .map(|v| {
                if let Some(c) = results.get(v) {
                    *c
                } else {
                    Solution::is_terminal(graph, *v, visited, results)
                }
            })
            .collect();
        let result = rs.len() == 0 || rs.into_iter().all(|r| r);
        results.insert(index, result);
        result
    }
    pub fn eventual_safe_nodes(graph: Vec<Vec<i32>>) -> Vec<i32> {
        let graph: Vec<Vec<usize>> = graph
            .into_iter()
            .map(|v| v.into_iter().map(|vv| vv as usize).collect())
            .collect();
        let mut results = HashMap::new();
        for i in 0..graph.len() {
            let mut visited = HashSet::new();
            Solution::is_terminal(&graph, i, &mut visited, &mut results);
        }
        let mut l: Vec<i32> = results
            .into_iter()
            .filter(|(_, v)| *v)
            .map(|(k, _)| k as i32)
            .collect();
        l.sort();
        l
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::eventual_safe_nodes(vec![
            vec![1, 2, 3, 4],
            vec![1, 2],
            vec![3, 4],
            vec![0, 4],
            vec![]
        ])
    );
}
