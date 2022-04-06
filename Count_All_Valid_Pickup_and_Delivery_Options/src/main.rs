struct Solution;

impl Solution {
    pub fn count_orders(n: i32) -> i32 {
        let mut res = 1i128;
        for i in 1..=n as i128 {
            res = res * i % (10i128.pow(9) + 7);
        }
        for i in 1..=n as i128 {
            res = res * (2 * i - 1) % (10i128.pow(9) + 7);
        }
        res as i32
    }
}

fn main() {
    println!("{}", Solution::count_orders(2));
}
