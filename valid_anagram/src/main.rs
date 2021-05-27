struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut sl: Vec<char> = s.chars().collect();
        let mut tl: Vec<char> = t.chars().collect();
        sl.sort();
        tl.sort();
        for tup in sl.iter().zip(tl.iter()) {
            if tup.0 != tup.1 {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
