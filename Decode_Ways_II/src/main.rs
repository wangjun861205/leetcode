struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        const M: i64 = 1000000007;
        let mut dp = vec![vec![0_i64; 2]; s.len()];
        let chars: Vec<char> = s.chars().collect();
        match chars[0] {
            '*' => {
                dp[0][0] = 9;
            }
            '0' => {}
            _ => {
                dp[0][0] = 1;
            }
        }
        for i in 1..chars.len() {
            let curr = chars[i];
            let prev = chars[i - 1];
            // 计算独立组合数量
            match curr {
                '*' => dp[i][0] = (dp[i - 1][0] + dp[i - 1][1]) * 9 % M,
                '0' => {}
                _ => {
                    dp[i][0] = dp[i - 1][0] + dp[i - 1][1];
                }
            }
            // 计算向前组合数量
            match curr {
                '*' => match prev {
                    '*' => {
                        dp[i][1] = dp[i - 1][0] / 9 * 15;
                    }
                    '1' => {
                        dp[i][1] = dp[i - 1][0] * 9 % M;
                    }
                    '2' => {
                        dp[i][1] = dp[i - 1][0] * 6 % M;
                    }
                    _ => {}
                },
                '0' => match prev {
                    '*' => {
                        dp[i][1] = dp[i - 1][0] / 9 * 2 % M;
                    }
                    '1' | '2' => {
                        dp[i][1] = dp[i - 1][0] % M;
                    }
                    _ => {}
                },
                '1' | '2' | '3' | '4' | '5' | '6' => match prev {
                    '*' => {
                        dp[i][1] = dp[i - 1][0] / 9 * 2 % M;
                    }
                    '1' | '2' => {
                        dp[i][1] = dp[i - 1][0] % M;
                    }
                    _ => {}
                },
                _ => match prev {
                    '*' => {
                        dp[i][1] = dp[i - 1][0] / 9 % M;
                    }
                    '1' => {
                        dp[i][1] = dp[i - 1][0] % M;
                    }
                    _ => {}
                },
            }
        }
        let last = dp.last().unwrap();
        ((last[0] + last[1]) % M) as i32
    }
}

fn main() {
    // println!(
    //     "{}",
    //     Solution::num_decodings("7*9*3*6*3*0*5*4*9*7*3*7*1*8*3*2*0*0*6*".to_owned())
    // );
    println!("{}", Solution::num_decodings("**".to_owned()));
}
