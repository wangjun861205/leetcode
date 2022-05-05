struct Solution;

impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut present_s = String::new();
        for c in s.chars() {
            if c == '#' {
                present_s.pop();
                continue;
            }
            present_s.push(c);
        }
        let mut present_t = String::new();
        for c in t.chars() {
            if c == '#' {
                present_t.pop();
                continue;
            }
            present_t.push(c);
        }
        present_s == present_t
    }
}

fn main() {
    println!("Hello, world!");
}
