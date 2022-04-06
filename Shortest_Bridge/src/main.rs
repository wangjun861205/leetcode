struct Solution;

impl Solution {
    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        'outer: for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    grid[i][j] = 2;
                    break 'outer;
                }
            }
        }
        'preproc: loop {
            let mut updated = false;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] == 2 {
                        if i > 0 && grid[i - 1][j] == 1 {
                            grid[i - 1][j] = 2;
                            updated = true;
                        }
                        if j > 0 && grid[i][j - 1] == 1 {
                            grid[i][j - 1] = 2;
                            updated = true;
                        }
                        if i < grid.len() - 1 && grid[i + 1][j] == 1 {
                            grid[i + 1][j] = 2;
                            updated = true;
                        }
                        if j < grid[0].len() - 1 && grid[i][j + 1] == 1 {
                            grid[i][j + 1] = 2;
                            updated = true;
                        }
                    }
                }
            }
            if !updated {
                break 'preproc;
            }
        }
        let mut ans = i32::MAX;
        'main: loop {
            let mut updated = false;
            for i in 0..grid.len() {
                for j in 0..grid[0].len() {
                    if grid[i][j] >= 2 {
                        if i > 0 {
                            match grid[i - 1][j] {
                                0 => {
                                    grid[i - 1][j] = grid[i][j] + 1;
                                    updated = true
                                }
                                1 => {
                                    if ans > grid[i][j] - 2 {
                                        ans = grid[i][j] - 2;
                                        updated = true
                                    }
                                }
                                _ => {
                                    if grid[i - 1][j] > grid[i][j] + 1 {
                                        grid[i - 1][j] = grid[i][j] + 1;
                                        updated = true;
                                    }
                                }
                            }
                        }
                        if i < grid.len() - 1 {
                            match grid[i + 1][j] {
                                0 => {
                                    grid[i + 1][j] = grid[i][j] + 1;
                                    updated = true
                                }
                                1 => {
                                    if ans > grid[i][j] - 2 {
                                        ans = grid[i][j] - 2;
                                        updated = true
                                    }
                                }
                                _ => {
                                    if grid[i + 1][j] > grid[i][j] + 1 {
                                        grid[i + 1][j] = grid[i][j] + 1;
                                        updated = true;
                                    }
                                }
                            }
                        }
                        if j > 0 {
                            match grid[i][j - 1] {
                                0 => {
                                    grid[i][j - 1] = grid[i][j] + 1;
                                    updated = true
                                }
                                1 => {
                                    if ans > grid[i][j] - 2 {
                                        ans = grid[i][j] - 2;
                                        updated = true
                                    }
                                }
                                _ => {
                                    if grid[i][j - 1] > grid[i][j] + 1 {
                                        grid[i][j - 1] = grid[i][j] + 1;
                                        updated = true;
                                    }
                                }
                            }
                        }
                        if j < grid[0].len() - 1 {
                            match grid[i][j + 1] {
                                0 => {
                                    grid[i][j + 1] = grid[i][j] + 1;
                                    updated = true
                                }
                                1 => {
                                    if ans > grid[i][j] - 2 {
                                        ans = grid[i][j] - 2;
                                        updated = true
                                    }
                                }
                                _ => {
                                    if grid[i][j + 1] > grid[i][j] + 1 {
                                        grid[i][j + 1] = grid[i][j] + 1;
                                        updated = true;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if !updated {
                break 'main;
            }
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::shortest_bridge(vec![
            vec![0, 0, 1, 0, 1],
            vec![0, 1, 1, 0, 1],
            vec![0, 1, 0, 0, 1],
            vec![0, 0, 0, 0, 0],
            vec![0, 0, 0, 0, 0]
        ])
    );
}
