struct Solution;

impl Solution {
    fn rc(l: Vec<char>) -> i32 {
        if l.len() == 0 {
            return 0;
        }
        let mut ans = 0;
        let mut i = 0_usize;
        let mut j = l.len() - 1;
        while i < j {
            if l[..i + 1].iter().collect::<String>() == l[j..].iter().collect::<String>() {
                ans += Solution::rc(l[i + 1..j].to_vec()) + 2;
                break;
            } else {
                i += 1;
                j -= 1;
            }
        }
        if ans == 0 {
            1
        } else {
            ans
        }
    }
    pub fn longest_decomposition(text: String) -> i32 {
        Solution::rc(text.chars().collect())
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_decomposition("elvtoelvto".to_owned())
    );
}
