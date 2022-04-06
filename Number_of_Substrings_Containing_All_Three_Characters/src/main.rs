struct Solution;

impl Solution {
    pub fn number_of_substrings(mut s: String) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut ans = 0;
        let mut dp = vec![(0, 0, 0, 0, 0, 0, 0); s.len()];
        let c = s.remove(0);
        match c {
            'a' => dp[0] = (1, 0, 0, 0, 0, 0, 0),
            'b' => dp[0] = (0, 1, 0, 0, 0, 0, 0),
            _ => dp[0] = (0, 0, 1, 0, 0, 0, 0),
        }
        for (i, c) in s.chars().enumerate() {
            match c {
                'a' => {
                    dp[i + 1] = (
                        dp[i].0 + 1,
                        0,
                        0,
                        dp[i].3 + dp[i].1,
                        dp[i].4 + dp[i].2,
                        0,
                        dp[i].6 + dp[i].5,
                    );
                }
                'b' => {
                    dp[i + 1] = (
                        0,
                        dp[i].1 + 1,
                        0,
                        dp[i].3 + dp[i].0,
                        0,
                        dp[i].5 + dp[i].2,
                        dp[i].6 + dp[i].4,
                    )
                }
                _ => {
                    dp[i + 1] = (
                        0,
                        0,
                        dp[i].2 + 1,
                        0,
                        dp[i].4 + dp[i].0,
                        dp[i].5 + dp[i].1,
                        dp[i].6 + dp[i].3,
                    )
                }
            }
            ans += dp[i + 1].6;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::number_of_substrings("aaacb".into()));
}
