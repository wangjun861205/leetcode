struct Solution;

impl Solution {
    pub fn is_ideal_permutation(a: Vec<i32>) -> bool {
        if a.len() == 1 {
            return true;
        }
        for (i, v) in a.into_iter().enumerate() {
            if (v - i as i32).abs() > 1 {
                return false;
            }
        }
        true
    }
}
fn main() {
    println!("{}", Solution::is_ideal_permutation(vec![1, 2, 0]));
}
