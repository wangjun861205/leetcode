struct Solution;

use std::collections::HashSet;

impl Solution {
    fn rc(s: String, n: i32) -> HashSet<String> {
        if n == 0 {
            return vec![s].into_iter().collect();
        }
        let mut ans: HashSet<String> = HashSet::new();
        for i in 0..s.len() {
            let mut ss = s.clone();
            ss.insert_str(i, "()");
            ans = ans
                .union(&Solution::rc(ss, n - 1))
                .map(|v| v.clone())
                .collect();
        }
        ans
    }
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Solution::rc("()".to_owned(), n - 1).into_iter().collect()
    }
}
fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
