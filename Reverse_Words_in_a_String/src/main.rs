struct Solution;

impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut buf: Vec<char> = Vec::new();
        let mut l: Vec<String> = Vec::new();
        for c in s.chars() {
            if c == ' ' {
                if buf.len() > 0 {
                    l.push(buf.iter().collect());
                    buf.clear();
                }
            } else {
                buf.push(c)
            }
        }
        if buf.len() > 0 {
            l.push(buf.iter().collect());
        }
        l.reverse();
        l.join(" ")
    }
}
fn main() {
    println!(
        "{}",
        Solution::reverse_words("  Bob    Loves  Alice   ".to_owned())
    );
}
