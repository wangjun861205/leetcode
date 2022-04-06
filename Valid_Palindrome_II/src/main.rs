struct Solution;

impl Solution {
    fn rc(chars: &Vec<char>, mut i: usize, mut j: usize, has_deleted: bool) -> bool {
        while i < j {
            if chars[i] != chars[j] {
                if has_deleted {
                    return false;
                }
                if Solution::rc(chars, i + 1, j, true) {
                    return true;
                }
                if Solution::rc(chars, i, j - 1, true) {
                    return true;
                }
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
    pub fn valid_palindrome(s: String) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let i = 0;
        let j = chars.len() - 1;
        Solution::rc(&chars, i, j, false)
    }
}

fn main() {
    println!(
        "{}",
        Solution::valid_palindrome("ebcbbececabbacecbbcbe".into())
    );
}
