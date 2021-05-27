struct Solution;

impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        if n % 2 == 0 {
            n * n / 4
        } else {
            let half = n / 2;
            half * half + half
        }
    }
}
fn main() {
    println!("{}", Solution::min_operations(6));
}
