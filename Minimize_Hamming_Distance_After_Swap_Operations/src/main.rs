struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn find(node: i32, parents: &Vec<i32>) -> i32 {
        if parents[node as usize] == -1 {
            return node;
        } else {
            return Solution::find(parents[node as usize], parents);
        }
    }

    pub fn minimum_hamming_distance(source: Vec<i32>, target: Vec<i32>, allowed_swaps: Vec<Vec<i32>>) -> i32 {
        let mut parents = vec![-1; source.len()];
        for p in allowed_swaps {
            let parent_source = Solution::find(p[0], &parents);
            let parent_target = Solution::find(p[1], &parents);
            if parent_source != parent_target {
                parents[parent_source as usize] = parent_target;
            }
        }
        let mut children: HashMap<i32, HashSet<i32>> = HashMap::new();
        for i in 0..parents.len() {
            let parent = Solution::find(i as i32, &parents);
            if parent != i as i32 {
                parents[i] = parent;
                children.entry(parent as i32).or_insert(HashSet::new()).insert(source[i]);
            }
        }
        let mut ans = 0;
        for (i, (s, t)) in source.clone().into_iter().zip(target).enumerate() {
            if s != t {
                let parent = Solution::find(i as i32, &parents);
                if t != source[parent as usize] {
                    if let Some(c) = children.get(&parent) {
                        if !c.contains(&t) {
                            ans += 1;
                        }
                    } else {
                        ans += 1;
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::minimum_hamming_distance(vec![5, 1, 2, 4, 3], vec![1, 5, 4, 2, 3], vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]])
    );
}
