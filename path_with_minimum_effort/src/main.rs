struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn minimum_effort_path_with_cache(
        heights: Vec<Vec<i32>>,
        point: (i32, i32),
        points_passed: &mut HashSet<(i32, i32)>,
        cache: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        points_passed.insert(point);
        let top = (point.0 - 1, point.1);
        let left = (point.0, point.1 - 1);
        let bottom = (point.0 + 1, point.1);
        let right = (point.0, point.1 + 1);
        let l = vec![top, left, bottom, right];
        if l.iter()
            .any(|(x, y)| *x == heights.len() as i32 - 1 && *y == heights[0].len() as i32 - 1)
        {
            let effort = (heights[point.0 as usize][point.1 as usize]
                - heights[heights.len() - 1][heights[0].len() - 1])
                .abs();
            cache.insert(point, effort);
            return effort;
        }
        let min_l: Vec<i32> = l
            .iter()
            .filter(|p| {
                p.0 >= 0
                    && p.0 <= heights.len() as i32 - 1
                    && p.1 >= 0
                    && p.1 <= heights[0].len() as i32 - 1
            })
            .map(|p| {
                if points_passed.contains(&(p.clone())) {
                    return -1;
                }
                if cache.contains_key(p) {
                    return *cache.get(p).unwrap();
                } else {
                    let eff = heights[point.0 as usize][point.1 as usize]
                        - heights[p.0 as usize][p.1 as usize];
                    let next_eff = Solution::minimum_effort_path_with_cache(
                        heights.clone(),
                        p.clone(),
                        points_passed,
                        cache,
                    );
                    let min = eff.min(next_eff);
                    cache.insert(p.clone(), min);
                    return min;
                }
            })
            .collect();
        *min_l.iter().filter(|v| v.is_positive()).min().unwrap()
    }
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        let mut points_passed: HashSet<(i32, i32)> = HashSet::new();
        let mut cache: HashMap<(i32, i32), i32> = HashMap::new();
        Solution::minimum_effort_path_with_cache(
            heights,
            (0, 0),
            &mut points_passed.clone(),
            &mut cache,
        )
    }
}

fn main() {
    println!(
        "{}",
        Solution::minimum_effort_path(vec![vec![1, 2, 3], vec![3, 8, 4], vec![5, 3, 5]])
    );
}
