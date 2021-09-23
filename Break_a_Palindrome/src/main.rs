struct Solution;

impl Solution {
    pub fn break_palindrome(palindrome: String) -> String {
        if palindrome.len() == 1 {
            return "".to_owned();
        }
        let mut chars: Vec<char> = palindrome.chars().collect();
        let mut i = 0;
        let mut j = palindrome.len() - 1;
        let mut ok = false;
        while i < j {
            if chars[i] != 'a' {
                chars[i] = 'a';
                ok = true;
                break;
            }
            i += 1;
            j -= 1;
        }
        if !ok {
            *chars.last_mut().unwrap() = 'b';
        }
        chars.into_iter().collect()
    }
}

fn main() {
    println!("{}", Solution::break_palindrome("aa".to_owned()));
}
