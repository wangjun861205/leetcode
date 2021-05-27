struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    fn rc(
        candidates: &Vec<i32>,
        target: i32,
        start_at: usize,
        cache: &mut HashMap<(i32, usize), HashSet<Vec<i32>>>,
    ) -> HashSet<Vec<i32>> {
        let mut set: HashSet<Vec<i32>> = HashSet::new();
        for i in start_at..candidates.len() {
            let next_target = target - candidates[i];
            if next_target == 0 {
                set.insert(vec![candidates[i]]);
            } else if next_target >= candidates[i] {
                let next_groups = if let Some(c) = cache.get(&(next_target, i + 1)) {
                    c.clone()
                } else {
                    Solution::rc(candidates, next_target, i + 1, cache)
                };
                if !next_groups.is_empty() {
                    for mut l in next_groups {
                        l.insert(0, candidates[i]);
                        set.insert(l);
                    }
                }
            }
        }
        cache.insert((target, start_at), set.clone());
        set
    }
    pub fn combination_sum2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut cache = HashMap::new();
        Solution::rc(&candidates, target, 0, &mut cache)
            .into_iter()
            .collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8)
    );
}
