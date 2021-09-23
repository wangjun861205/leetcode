struct Solution;

use std::collections::HashSet;

impl Solution {
    fn find(parents: &mut Vec<i32>, n: i32, visited: &mut HashSet<i32>) -> i32 {
        if parents[n as usize] == n {
            return n;
        }
        if visited.contains(&n) {
            return -1;
        }
        let parent = parents[n as usize];
        visited.insert(n);
        let p = Solution::find(parents, parent, visited);
        parents[n as usize] = p;
        p
    }

    pub fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
        let mut parents: Vec<i32> = (0..n).into_iter().collect();
        for (i, l) in left_child.into_iter().enumerate() {
            if l >= 0 {
                parents[l as usize] = i as i32;
            }
        }
        for (i, r) in right_child.into_iter().enumerate() {
            if r >= 0 {
                if parents[r as usize] == r {
                    parents[r as usize] = i as i32;
                } else if parents[r as usize] != i as i32 {
                    return false;
                }
            }
        }
        for i in 0..parents.len() {
            let mut visited = HashSet::new();
            let p = Solution::find(&mut parents, i as i32, &mut visited);
            if p == -1 {
                return false;
            }
        }
        let mut root_count = 0;
        for (i, p) in parents.into_iter().enumerate() {
            if i == p as usize {
                root_count += 1;
            }
        }
        root_count == 1
    }
}

fn main() {
    println!(
        "{}",
        Solution::validate_binary_tree_nodes(5, vec![1, 3, -1, -1, -1], vec![-1, 2, 4, -1, -1])
    );
}
