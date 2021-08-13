struct Solution;

impl Solution {
    pub fn max_increase_keeping_skyline(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let row_max: Vec<i32> = grid.iter().map(|r| *r.iter().max().unwrap()).collect();
        let col_max: Vec<i32> = grid.iter().fold(vec![0; n], |mut l, r| {
            r.iter().enumerate().for_each(|(i, v)| l[i] = l[i].max(*v));
            l
        });
        let mut ans = 0;
        for i in 0..n {
            for j in 0..n {
                ans += row_max[i].min(col_max[j]) - grid[i][j];
            }
        }
        ans
    }
}
fn main() {
    println!(
        "{}",
        Solution::max_increase_keeping_skyline(vec![
            vec![3, 0, 8, 4],
            vec![2, 4, 5, 7],
            vec![9, 2, 6, 3],
            vec![0, 3, 1, 0]
        ])
    );
}
