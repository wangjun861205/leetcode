fn n_digit_set(mut digits: Vec<String>, n: i32) -> i32 {
    digits.sort();
    let target_str = n.to_string();
    let target_len = target_str.len();
    let digits_len = digits.len();
    let mut total = 0;
    for i in 1..target_len {
        total += digits_len.pow(i as u32)
    }
    for (i, c) in target_str.chars().enumerate() {
        let mut has_same = false;
        for d in &digits {
            if d < &c.to_string() {
                total += digits_len.pow((target_len - 1 - i) as u32);
                continue;
            } else if d == &c.to_string() {
                has_same = true;
            }
        }
        if !has_same {
            return total as i32;
        }
    }
    (total + 1) as i32
}

use std::str::from_utf8;

pub fn repeated_substring_pattern(s: String) -> bool {
    for i in 1..=s.len() / 2 {
        if s.len() % i == 0 {
            let ns = from_utf8(&s.as_bytes()[0..i]).unwrap();
            if ns.repeat(s.len() / i) == s {
                return true;
            }
        }
    }
    false
}

fn main() {
    println!("{}", repeated_substring_pattern("abcaabc".to_owned()))
}
