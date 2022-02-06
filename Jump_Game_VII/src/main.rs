struct Solution;

impl Solution {
    pub fn can_reach(s: String, min_jump: i32, max_jump: i32) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut dp = vec![false; chars.len()];
        dp[0] = true;
        let mut pre_count = 0;
        for i in 1..chars.len() as i32 {
            if i >= min_jump && dp[i as usize - min_jump as usize] {
                pre_count += 1;
            }
            if i > max_jump && dp[i as usize - max_jump as usize - 1] {
                pre_count -= 1;
            }
            if chars[i as usize] == '0' && pre_count > 0 {
                dp[i as usize] = true;
            }
        }
        *dp.last().unwrap()
    }
}

fn main() {
    println!("{}", Solution::can_reach("01101110".into(), 2, 3));
}
