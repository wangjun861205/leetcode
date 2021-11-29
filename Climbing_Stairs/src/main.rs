struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        match n {
            1 => return 1,
            2 => return 2,
            _ => {
                let mut dp = vec![0; n as usize];
                dp[0] = 1;
                dp[1] = 2;
                for i in 2..n as usize {
                    dp[i] = dp[i - 1] + dp[i - 2];
                }
                return *dp.last().unwrap();
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}
