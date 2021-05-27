#[macro_use]
extern crate lazy_static;

use std::collections::hash_map::{Entry, HashMap};
use std::sync::Mutex;

lazy_static! {
    static ref HM: Mutex<HashMap<i32, i32>> = Mutex::new(HashMap::new());
}

fn is_all_discrete(nodes: &Vec<i32>, edges: &Vec<Vec<i32>>) -> bool {
    !nodes
        .iter()
        .any(|n| edges.iter().any(|e| &e[0] == n || &e[1] == n))
}

fn min_height(nodes: &Vec<i32>, edges: &Vec<Vec<i32>>) -> i32 {
    if nodes.len() == 0 {
        return 0;
    }
    if is_all_discrete(nodes, edges) {
        return 0;
    }
    let mut l: Vec<i32> = Vec::new();
    for i in 0..nodes.len() {
        let node = nodes[i];
        if let Entry::Occupied(n) = HM.lock().unwrap().entry(node) {
            return *n.get();
        }
        let remain_edges = edges
            .iter()
            .filter(|v| v[0] != node && v[1] != node)
            .map(|v| v.clone())
            .collect();
        let mut remain_nodes = nodes.clone();
        remain_nodes.remove(i);
        let next_height = min_height(&remain_nodes, &remain_edges);
        l.push(next_height + 1);
        HM.lock().unwrap().insert(node, next_height + 1);
    }
    *l.iter().min().unwrap()
}
fn main() {
    let nodes: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let edges: Vec<Vec<i32>> = vec![
        vec![0, 1],
        vec![0, 2],
        vec![0, 3],
        vec![0, 4],
        vec![0, 5],
        vec![0, 6],
        vec![6, 7],
        vec![7, 8],
        vec![8, 9],
    ];
    println!("{}", min_height(&nodes, &edges));
}

#[cfg(test)]
mod tests {
    #[test]
    fn is_all_discrete() {
        let nodes: Vec<i32> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let edges: Vec<Vec<i32>> = vec![
            vec![0, 1],
            vec![0, 2],
            vec![0, 3],
            vec![0, 4],
            vec![0, 5],
            vec![0, 6],
            vec![0, 7],
        ];
        assert_eq!(super::is_all_discrete(&nodes, &edges), false);
    }
}
