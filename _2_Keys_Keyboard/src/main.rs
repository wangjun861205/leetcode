struct Solution;

impl Solution {
    pub fn min_steps(n: i32) -> i32 {
        let mut dp = vec![0; 1001];
        dp[1] = 0;
        'outer: for i in 2..=n {
            for j in (1..i).rev() {
                if i % j == 0 {
                    dp[i as usize] = dp[j as usize] + i / j;
                    continue 'outer;
                }
            }
        }
        dp[n as usize]
    }
}

fn main() {
    println!("{}", Solution::min_steps(2));
}
