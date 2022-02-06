struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn nearest_exit(maze: Vec<Vec<char>>, entrance: Vec<i32>) -> i32 {
        let mut points = vec![(entrance[0], entrance[1], 0)];
        let mut steps = HashMap::new();
        let mut ans = i32::MAX;

        while !points.is_empty() {
            let mut new_points = Vec::new();
            for p in points {
                if p.0 == 0 || p.0 == maze.len() as i32 - 1 || p.1 == 0 || p.1 == maze[0].len() as i32 - 1 {
                    if p.0 != entrance[0] || p.1 != entrance[1] {
                        ans = ans.min(p.2);
                        continue;
                    }
                }
                if p.0 > 0 && maze[p.0 as usize - 1][p.1 as usize] != '+' {
                    let entry = steps.entry((p.0 - 1, p.1)).or_insert(i32::MAX);
                    if *entry > p.2 + 1 {
                        *entry = p.2 + 1;
                        new_points.push((p.0 - 1, p.1, p.2 + 1));
                    }
                }
                if p.0 < maze.len() as i32 - 1 && maze[p.0 as usize + 1][p.1 as usize] != '+' {
                    let entry = steps.entry((p.0 + 1, p.1)).or_insert(i32::MAX);
                    if *entry > p.2 + 1 {
                        *entry = p.2 + 1;
                        new_points.push((p.0 + 1, p.1, p.2 + 1));
                    }
                }
                if p.1 > 0 && maze[p.0 as usize][p.1 as usize - 1] != '+' {
                    let entry = steps.entry((p.0, p.1 - 1)).or_insert(i32::MAX);
                    if *entry > p.2 + 1 {
                        *entry = p.2 + 1;
                        new_points.push((p.0, p.1 - 1, p.2 + 1));
                    }
                }
                if p.1 < maze[0].len() as i32 - 1 && maze[p.0 as usize][p.1 as usize + 1] != '+' {
                    let entry = steps.entry((p.0, p.1 + 1)).or_insert(i32::MAX);
                    if *entry > p.2 + 1 {
                        *entry = p.2 + 1;
                        new_points.push((p.0, p.1 + 1, p.2 + 1));
                    }
                }
            }
            points = new_points;
        }
        if ans == i32::MAX {
            -1
        } else {
            ans
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::nearest_exit(
            vec![
                vec!['+', '.', '+', '+', '+', '+', '+'],
                vec!['+', '.', '+', '.', '.', '.', '+'],
                vec!['+', '.', '+', '.', '+', '.', '+'],
                vec!['+', '.', '.', '.', '+', '.', '+'],
                vec!['+', '+', '+', '+', '+', '.', '+']
            ],
            vec![0, 1]
        )
    );
}
