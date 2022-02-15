struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut xor = 0;
        for n in nums {
            xor ^= n;
        }
        xor
    }
}

fn main() {
    println!("Hello, world!");
}
