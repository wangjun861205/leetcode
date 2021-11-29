struct Solution;

use std::collections::{BinaryHeap, HashSet};

impl Solution {
    fn bfs(
        topping_costs: &Vec<i32>,
        curr: i32,
        idx: usize,
        set: &mut HashSet<i32>,
        heap: &mut BinaryHeap<i32>,
    ) {
        if idx == topping_costs.len() {
            if !set.contains(&curr) {
                heap.push(curr);
            }
            return;
        }
        for i in 0..3 {
            Solution::bfs(
                topping_costs,
                curr + topping_costs[idx] * i,
                idx + 1,
                set,
                heap,
            );
        }
    }
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        let mut heap = BinaryHeap::new();
        Solution::bfs(&topping_costs, 0, 0, &mut HashSet::new(), &mut heap);
        let l = heap.into_sorted_vec();
        let mut ans = base_costs[0];
        for b in base_costs {
            let remain = target - b;
            if remain == 0 {
                return b;
            }
            if remain < 0 {
                if (target - ans).abs() > remain.abs() {
                    ans = b;
                }
                continue;
            }
            let mut left = 0_usize;
            let mut right = l.len() - 1;
            while left < right {
                let m = left + (right - left) / 2;
                if l[m] == remain {
                    return target;
                }
                if l[m] < remain {
                    left = m + 1;
                } else {
                    right = m;
                }
            }
            let idx = left;
            if idx > 0 {
                if (target - ans).abs() > (target - b - l[idx]).abs()
                    || (target - ans).abs() == (target - b - l[idx]).abs() && ans > b + l[idx]
                {
                    ans = b + l[idx];
                }
                if (target - ans).abs() > (target - b - l[idx - 1]).abs()
                    || (target - ans).abs() == (target - b - l[idx - 1]).abs()
                        && ans > b + l[idx - 1]
                {
                    ans = b + l[idx - 1];
                }
            } else {
                if (target - ans).abs() > (target - b - l[idx]).abs()
                    || (target - ans).abs() == (target - b - l[idx]).abs() && ans > b + l[idx]
                {
                    ans = b + l[idx];
                }
                if (target - ans).abs() > (target - b)
                    || (target - ans).abs() == (target - b).abs() && ans > b
                {
                    ans = b;
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::closest_cost(vec![3, 2], vec![3], 10));
}
