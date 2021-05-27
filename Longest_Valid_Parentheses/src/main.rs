struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut ans = 0;
        let mut start = -1_i32;
        let mut stack: Vec<usize> = Vec::new();
        for i in 0..chars.len() {
            if chars[i] == '(' {
                stack.push(i);
            } else {
                if stack.is_empty() {
                    start = i as i32;
                } else {
                    stack.pop();
                    if stack.is_empty() {
                        ans = ans.max(i as i32 - start);
                    } else {
                        ans = ans.max((i - *stack.last().unwrap()) as i32);
                    }
                }
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::longest_valid_parentheses(")(((((()())()()))()(()))(".to_owned()));
}
