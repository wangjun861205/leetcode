struct Solution;

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        3_f64.powi(19) as i32 % n == 0
    }
}
fn main() {
    println!("{}", 3_f64.powi(19) as i32 % 28);
}
