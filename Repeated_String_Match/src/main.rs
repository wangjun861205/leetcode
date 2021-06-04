struct Solution;

impl Solution {
    pub fn repeated_string_match(a: String, b: String) -> i32 {
        let times = b.len() / a.len();
        if a.repeat(times) == b {
            return times as i32;
        }
        let repeated = a.repeat(times + 2);
        let a_chars: Vec<char> = repeated.chars().collect();
        for (i, w) in a_chars.windows(b.len()).enumerate() {
            if w.to_vec().iter().collect::<String>() == b {
                if i < a.len() {
                    if a_chars.len() - (i + b.len()) >= a.len() {
                        return times as i32 + 1;
                    } else {
                        return times as i32 + 2;
                    }
                }
                return times as i32 + 1;
            }
        }
        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::repeated_string_match("abcd".to_owned(), "cdabcdab".to_owned())
    );
}
