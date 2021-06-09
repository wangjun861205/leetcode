struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut chars: Vec<char> = s.chars().collect();
        let mut count = 0;
        while chars.len() > 1 {
            if *chars.last().unwrap() == '1' {
                if let Some(pos) = chars.iter().rposition(|v| v == &'0') {
                    count += chars.len() - pos;
                    chars[pos] = '1';
                    chars = chars[..=pos].to_vec();
                } else {
                    count += chars.len() + 1;
                    break;
                }
            } else {
                chars.pop();
                count += 1;
            }
        }
        count as i32
    }
}
fn main() {
    println!("{}", Solution::num_steps("1101".to_owned()));
}
