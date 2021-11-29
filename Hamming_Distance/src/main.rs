struct Solution;

impl Solution {
    pub fn hamming_distance(mut x: i32, mut y: i32) -> i32 {
        let mut count = 0;
        while x > 0 || y > 0 {
            count += (x ^ y) & 1;
            x >>= 1;
            y >>= 1;
        }
        count
    }
}
fn main() {
    println!("{}", Solution::hamming_distance(3, 1));
}
