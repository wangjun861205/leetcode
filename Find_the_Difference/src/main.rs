struct Solution;

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut counts = vec![0; 26];
        for c in t.chars() {
            counts[c as usize - 97] += 1;
        }
        for c in s.chars() {
            counts[c as usize - 97] -= 1;
        }
        for (i, v) in counts.into_iter().enumerate() {
            if v == 1 {
                return (i as u8 + 97) as char;
            }
        }
        unreachable!()
    }
}

fn main() {
    println!("Hello, world!");
}
