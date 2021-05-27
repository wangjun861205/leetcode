struct Solution;

use std::collections::HashMap;

impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
        let exact: HashMap<String, String> = wordlist
            .iter()
            .map(|s| (s.to_owned(), s.to_owned()))
            .collect();
        let mut icase: HashMap<String, String> = HashMap::new();
        for s in wordlist.iter() {
            icase.entry(s.to_lowercase()).or_insert(s.to_owned());
        }
        let mut vowel: HashMap<String, String> = HashMap::new();
        for s in wordlist.iter() {
            let v: String = s
                .to_lowercase()
                .chars()
                .map(|c| match c {
                    'a' | 'e' | 'i' | 'o' | 'u' => '-',
                    _ => c,
                })
                .collect();
            vowel.entry(v).or_insert(s.to_owned());
        }
        queries
            .iter()
            .map(|s| {
                if exact.contains_key(s) {
                    return exact.get(s).unwrap().to_owned();
                }
                if icase.contains_key(&s.to_lowercase()) {
                    return icase.get(&s.to_lowercase()).unwrap().to_owned();
                }
                let v: String = s
                    .to_lowercase()
                    .chars()
                    .map(|c| match c {
                        'a' | 'e' | 'i' | 'o' | 'u' => '-',
                        _ => c,
                    })
                    .collect();
                if vowel.contains_key(&v) {
                    return vowel.get(&v).unwrap().to_owned();
                }
                return "".to_owned();
            })
            .collect()
    }
}

// ["kite","KiTe","KiTe","Hare","hare","","","KiTe","","KiTe"]
fn main() {
    println!(
        "{:?}",
        Solution::spellchecker(
            vec!["KiTe", "kite", "hare", "Hare"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
            vec!["kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect()
        )
    );
}
