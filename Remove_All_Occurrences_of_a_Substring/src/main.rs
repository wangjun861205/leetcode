struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack = Vec::new();
        let p: Vec<char> = part.chars().collect();
        for c in s.chars() {
            stack.push(c);
            if c == *p.last().unwrap() {
                if stack.len() < p.len() {
                    continue;
                }
                if stack[stack.len() - p.len()..] != p[..] {
                    continue;
                }
                stack = stack[..stack.len() - p.len()].to_vec();
            }
        }
        stack.into_iter().collect()
    }
}
fn main() {
    println!(
        "{}",
        Solution::remove_occurrences("daabcbaabcbc".to_owned(), "abc".to_owned())
    );
}
