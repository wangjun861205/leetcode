struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn count_vowel_strings_with_cache(
        n: i32,
        nn: i32,
        m: &mut HashMap<(i32, i32), i32>,
    ) -> i32 {
        if n == 0 {
            return 1;
        }
        if nn == 0 {
            return 0;
        }
        if m.contains_key(&(n, nn)) {
            return m.get(&(n, nn)).unwrap().clone();
        }
        let sum = (0..=n)
            .map(|v| Solution::count_vowel_strings_with_cache(n - v, nn - 1, m))
            .sum();
        m.insert((n, nn), sum);
        sum
    }

    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut m: HashMap<(i32, i32), i32> = HashMap::new();
        Solution::count_vowel_strings_with_cache(n, 5, &mut m)
    }
}
fn main() {
    println!("{}", Solution::count_vowel_strings(33));
}
