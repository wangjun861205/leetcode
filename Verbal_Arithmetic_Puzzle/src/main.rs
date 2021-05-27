struct Solution;

use std::collections::HashMap;

impl Solution {
    fn matrix(words: Vec<String>, result: String) -> Vec<Vec<char>> {
        result
            .chars()
            .rev()
            .enumerate()
            .map(|(i, v)| {
                let mut l: Vec<char> = words.iter().map(|v| if i < v.len() { v.chars().rev().nth(i).unwrap() } else { '-' }).collect();
                l.insert(0, v);
                l
            })
            .collect()
    }
    fn rc(mut word_list_list: Vec<Vec<char>>, mut result_list: Vec<char>, result_char: char, index: usize, target: i32, step: i32, cache: HashMap<char, i32>) -> bool {
        match index {
            idx if idx == word_list_list.len() => {}
            idx if idx == word_list_list.len() - 1 => {}
            _ => {}
        }
        false
    }
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut wl: Vec<Vec<char>> = words.into_iter().map(|s| s.chars().collect()).collect();
        let mut rl: Vec<char> = result.chars().collect();
        let cache = HashMap::new();
        Solution::rc(wl, rl, 0, 0, 0, cache)
    }
}
fn main() {
    println!("{:?}", Solution::matrix(vec!["SEND".to_owned(), "MORE".to_owned()], "MONEY".to_owned()));
}
