struct Solution;

impl Solution {
    pub fn is_power_of_two(mut n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        while n > 1 {
            if n % 2 != 0 {
                return false;
            }
            n /= 2;
        }
        true
    }
}
fn main() {
    println!("Hello, world!");
}
