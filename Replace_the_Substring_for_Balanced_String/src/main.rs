struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn balanced_string(s: String) -> i32 {
        let (mut q, mut w, mut e, mut r) = (0, 0, 0, 0);
        for c in s.chars() {
            match c {
                'Q' => q += 1,
                'W' => w += 1,
                'E' => e += 1,
                _ => r += 1,
            }
        }
        let quart = s.len() as i32 / 4;
        if q == quart && w == quart && e == quart && r == quart {
            return 0;
        }
        let mut m = HashMap::new();
        m.insert('Q', q - s.len() as i32 / 4);
        m.insert('W', w - s.len() as i32 / 4);
        m.insert('E', e - s.len() as i32 / 4);
        m.insert('R', r - s.len() as i32 / 4);
        let mut ans = s.len();
        let mut start = 0_usize;
        let mut end = 0_usize;
        let chars: Vec<char> = s.chars().collect();
        while m.values().any(|v| v > &0) {
            *m.get_mut(&chars[end]).unwrap() -= 1;
            end += 1;
        }
        loop {
            if let Some(c) = m.get_mut(&chars[start]) {
                if *c < 0 {
                    *c += 1;
                    start += 1;
                } else {
                    break;
                }
            }
        }
        ans = ans.min(end - start);
        while end < chars.len() {
            *m.get_mut(&chars[end]).unwrap() -= 1;
            end += 1;
            loop {
                if let Some(c) = m.get_mut(&chars[start]) {
                    if *c < 0 {
                        *c += 1;
                        start += 1;
                    } else {
                        break;
                    }
                }
            }
            ans = ans.min(end - start);
        }
        ans as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::balanced_string("WWEQERQWQWWRWWERQWEQ".to_owned())
    );
}
