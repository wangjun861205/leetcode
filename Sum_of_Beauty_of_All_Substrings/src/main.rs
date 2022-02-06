struct Solution;

impl Solution {
    pub fn beauty_sum(s: String) -> i32 {
        let mut frq = vec![vec![0; s.len() + 1]; 26];
        for (i, c) in s.chars().enumerate() {
            for j in 0..26 {
                if j == (c as usize) - 97 {
                    frq[j][i + 1] = frq[j][i] + 1;
                } else {
                    frq[j][i + 1] = frq[j][i];
                }
            }
        }
        let mut ans = 0;
        for i in 0..s.len() + 1 {
            for j in 0..i {
                let mut min = i32::MAX;
                let mut max = 0;
                for k in 0..26 {
                    let diff = frq[k][i] - frq[k][j];
                    if diff == 0 {
                        continue;
                    }
                    min = min.min(diff);
                    max = max.max(diff);
                }
                ans += max - min;
            }
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::beauty_sum("aabcbaa".into()));
}
