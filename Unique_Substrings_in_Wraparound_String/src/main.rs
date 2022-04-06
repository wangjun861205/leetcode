struct Solution;

impl Solution {
    pub fn find_substring_in_wrapround_string(p: String) -> i32 {
        let chars: Vec<char> = p.chars().collect();
        let mut length = 1;
        let mut max_lengths = vec![0; 26];
        max_lengths[chars[0] as usize - 97] = 1;
        for i in 1..chars.len() {
            if chars[i] as usize == chars[i - 1] as usize + 1
                || (chars[i] == 'a' && chars[i - 1] == 'z')
            {
                length += 1;
            } else {
                length = 1;
            }
            max_lengths[chars[i] as usize - 97] = max_lengths[chars[i] as usize - 97].max(length);
        }
        max_lengths.into_iter().sum()
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_substring_in_wrapround_string("zab".into())
    );
}
