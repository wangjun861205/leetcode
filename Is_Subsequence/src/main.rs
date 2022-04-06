struct Solution;

impl Solution {
    pub fn is_subsequence(mut s: String, mut t: String) -> bool {
        if s.is_empty() {
            return true;
        }
        let sc = s.remove(0);
        while t.len() > s.len() {
            let tc = t.remove(0);
            if sc == tc {
                return Solution::is_subsequence(s, t);
            }
        }
        false
    }
}

fn main() {
    println!("{}", Solution::is_subsequence("b".into(), "c".into()));
}
