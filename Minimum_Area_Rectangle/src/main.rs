struct Solution;

use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn min_area_rect(points: Vec<Vec<i32>>) -> i32 {
        let all_points: HashSet<(i32, i32)> = points.iter().map(|v| (v[0], v[1])).collect();
        let same_y: HashMap<i32, Vec<(i32, i32)>> =
            all_points.iter().fold(HashMap::new(), |mut m, p| {
                m.entry(p.0).or_insert(Vec::new()).push(*p);
                m
            });
        let same_x: HashMap<i32, Vec<(i32, i32)>> =
            all_points.iter().fold(HashMap::new(), |mut m, p| {
                m.entry(p.1).or_insert(Vec::new()).push(*p);
                m
            });
        let mut ans = i32::MAX;
        for p in &all_points {
            if let Some(ly) = same_y.get(&p.0) {
                for py in ly {
                    if p != py {
                        if let Some(lx) = same_x.get(&p.1) {
                            for px in lx {
                                if p != px {
                                    if all_points.contains(&(px.0, py.1)) {
                                        ans = ans.min((p.0 - px.0).abs() * (p.1 - py.1).abs());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        if ans == i32::MAX {
            0
        } else {
            ans
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_area_rect(vec![
            vec![1, 1],
            vec![1, 3],
            vec![3, 1],
            vec![3, 3],
            vec![4, 1],
            vec![4, 3]
        ])
    );
}
