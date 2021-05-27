struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut char = '-';
        let mut pattern = '-';
        let mut s_index = 0;
        let mut p_index = 0;
        while p_index < p.len() {
            let pat = p.as_bytes()[p_index] as char;
            match pat {
                '*' | '.' => pattern,
            }
        }
        false
    }
}
fn main() {
    println!("Hello, world!");
}
