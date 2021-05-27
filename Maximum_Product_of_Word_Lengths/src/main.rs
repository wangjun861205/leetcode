struct Solution;

impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let bits: Vec<i32> = words
            .iter()
            .map(|s| {
                s.chars().fold(0, |mut s, c| {
                    s |= 1 << (c as usize - 97);
                    s
                })
            })
            .collect();
        let mut ans = 0;
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if bits[i] & bits[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }
        ans as i32
    }
}
fn main() {
    println!("Hello, world!");
}
