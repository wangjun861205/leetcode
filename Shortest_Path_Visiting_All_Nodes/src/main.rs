struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        let node_count = graph.len();
        let masks: Vec<i32> = (0..graph.len()).map(|i| 1 << i).collect();
        let all_visited = (1 << node_count) - 1;
        let mut queue: Vec<(i32, i32)> = (0..node_count).map(|i| (i as i32, masks[i])).collect();
        let mut steps = 0;
        let mut visited_states: Vec<HashSet<i32>> = (0..node_count)
            .map(|i| vec![masks[i]].into_iter().collect())
            .collect();
        while !queue.is_empty() {
            let mut count = queue.len();
            while count > 0 {
                let (curr_node, visited) = queue.remove(0);
                if visited == all_visited {
                    return steps;
                }
                for &nb in &graph[curr_node as usize] {
                    let new_visited = visited | masks[nb as usize];
                    if new_visited == all_visited {
                        return steps + 1;
                    }
                    if !visited_states[nb as usize].contains(&new_visited) {
                        visited_states[nb as usize].insert(new_visited);
                        queue.push((nb, new_visited))
                    }
                }

                count -= 1
            }
            steps += 1
        }
        return i32::MAX;
    }
}

fn main() {
    println!(
        "{}",
        Solution::shortest_path_length(vec![vec![1, 2, 3], vec![0], vec![0], vec![0]])
    );
    println!(
        "{}",
        Solution::shortest_path_length(vec![
            vec![1],
            vec![0, 2, 4],
            vec![1, 3, 4],
            vec![2],
            vec![1, 2]
        ])
    );
}
