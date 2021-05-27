struct Solution;

impl Solution {
    pub fn make_good(s: String) -> String {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            if stack.len() == 0 {
                stack.push(c);
            } else {
                if stack.last().unwrap() != &c
                    && stack.last().unwrap().to_ascii_lowercase() == c.to_ascii_lowercase()
                {
                    stack.pop();
                } else {
                    stack.push(c);
                }
            }
        }
        stack.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::make_good("abBAcC".to_string()));
}
