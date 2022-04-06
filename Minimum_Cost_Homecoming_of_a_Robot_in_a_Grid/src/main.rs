struct Solution;

impl Solution {
    pub fn min_cost(
        start_pos: Vec<i32>,
        home_pos: Vec<i32>,
        row_costs: Vec<i32>,
        col_costs: Vec<i32>,
    ) -> i32 {
        let (mut start_row, mut start_col) = (start_pos[0], start_pos[1]);
        let (home_row, home_col) = (home_pos[0], home_pos[1]);
        let mut ans = 0;
        while start_row != home_row {
            start_row += (home_row - start_row) / (home_row - start_row).abs();
            ans += row_costs[start_row as usize];
        }
        while start_col != home_col {
            start_col += (home_col - start_col) / (home_col - start_col).abs();
            ans += col_costs[start_col as usize];
        }
        ans
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_cost(vec![0, 0], vec![0, 0], vec![5], vec![26])
    );
}
