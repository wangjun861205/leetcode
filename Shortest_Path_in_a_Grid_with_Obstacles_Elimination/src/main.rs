struct Solution;

use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut ans = vec![vec![vec![1600; k as usize + 1]; grid[0].len()]; grid.len()];
        ans[0][0][k as usize] = 0;
        let mut queue = vec![(0_usize, 0_usize, k as usize)];
        while !queue.is_empty() {
            let (i, j, k) = queue.remove(0);
            if i != 0 {
                if grid[i - 1][j] == 1 && k > 0 {
                    if ans[i - 1][j][k - 1] > ans[i][j][k] + 1 {
                        ans[i - 1][j][k - 1] = ans[i][j][k] + 1;
                        queue.push((i - 1, j, k - 1));
                    }
                } else if grid[i - 1][j] == 0 {
                    if ans[i - 1][j][k] > ans[i][j][k] + 1 {
                        ans[i - 1][j][k] = ans[i][j][k] + 1;
                        queue.push((i - 1, j, k));
                    }
                }
            }
            if i != grid.len() - 1 {
                if grid[i + 1][j] == 1 && k > 0 {
                    if ans[i + 1][j][k - 1] > ans[i][j][k] + 1 {
                        ans[i + 1][j][k - 1] = ans[i][j][k] + 1;
                        queue.push((i + 1, j, k - 1));
                    }
                } else if grid[i + 1][j] == 0 {
                    if ans[i + 1][j][k] > ans[i][j][k] + 1 {
                        ans[i + 1][j][k] = ans[i][j][k] + 1;
                        queue.push((i + 1, j, k));
                    }
                }
            }
            if j != 0 {
                if grid[i][j - 1] == 1 && k > 0 {
                    if ans[i][j - 1][k - 1] > ans[i][j][k] + 1 {
                        ans[i][j - 1][k - 1] = ans[i][j][k] + 1;
                        queue.push((i, j - 1, k - 1));
                    }
                } else if grid[i][j - 1] == 0 {
                    if ans[i][j - 1][k] > ans[i][j][k] + 1 {
                        ans[i][j - 1][k] = ans[i][j][k] + 1;
                        queue.push((i, j - 1, k));
                    }
                }
            }
            if j != grid[0].len() - 1 {
                if grid[i][j + 1] == 1 && k > 0 {
                    if ans[i][j + 1][k - 1] > ans[i][j][k] + 1 {
                        ans[i][j + 1][k - 1] = ans[i][j][k] + 1;
                        queue.push((i, j + 1, k - 1));
                    }
                } else if grid[i][j + 1] == 0 {
                    if ans[i][j + 1][k] > ans[i][j][k] + 1 {
                        ans[i][j + 1][k] = ans[i][j][k] + 1;
                        queue.push((i, j + 1, k));
                    }
                }
            }
        }
        let res = *ans.last().unwrap().last().unwrap().into_iter().min().unwrap();
        if res >= 1600 {
            -1
        } else {
            res
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::shortest_path(vec![vec![0, 1, 0, 0, 0, 1, 0, 0], vec![0, 1, 0, 1, 0, 1, 0, 1], vec![0, 0, 0, 1, 0, 0, 1, 0]], 1)
    );
}
