struct Solution;

impl Solution {
    fn rc(
        chars1: Vec<char>,
        chars2: Vec<char>,
        chars3: Vec<char>,
        cache: &mut Vec<Vec<Vec<bool>>>,
    ) -> bool {
        if chars3.len() == 0 {
            return true;
        }
        if let Some(c) = chars1.get(0) {
            if c == &chars3[0] {
                let chars1 = chars1[1..].to_vec();
                let chars3 = chars3[1..].to_vec();
                if !cache[chars1.len()][chars2.len()][chars3.len()] {
                    if Solution::rc(chars1, chars2.clone(), chars3, cache) {
                        return true;
                    }
                }
            }
        }
        if let Some(c) = chars2.get(0) {
            if c == &chars3[0] {
                let chars2 = chars2[1..].to_vec();
                let chars3 = chars3[1..].to_vec();
                if !cache[chars1.len()][chars2.len()][chars3.len()] {
                    if Solution::rc(chars1.clone(), chars2, chars3, cache) {
                        return true;
                    }
                }
            }
        }
        cache[chars1.len()][chars2.len()][chars3.len()] = true;
        false
    }
    pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let chars3: Vec<char> = s3.chars().collect();
        let mut cache: Vec<Vec<Vec<bool>>> =
            vec![vec![vec![false; chars3.len() + 1]; chars2.len() + 1]; chars1.len() + 1];
        if chars1.len() + chars2.len() != chars3.len() {
            return false;
        }
        Solution::rc(chars1, chars2, chars3, &mut cache)
    }
}
fn main() {
    println!("Hello, world!");
}
