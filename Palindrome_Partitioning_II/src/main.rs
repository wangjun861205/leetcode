struct Solution;

use std::collections::HashMap;

impl Solution {
    fn is_palindrome(chars: &[char]) -> bool {
        let mut i = 0 as i32;
        let mut j = chars.len() as i32 - 1;
        while i < j {
            if chars[i as usize] != chars[j as usize] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
    pub fn cut(chars: &Vec<char>, cache: &mut HashMap<Vec<char>, i32>) -> i32 {
        if chars.is_empty() {
            return 0;
        }
        if Solution::is_palindrome(chars) {
            return 0;
        }
        let mut ans = chars.len() as i32 - 1;
        for i in 0..chars.len() {
            if Solution::is_palindrome(&chars[..=i]) {
                let remain = if let Some(c) = cache.get(&chars[i + 1..]) {
                    *c
                } else {
                    Solution::cut(&chars[i + 1..].to_vec(), cache)
                };
                ans = ans.min(remain);
            }
        }
        cache.insert(chars[..].to_vec(), ans + 1);
        ans + 1
    }
    pub fn min_cut(s: String) -> i32 {
        let mut cache = HashMap::new();
        Solution::cut(&s.chars().collect(), &mut cache)
    }
}
fn main() {
    println!("{}", Solution::min_cut("a".to_owned()));
}
