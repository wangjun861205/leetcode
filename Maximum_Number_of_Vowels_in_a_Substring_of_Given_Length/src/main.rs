struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let vowel_set: HashSet<char> = vec!['a', 'e', 'i', 'o', 'u'].into_iter().collect();
        let chars: Vec<char> = s.chars().collect();
        let mut stack: Vec<char> = Vec::new();
        let mut sum = 0;
        let mut ans = 0;
        for c in chars {
            if vowel_set.contains(&c) {
                sum += 1;
            }
            stack.push(c);
            if stack.len() > k as usize {
                let first_char = stack.remove(0);
                if vowel_set.contains(&first_char) {
                    sum -= 1;
                }
            }
            ans = ans.max(sum);
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::max_vowels("aeiou".to_owned(), 2));
}
