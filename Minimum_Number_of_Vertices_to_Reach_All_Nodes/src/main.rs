struct Solution;

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

fn main() {
    println!(
        "{:?}",
        Solution::find_smallest_set_of_vertices(
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]
        )
    );
}
