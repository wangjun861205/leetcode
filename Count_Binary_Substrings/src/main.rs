struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ans = 0;
        for (i, w) in chars.windows(2).enumerate() {
            if w[0] != w[1] {
                ans += 1;
                let left_char = w[0];
                let right_char = w[1];
                let mut left_index = i;
                let mut right_index = i + 1;
                while left_index > 0 && right_index < chars.len() - 1 {
                    left_index -= 1;
                    right_index += 1;
                    if chars[left_index] == left_char && chars[right_index] == right_char {
                        ans += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::count_binary_substrings("1011001001".to_string()));
}
