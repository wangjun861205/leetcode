struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let mut set1 = vec![0; 26];
        let mut set2 = set1.clone();
        for c in s1.chars() {
            set1[c as usize - 97] += 1;
        }
        for c in s2.chars().take(s1.len()) {
            set2[c as usize - 97] += 1;
        }
        let cmp = |l1: &Vec<i32>, l2: &Vec<i32>| -> bool {
            for (c1, c2) in l1.iter().zip(l2) {
                if c1 != c2 {
                    return false;
                }
            }
            true
        };
        if cmp(&set1, &set2) {
            return true;
        }
        let chars2: Vec<usize> = s2.chars().map(|c| c as usize - 97).collect();
        for i in s1.len()..s2.len() {
            set2[chars2[i]] += 1;
            set2[chars2[i - s1.len()]] -= 1;
            if cmp(&set1, &set2) {
                return true;
            }
        }
        false
    }
}

fn main() {
    println!(
        "{}",
        Solution::check_inclusion("ab".to_owned(), "eidbcaooo".to_owned())
    );
}
