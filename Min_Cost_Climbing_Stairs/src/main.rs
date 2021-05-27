struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(cost: &Vec<i32>, index: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if index == cost.len() - 1 || index == cost.len() - 2 {
            return cost[index];
        }
        let c = cost[index];
        let one_step = if let Some(v) = cache.get(&(index + 1)) {
            *v
        } else {
            Solution::rc(cost, index + 1, cache)
        };
        let two_steps = if let Some(v) = cache.get(&(index + 2)) {
            *v
        } else {
            Solution::rc(cost, index + 2, cache)
        };
        let ans = one_step.min(two_steps) + c;
        cache.insert(index, ans);
        ans
    }
    pub fn min_cost_climbing_stairs(cost: Vec<i32>) -> i32 {
        let mut cache = HashMap::new();
        let one_step = Solution::rc(&cost, 0, &mut cache);
        let two_steps = Solution::rc(&cost, 1, &mut cache);
        one_step.min(two_steps)
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_cost_climbing_stairs(vec![1, 100, 1, 1, 1, 100, 1, 1, 100, 1])
    );
}
