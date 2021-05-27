struct Solution;

impl Solution {
    pub fn h_index(mut citations: Vec<i32>) -> i32 {
        citations.sort();
        let len = citations.len();
        let mut ans = 0;
        for (i, c) in citations.into_iter().enumerate() {
            if c >= (len - i) as i32 {
                ans = ans.max((len - i) as i32)
            }
        }
        ans
    }
}
fn main() {
    println!("{}", Solution::h_index(vec![3, 0, 6, 1, 5]));
}
