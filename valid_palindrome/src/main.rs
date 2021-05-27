struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let chars: Vec<char> = s
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if chars.len() == 0 {
            return true;
        }
        let mut i: usize = 0;
        let mut j: usize = chars.len() - 1;
        while i < j {
            if chars[i] != chars[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
