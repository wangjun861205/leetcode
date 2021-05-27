struct Solution;

impl Solution {
    pub fn check_record(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let mut absent = 0;
        for (i, c) in chars.iter().enumerate() {
            if c == &'A' {
                absent += 1;
                if absent == 2 {
                    return false;
                }
            }
            if c == &'L' {
                if i + 2 < chars.len() && chars[i + 1] == 'L' && chars[i + 2] == 'L' {
                    return false;
                }
            }
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
