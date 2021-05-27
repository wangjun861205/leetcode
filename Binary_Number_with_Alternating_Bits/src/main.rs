struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let chars: Vec<char> = format!("{:b}", n).chars().collect();
        let mut prev = &chars[0];
        for c in &chars[1..] {
            if c == prev {
                return false;
            }
            prev = c;
        }
        true
    }
}
fn main() {
    println!("{}", Solution::has_alternating_bits(7));
}
