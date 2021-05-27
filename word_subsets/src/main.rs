struct Solution;

impl Solution {
    pub fn word_subsets(a: Vec<String>, b: Vec<String>) -> Vec<String> {
        let mut limit = vec![0; 26];
        for s in b.into_iter() {
            let mut l = vec![0; 26];
            for c in s.chars() {
                l[(c as u8 - 'a' as u8) as usize] += 1;
            }
            for i in 0..26 {
                if l[i] > limit[i] {
                    limit[i] = l[i];
                }
            }
        }
        let mut ans: Vec<String> = Vec::new();
        'outer: for s in a.into_iter() {
            let mut l = vec![0; 26];
            for c in s.clone().chars() {
                l[(c as u8 - 'a' as u8) as usize] += 1;
            }
            for i in 0..26 {
                if l[i] < limit[i] {
                    continue 'outer;
                }
            }
            ans.push(s.clone())
        }
        ans
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::word_subsets(
            vec!["amazon", "apple", "facebook", "google", "leetcode"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
            vec!["e".to_owned(), "oo".to_owned()]
        )
    );
}
