struct Solution;

impl Solution {
    pub fn even(chars: &Vec<char>, start: i32) -> i32 {
        let mut left = start;
        let mut right = start + 1;
        let mut count = 0;
        while left >= 0 && right < chars.len() as i32 {
            if chars[left as usize] == chars[right as usize] {
                count += 1;
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }
        count
    }

    pub fn odd(chars: &Vec<char>, start: i32) -> i32 {
        let mut left = start - 1;
        let mut right = start + 1;
        let mut count = 0;
        while left >= 0 && right < chars.len() as i32 {
            if chars[left as usize] == chars[right as usize] {
                count += 1;
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }
        count
    }
    pub fn count_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ans = 0;
        for i in 0..chars.len() {
            ans += Solution::odd(&chars, i as i32);
            ans += Solution::even(&chars, i as i32);
        }
        ans + chars.len() as i32
    }
}
fn main() {
    println!("{}", Solution::count_substrings("aaa".to_owned()));
}
