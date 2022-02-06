struct Solution;

impl Solution {
    pub fn count_vowels(word: String) -> i64 {
        let set = vec!['a', 'e', 'i', 'o', 'u'];
        let n = word.len();
        let mut ans = 0;
        for (i, c) in word.chars().enumerate() {
            if set.contains(&c) {
                ans += (i + 1) as i64 * (n - i) as i64;
            }
        }
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
