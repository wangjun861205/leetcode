struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(c1: &Vec<char>, c2: &Vec<char>, i: usize, j: usize, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        let mut ans = 0;
        if i == c1.len() || j == c2.len() {
            for c in c1[i..].to_vec() {
                ans += c as i32;
            }
            for c in c2[j..].to_vec() {
                ans += c as i32;
            }
            return ans;
        }
        let not_remove = if c1[i] == c2[j] {
            if let Some(c) = cache.get(&(i + 1, j + 1)) {
                *c
            } else {
                Solution::dp(c1, c2, i + 1, j + 1, cache)
            }
        } else {
            c1.len().max(c2.len()) as i32 * 123 * 2
        };
        let remove1 = c1[i] as i32 + if let Some(c) = cache.get(&(i + 1, j)) { *c } else { Solution::dp(c1, c2, i + 1, j, cache) };
        let remove2 = c2[j] as i32 + if let Some(c) = cache.get(&(i, j + 1)) { *c } else { Solution::dp(c1, c2, i, j + 1, cache) };
        ans = not_remove.min(remove1).min(remove2);
        cache.insert((i, j), ans);
        ans
    }

    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut cache = HashMap::new();
        Solution::dp(&s1.chars().collect(), &s2.chars().collect(), 0, 0, &mut cache)
    }
}

fn main() {
    println!("{}", Solution::minimum_delete_sum("ccaccjp".to_owned(), "fwosarcwge".to_owned()));
}
