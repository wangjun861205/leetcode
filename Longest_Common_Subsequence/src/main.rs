struct Solution;

use std::collections::HashMap;

impl Solution {
    fn dp(chars1: &Vec<char>, chars2: &Vec<char>, m1: &Vec<HashMap<char, usize>>, m2: &Vec<HashMap<char, usize>>, i: usize, j: usize, cache: &mut HashMap<(usize, usize), i32>) -> i32 {
        if i == chars1.len() || j == chars2.len() {
            return 0;
        }
        if chars1[i] == chars2[j] {
            return if let Some(c) = cache.get(&(i + 1, j + 1)) {
                *c
            } else {
                Solution::dp(chars1, chars2, m1, m2, i + 1, j + 1, cache)
            } + 1;
        }
        let mut ans = Vec::new();
        let c1 = chars1[i];
        if let Some(k) = m2[j].get(&c1) {
            ans.push(
                if let Some(c) = cache.get(&(i + 1, *k + 1)) {
                    *c
                } else {
                    Solution::dp(chars1, chars2, m1, m2, i + 1, *k + 1, cache)
                } + 1,
            );
        }
        let c2 = chars2[j];
        if let Some(k) = m1[i].get(&c2) {
            ans.push(
                if let Some(c) = cache.get(&(*k + 1, j + 1)) {
                    *c
                } else {
                    Solution::dp(chars1, chars2, m1, m2, *k + 1, j + 1, cache)
                } + 1,
            );
        }
        ans.push(if let Some(c) = cache.get(&(i + 1, j + 1)) {
            *c
        } else {
            Solution::dp(chars1, chars2, m1, m2, i + 1, j + 1, cache)
        });
        let max = ans.into_iter().max().unwrap();
        cache.insert((i, j), max);
        max
    }
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let mut m1: Vec<HashMap<char, usize>> = text1
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .enumerate()
            .rev()
            .scan(HashMap::new(), |m, (i, c)| {
                *m.entry(c).or_insert(0) = i;
                Some(m.clone())
            })
            .collect();
        m1.reverse();
        let mut m2: Vec<HashMap<char, usize>> = text2
            .chars()
            .collect::<Vec<char>>()
            .into_iter()
            .enumerate()
            .rev()
            .scan(HashMap::new(), |m, (i, c)| {
                *m.entry(c).or_insert(0) = i;
                Some(m.clone())
            })
            .collect();
        m2.reverse();
        let mut cache = HashMap::new();
        Solution::dp(&text1.chars().collect(), &text2.chars().collect(), &m1, &m2, 0, 0, &mut cache)
    }
}

fn main() {
    println!("{}", Solution::longest_common_subsequence("ezupkr".to_owned(), "ubmrapg".to_owned()));
}
