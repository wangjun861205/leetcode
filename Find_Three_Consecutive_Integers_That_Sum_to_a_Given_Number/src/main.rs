struct Solution;

impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        if num % 3 != 0 {
            return vec![];
        }
        vec![num / 3 - 1, num / 3, num / 3 + 1]
    }
}
fn main() {}
