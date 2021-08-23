struct Solution;

impl Solution {
    pub fn maximum_binary_string(binary: String) -> String {
        let mut chars: Vec<char> = binary.chars().collect();
        let mut prev_zero_index: Option<usize> = None;
        for i in 0..chars.len() {
            if chars[i] == '0' {
                if let Some(p) = prev_zero_index {
                    chars[p] = '1';
                    chars[i] = '1';
                    chars[p + 1] = '0';
                    prev_zero_index = Some(p + 1);
                } else {
                    prev_zero_index = Some(i);
                }
            }
        }
        chars.into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::maximum_binary_string("000110".to_owned()));
}
