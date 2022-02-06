struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(days: &Vec<i32>, costs: &Vec<i32>, i: usize, cache: &mut HashMap<usize, i32>) -> i32 {
        if i >= days.len() {
            return 0;
        }
        let find_next_index = |mut idx: usize, next_day: i32| {
            while idx < days.len() {
                if days[idx] >= next_day {
                    return idx;
                }
                idx += 1;
            }
            return idx;
        };
        let one = days[i] + 1;
        let one_i = find_next_index(i, one);
        let one_cost = if let Some(c) = cache.get(&one_i) { *c } else { Solution::dp(days, costs, one_i, cache) } + costs[0];

        let seven = days[i] + 7;
        let seven_i = find_next_index(i, seven);
        let seven_cost = if let Some(c) = cache.get(&seven_i) { *c } else { Solution::dp(days, costs, seven_i, cache) } + costs[1];

        let thirty = days[i] + 30;
        let thirty_i = find_next_index(i, thirty);
        let thirty_cost = if let Some(c) = cache.get(&thirty_i) { *c } else { Solution::dp(days, costs, thirty_i, cache) } + costs[2];

        let ans = one_cost.min(seven_cost.min(thirty_cost));
        cache.insert(i, ans);
        ans
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        Solution::dp(&days, &costs, 0, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]));
}
