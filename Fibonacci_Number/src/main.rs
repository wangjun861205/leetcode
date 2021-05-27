struct Solution;

impl Solution {
    pub fn fib(n: i32) -> i32 {
        match n {
            0 => return 0,
            1 => return 1,
            _ => return Solution::fib(n - 1) + Solution::fib(n - 2),
        }
    }
}
fn main() {
    println!("{}", Solution::fib(4));
}
