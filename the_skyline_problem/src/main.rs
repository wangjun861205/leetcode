use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn get_skyline(buildings: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if buildings.len() == 0 {
            return buildings;
        }
        let len = buildings.len();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for (i, build) in buildings.iter().enumerate() {
            if i == 0 {
                ans.push(vec![build[0], build[3]]);
                continue
            }
            buildings[0..i].iter().filter(|&b| b[0] <= build[0] && b[1] >= build[0] && b[2] > build[2]).
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::get_skyline(vec![
            vec![2, 9, 10],
            vec![3, 7, 15],
            vec![5, 12, 12],
            vec![15, 20, 10],
            vec![19, 24, 8]
        ])
    );
}
