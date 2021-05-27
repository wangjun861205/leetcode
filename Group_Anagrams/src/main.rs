struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut ans: HashMap<Vec<char>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            ans.entry(chars).or_insert(Vec::new()).push(s);
        }
        ans.values().into_iter().map(|v| v.clone()).collect()
    }
}
fn main() {
    println!(
        "{:?}",
        Solution::group_anagrams(
            vec!["eat", "tea", "tan", "ate", "nat", "bat"]
                .into_iter()
                .map(|v| v.to_owned())
                .collect()
        )
    );
}
