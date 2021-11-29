struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(
        obstacles: &Vec<i32>,
        lane: i32,
        point: usize,
        cache: &mut HashMap<(i32, usize), i32>,
    ) -> i32 {
        if point == obstacles.len() - 1 {
            return 0;
        }
        let curr_obstacle = obstacles[point];
        let next_obstacle = obstacles[point + 1];
        if next_obstacle != lane {
            let next = if let Some(c) = cache.get(&(lane, point + 1)) {
                *c
            } else {
                Solution::dp(obstacles, lane, point + 1, cache)
            };
            cache.insert((lane, point), next);
            return next;
        }
        let bits = !((1 << curr_obstacle >> 1) ^ (1 << next_obstacle >> 1));
        let mut ans = i32::MAX;
        for i in 0..3 {
            if (bits >> i) & 1 == 1 {
                let next = if let Some(c) = cache.get(&(i + 1, point)) {
                    *c
                } else {
                    Solution::dp(obstacles, i + 1, point, cache)
                };
                ans = ans.min(next + 1);
            }
        }
        cache.insert((lane, point), ans);
        return ans;
    }
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        Solution::dp(&obstacles, 2, 0, &mut HashMap::new())
    }
}
fn main() {
    println!("{}", Solution::min_side_jumps(vec![0, 2, 1, 0, 3, 0]));
}
