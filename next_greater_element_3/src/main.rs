struct Solution;

use std::str;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut chars: Vec<u8> = n.to_string().as_bytes().to_vec();
        let max_index = chars.len() - 1;
        for (i, c) in chars.clone().iter().enumerate().rev() {
            match i {
                0 => return -1,
                _ => {
                    if c > &chars[i - 1] {
                        chars.swap(i, i - 1);
                        for (j, d) in chars.clone().iter().enumerate().skip(i) {
                            if j == max_index {
                                break;
                            } else {
                                if d > &chars[j + 1] {
                                    chars.swap(j, j + 1);
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
        str::from_utf8(&chars).unwrap().parse::<i32>().unwrap()
    }
}
fn main() {
    println!("{}", Solution::next_greater_element(230241));
}
