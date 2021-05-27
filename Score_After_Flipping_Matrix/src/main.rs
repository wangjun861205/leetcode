struct Solution;

impl Solution {
    pub fn matrix_score(mut grid: Vec<Vec<i32>>) -> i32 {
        for r in 0..grid.len() {
            if grid[r][0] == 0 {
                for c in grid[r].iter_mut() {
                    if c == &0 {
                        *c = 1;
                    } else {
                        *c = 0;
                    }
                }
            }
        }
        for c in 0..grid[0].len() {
            let mut one_count = 0;
            let mut zero_count = 0;
            for r in 0..grid.len() {
                if grid[r][c] == 0 {
                    zero_count += 1;
                } else {
                    one_count += 1;
                }
            }
            if one_count < zero_count {
                for r in 0..grid.len() {
                    if grid[r][c] == 0 {
                        grid[r][c] = 1;
                    } else {
                        grid[r][c] = 0;
                    }
                }
            }
        }
        grid.into_iter()
            .map(|r| {
                let s: String = r
                    .into_iter()
                    .map(|c| format!("{}", c))
                    .collect::<Vec<String>>()
                    .join("");
                i32::from_str_radix(&s, 2).unwrap()
            })
            .sum()
    }
}
fn main() {
    println!(
        "{}",
        Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]])
    );
}
