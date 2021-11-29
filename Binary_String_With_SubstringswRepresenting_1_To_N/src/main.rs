struct Solution;

impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        let mut seen = vec![false; n as usize];
        let mut count = 0_usize;
        let chars: Vec<char> = s.chars().collect();
        for i in 0..s.len() {
            if chars[i] == '0' {
                continue;
            }
            let mut num = 1;
            if !seen[num - 1] {
                seen[num - 1] = true;
                count += 1;
            }
            for j in i + 1..s.len() {
                num = num * 2 + if chars[j] == '0' { 0 } else { 1 };
                if num > n as usize {
                    break;
                }
                if !seen[num - 1] {
                    seen[num - 1] = true;
                    count += 1;
                }
            }
            if count == n as usize {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!("{}", Solution::query_string("0110".to_owned(), 4));
}
