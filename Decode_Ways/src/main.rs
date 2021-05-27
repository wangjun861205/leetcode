struct Solution;

use std::collections::HashMap;

impl Solution {
    fn rc(s: String, cache: &mut HashMap<usize, i32>) -> i32 {
        if s.len() == 0 {
            return 1;
        }
        if s.len() == 1 {
            if s == "0" {
                return 0;
            } else {
                return 1;
            }
        }
        let mut ans = 0;
        for i in 1..=26 {
            let start = i.to_string();
            if s.starts_with(&start) {
                if let Some(c) = cache.get(&(s.len() - start.len())) {
                    ans += *c;
                } else {
                    let mut ss = s.clone();
                    ss.drain(..(if i > 9 { 2 } else { 1 }));
                    ans += Solution::rc(ss, cache);
                }
            }
        }
        cache.insert(s.len(), ans);
        ans
    }
    pub fn num_decodings(s: String) -> i32 {
        let mut cache = HashMap::new();
        Solution::rc(s, &mut cache)
    }
}
fn main() {
    println!("{}", Solution::num_decodings("226".to_owned()));
}
