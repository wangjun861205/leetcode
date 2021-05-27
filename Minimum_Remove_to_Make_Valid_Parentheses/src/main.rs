struct Solution;
impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack: Vec<usize> = Vec::new();
        let mut chars: Vec<char> = s.chars().collect();
        for (i, c) in chars.clone().iter().enumerate() {
            if c == &'(' {
                stack.push(i);
            } else if c == &')' {
                if !stack.is_empty() {
                    stack.pop();
                } else {
                    chars[i] = '-';
                }
            }
        }
        for i in stack {
            chars[i] = '-';
        }
        chars.into_iter().filter(|v| v != &'-').collect()
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_remove_to_make_valid("(a(b(c)d)".to_owned())
    );
}
