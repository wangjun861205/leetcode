struct Solution;

impl Solution {
    pub fn next_greater_element(n: i32) -> i32 {
        let mut chars: Vec<char> = n.to_string().chars().collect();
        let mut pos = None;
        'outer: for i in (0..chars.len() - 1).rev() {
            let mut idx = None;
            for j in i + 1..chars.len() {
                if chars[j] > chars[i] {
                    if idx.is_none() {
                        idx = Some(j);
                    } else {
                        if chars[j] < chars[idx.unwrap()] {
                            idx = Some(j);
                        }
                    }
                }
            }
            if let Some(j) = idx {
                pos = Some(i);
                chars.swap(i, j);
                break 'outer;
            }
        }
        if let Some(p) = pos {
            let mut tail: Vec<char> = chars.drain(p + 1..).collect();
            tail.sort();
            chars.append(&mut tail);
            return chars
                .into_iter()
                .collect::<String>()
                .parse::<i32>()
                .unwrap_or(-1);
        }
        -1
    }
}
fn main() {
    println!("{}", Solution::next_greater_element(230241));
}
