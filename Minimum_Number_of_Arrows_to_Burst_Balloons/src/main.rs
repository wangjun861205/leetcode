struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(points: &Vec<Vec<i32>>, start: i32, end: i32, idx: usize, cache: &mut HashMap<(i32, i32, usize), i32>) -> i32 {
        let p = points[idx].clone();
        if idx == points.len() - 1 {
            if start == -1 && end == -1 {
                return 1;
            }
            if end < p[0] {
                return 2;
            }
            return 1;
        }
        if start == -1 && end == -1 {
            let shot = if let Some(c) = cache.get(&(-1, -1, idx + 1)) {
                *c
            } else {
                Solution::dp(points, -1, -1, idx + 1, cache)
            } + 1;
            let pass = if let Some(c) = cache.get(&(p[0], p[1], idx + 1)) {
                *c
            } else {
                Solution::dp(points, p[0], p[1], idx + 1, cache)
            };
            let ans = shot.min(pass);
            cache.insert((start, end, idx), ans);
            return ans;
        } else {
            if end < p[0] {
                let shot = if let Some(c) = cache.get(&(-1, -1, idx + 1)) {
                    *c
                } else {
                    Solution::dp(points, -1, -1, idx + 1, cache)
                } + 2;
                let pass = if let Some(c) = cache.get(&(p[0], p[1], idx + 1)) {
                    *c
                } else {
                    Solution::dp(points, p[0], p[1], idx + 1, cache)
                } + 1;
                let ans = shot.min(pass);
                cache.insert((start, end, idx), ans);
                return ans;
            } else {
                let next_start = p[0];
                let next_end = end.min(p[1]);
                let shot = if let Some(c) = cache.get(&(-1, -1, idx + 1)) {
                    *c
                } else {
                    Solution::dp(points, -1, -1, idx + 1, cache)
                } + 1;
                let pass = if let Some(c) = cache.get(&(next_start, next_end, idx + 1)) {
                    *c
                } else {
                    Solution::dp(points, next_start, next_end, idx + 1, cache)
                };
                let ans = shot.min(pass);
                cache.insert((start, end, idx), ans);
                return ans;
            }
        }
    }

    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        points.sort_by_key(|v| v[0]);
        Solution::dp(&points, -1, -1, 0, &mut HashMap::new())
    }
}

fn main() {
    println!("{}", Solution::find_min_arrow_shots(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]));
}
