struct Solution;

impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut chars: Vec<char> = x.to_string().chars().collect();
        if x < 0 {
            chars = chars[1..].to_vec();
            chars.reverse();
            chars.insert(0, '-');
        } else {
            chars.reverse();
        }
        chars
            .into_iter()
            .collect::<String>()
            .parse::<i32>()
            .unwrap_or(0)
    }
}
fn main() {
    println!("Hello, world!");
}
