struct Solution;

impl Solution {
    pub fn remove_duplicates(s: String) -> String {
        let ans = s.chars().fold(Vec::new(), |mut l, c| {
            if l.len() == 0 {
                l.push(c);
            } else {
                if l.last().unwrap() == &c {
                    l.pop();
                } else {
                    l.push(c);
                }
            }
            l
        });
        ans.into_iter().collect()
    }
}
fn main() {
    println!("{}", Solution::remove_duplicates("abbaca".to_owned()));
}
