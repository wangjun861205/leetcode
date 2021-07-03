struct Solution;

impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];
        let mut ans = 0;
        let module: i32 = 1000000000 + 7;
        dp[start_row as usize][start_column as usize] = 1;
        for _ in 0..max_move {
            let mut tmp = vec![vec![0; n as usize]; m as usize];
            for row in 0..m as usize {
                for col in 0..n as usize {
                    if row == 0 {
                        ans = (ans + dp[row][col]) % module;
                    } else {
                        tmp[row - 1][col] = (tmp[row - 1][col] + dp[row][col]) % module;
                    }
                    if row == m as usize - 1 {
                        ans = (ans + dp[row][col]) % module;
                    } else {
                        tmp[row + 1][col] = (tmp[row + 1][col] + dp[row][col]) % module;
                    }
                    if col == 0 {
                        ans = (ans + dp[row][col]) % module;
                    } else {
                        tmp[row][col - 1] = (tmp[row][col - 1] + dp[row][col]) % module;
                    }
                    if col == n as usize - 1 {
                        ans = (ans + dp[row][col]) % module;
                    } else {
                        tmp[row][col + 1] = (tmp[row][col + 1] + dp[row][col]) % module;
                    }
                }
            }
            dp = tmp;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::find_paths(1, 3, 3, 0, 1));
}
