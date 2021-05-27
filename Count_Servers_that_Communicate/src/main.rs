struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_counts: Vec<i32> = vec![0; grid.len()];
        let mut col_counts: Vec<i32> = vec![0; grid[0].len()];
        let mut total = 0;
        for (i, r) in grid.iter().enumerate() {
            for (j, c) in r.iter().enumerate() {
                if c == &0 {
                    continue;
                }
                row_counts[i] += 1;
                col_counts[j] += 1;
                total += 1;
            }
        }
        for (i, r) in row_counts.iter().enumerate() {
            if r == &1 {
                for (j, c) in col_counts.iter().enumerate() {
                    if c == &1 && grid[i][j] == 1 {
                        total -= 1;
                    }
                }
            }
        }
        total
    }
}
fn main() {
    println!(
        "{}",
        Solution::count_servers(vec![
            vec![1, 1, 0, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 1, 0],
            vec![0, 0, 0, 1]
        ])
    );
}
