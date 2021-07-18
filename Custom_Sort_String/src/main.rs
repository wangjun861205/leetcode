struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn custom_sort_string(order: String, str: String) -> String {
        let mut chars = str.chars().fold(HashMap::new(), |mut m, v| {
            *m.entry(v).or_insert(0) += 1;
            m
        });
        let mut s = String::new();
        for o in order.chars() {
            if let Some(count) = chars.remove(&o) {
                s.push_str(&str::repeat(&o.to_string(), count));
            }
        }
        for (c, count) in chars {
            s.push_str(&str::repeat(&c.to_string(), count));
        }
        s
    }
}
fn main() {
    println!(
        "{}",
        Solution::custom_sort_string("cba".to_owned(), "abcd".to_owned())
    );
}
