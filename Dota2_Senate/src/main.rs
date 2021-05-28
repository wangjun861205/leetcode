struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn predict_party_victory(senate: String) -> String {
        let mut senators: Vec<char> = Vec::new();
        let mut r_count = 0;
        let mut d_count = 0;
        for c in senate.chars() {
            if c == 'R' {
                r_count += 1;
            } else {
                d_count += 1;
            }
            senators.push(c);
        }
        while r_count != 0 && d_count != 0 {
            for i in 0..senators.len() {
                let s = senators[i];
                if s == '-' {
                    continue;
                }
                if let Some(idx) = senators.iter().skip(i + 1).position(|v| v != &s && v != &'-') {
                    senators[idx + i + 1] = '-';
                    if s == 'R' {
                        d_count -= 1;
                    } else {
                        r_count -= 1;
                    }
                } else {
                    if let Some(idx) = senators[..i].into_iter().position(|v| v != &s && v != &'-') {
                        senators[idx] = '-';
                        if s == 'R' {
                            d_count -= 1;
                        } else {
                            r_count -= 1;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        if r_count == 0 {
            return "Dire".to_owned();
        } else {
            return "Radiant".to_owned();
        }
    }
}
fn main() {
    println!("{}", Solution::predict_party_victory("RD".to_owned()));
}
