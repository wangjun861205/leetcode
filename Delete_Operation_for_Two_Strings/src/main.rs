struct Solution;

use std::collections::HashMap;

impl Solution {
    fn lcs(
        word1: &Vec<char>,
        word2: &Vec<char>,
        i1: usize,
        i2: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if i1 == word1.len() || i2 == word2.len() {
            return 0;
        }
        if word1[i1] == word2[i2] {
            if let Some(v) = cache.get(&(i1 + 1, i2 + 1)) {
                return *v + 1;
            } else {
                let ans = Solution::lcs(word1, word2, i1 + 1, i2 + 1, cache) + 1;
                cache.insert((i1, i2), ans);
                return ans;
            }
        } else {
            let ans1 = if let Some(v) = cache.get(&(i1 + 1, i2)) {
                *v
            } else {
                let ans = Solution::lcs(word1, word2, i1 + 1, i2, cache);
                ans
            };
            let ans2 = if let Some(v) = cache.get(&(i1, i2 + 1)) {
                *v
            } else {
                let ans = Solution::lcs(word1, word2, i1, i2 + 1, cache);
                ans
            };
            let ans = ans1.max(ans2);
            cache.insert((i1, i2), ans);
            return ans;
        }
    }
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let mut cache = HashMap::new();
        word1.len() as i32 + word2.len() as i32
            - 2 * Solution::lcs(
                &word1.chars().collect(),
                &word2.chars().collect(),
                0,
                0,
                &mut cache,
            )
    }
}
fn main() {
    println!(
        "{}",
        Solution::min_distance("leetcode".to_owned(), "etco".to_owned())
    );
}
