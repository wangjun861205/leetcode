struct Solution;

impl Solution {
    pub fn oranges_rotting(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut total_orange = 0;
        let mut total_rotten = 0;
        for r in &grid {
            for c in r {
                if c == &2 {
                    total_orange += 1;
                    total_rotten += 1;
                } else if c == &1 {
                    total_orange += 1;
                }
            }
        }
        let mut minutes = 0;
        while total_rotten < total_orange {
            let mut cur_rotten = total_rotten;
            for r in 0..grid.len() {
                for c in 0..grid[0].len() {
                    if grid[r][c] == 2 {
                        if r != 0 && grid[r - 1][c] == 1 {
                            grid[r - 1][c] = 3;
                            cur_rotten += 1;
                        }
                        if r != grid.len() - 1 && grid[r + 1][c] == 1 {
                            grid[r + 1][c] = 3;
                            cur_rotten += 1;
                        }
                        if c != 0 && grid[r][c - 1] == 1 {
                            grid[r][c - 1] = 3;
                            cur_rotten += 1;
                        }
                        if c != grid[0].len() - 1 && grid[r][c + 1] == 1 {
                            grid[r][c + 1] = 3;
                            cur_rotten += 1;
                        }
                    }
                }
            }
            grid.iter_mut().for_each(|r| {
                r.iter_mut().for_each(|c| {
                    if c == &3 {
                        *c = 2
                    }
                })
            });
            if cur_rotten == total_rotten {
                return -1;
            }
            total_rotten = cur_rotten;
            minutes += 1;
        }
        minutes
    }
}
fn main() {
    println!(
        "{}",
        Solution::oranges_rotting(vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]])
    );
}
