struct Solution;

impl Solution {
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let is_palindrome = |w1: &String, w2: &String| {
            for (c1, c2) in w1
                .chars()
                .chain(w2.chars())
                .zip(w1.chars().chain(w2.chars()).rev())
            {
                if c1 != c2 {
                    return false;
                }
            }
            true
        };
        let mut ans = Vec::new();
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if is_palindrome(&words[i], &words[j]) {
                    ans.push(vec![i as i32, j as i32]);
                }
                if is_palindrome(&words[j], &words[i]) {
                    ans.push(vec![j as i32, i as i32]);
                }
            }
        }
        ans
    }
}
fn main() {
    println!("Hello, world!");
}
