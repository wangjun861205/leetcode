struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn to_lower_case(s: String) -> String {
        s.chars()
            .map(|c| {
                if c >= 65 as char && c <= 90 as char {
                    (c as u8 + 32) as char
                } else {
                    c
                }
            })
            .collect()
    }
}
fn main() {
    println!("{}", Solution::to_lower_case("Hello".to_owned()));
}
