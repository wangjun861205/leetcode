struct Solution;

impl Solution {
    pub fn even(s: String) -> String {
        let b = s.as_bytes();
        let mut ans = String::new();
        for i in 0..b.len() - 1 {
            if b[i] == b[i + 1] {
                let mut left_index = i;
                let mut right_index = i + 1;
                while left_index != 0
                    && right_index != b.len() - 1
                    && b[left_index - 1] == b[right_index + 1]
                {
                    left_index -= 1;
                    right_index += 1;
                }
                let p = s[left_index..=right_index].to_owned();
                if ans.len() < p.len() {
                    ans = p;
                }
            }
        }
        ans
    }
    pub fn longest_palindrome(s: String) -> String {
        if s.len() == 1 {
            return s;
        }
        let mut ans = s.chars().nth(0).unwrap().to_string();
        for i in 1..s.len() - 1 {
            let mut left_index = i as i32;
            let mut right_index = i as i32;
            while left_index - 1 >= 0
                && right_index + 1 < s.len() as i32
                && s.as_bytes()[left_index as usize - 1] == s.as_bytes()[right_index as usize + 1]
            {
                left_index -= 1;
                right_index += 1;
            }
            if right_index - left_index + 1 > ans.len() as i32 {
                ans = s[left_index as usize..=right_index as usize].to_owned();
            }
        }
        let even_ans = Solution::even(s.clone());
        if even_ans.len() > ans.len() {
            even_ans
        } else {
            ans
        }
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_palindrome((0..1000).map(|_| 'a').collect()).len()
    );
}
