struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        let b = 1_u32;
        let mut n = n;
        let mut count = 0;
        for _ in 0..32 {
            count += n & b;
            n = n >> 1;
        }
        count as i32
    }
}
fn main() {
    println!("{}", Solution::hammingWeight(4));
}
