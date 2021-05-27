struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut once = 0;
        let mut twice = 0;
        for n in nums {
            twice |= once & n;
            once ^= n;
            let mask = !(once & twice);
            twice &= mask;
            once &= mask;
        }
        once
    }
}
fn main() {
    println!("Hello, world!");
}
