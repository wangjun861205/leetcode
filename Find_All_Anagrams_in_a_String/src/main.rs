struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        if p.len() > s.len() {
            return Vec::new();
        }
        let mut remains = vec![0; 26];
        let s_chars: Vec<char> = s.chars().collect();
        let p_chars: Vec<char> = p.chars().collect();
        for i in 0..p.len() {
            remains[p_chars[i] as usize - 97] -= 1;
            remains[s_chars[i] as usize - 97] += 1;
        }
        let mut start = 0;
        let mut end = p.len() - 1;
        let mut ans = Vec::new();
        if remains.iter().all(|v| v == &0) {
            ans.push(start as i32);
        }
        end += 1;
        while end < s.len() {
            remains[s_chars[start] as usize - 97] -= 1;
            start += 1;
            remains[s_chars[end] as usize - 97] += 1;
            if remains.iter().all(|v| v == &0) {
                ans.push(start as i32);
            }
            end += 1;
        }
        ans
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::find_anagrams("abab".to_owned(), "ab".to_owned())
    );
}
