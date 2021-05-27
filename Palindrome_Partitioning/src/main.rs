struct Solution;

use std::collections::HashMap;

impl Solution {
    fn check(chars: &Vec<char>) -> bool {
        let mut left = 0;
        let mut right = chars.len() - 1;
        while left < right {
            if chars[left] != chars[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
    fn rc(chars: Vec<char>, cache: &mut HashMap<usize, Vec<Vec<String>>>) -> Vec<Vec<String>> {
        if chars.len() == 0 {
            return Vec::new();
        }
        let mut ans: Vec<Vec<String>> = Vec::new();
        for i in 0..chars.len() {
            let cur = chars[..=i].to_vec();
            if Solution::check(&cur) {
                let s: String = cur.into_iter().collect();
                let next = if let Some(c) = cache.get(&(chars.len() - i - 1)) {
                    c.clone()
                } else {
                    Solution::rc(chars[i + 1..].to_vec(), cache)
                };
                if next.len() == 0 {
                    ans.push(vec![s])
                } else {
                    for mut n in next {
                        n.insert(0, s.clone());
                        ans.push(n);
                    }
                }
            }
        }
        cache.insert(chars.len(), ans.clone());
        ans
    }
    pub fn partition(s: String) -> Vec<Vec<String>> {
        let chars = s.chars().collect();
        let mut cache = HashMap::new();
        Solution::rc(chars, &mut cache)
    }
}

fn main() {
    println!("Hello, world!");
}
