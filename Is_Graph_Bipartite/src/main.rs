struct Solution;

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

fn main() {
    println!(
        "{}",
        Solution::is_bipartite(vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]])
    );
}
