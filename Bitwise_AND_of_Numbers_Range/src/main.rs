struct Solution;

//  0 -> 00000 2 ** 0 - 1
//  1 -> 00001 2 ** 1 - 1
//  2 -> 00010
//  3 -> 00011 2 ** 2 - 1
//  4 -> 00100
//  5 -> 00101
//  6 -> 00110
//  7 -> 00111 2 ** 3 - 1
//  8 -> 01000
//  9 -> 01001
// 10 -> 01010
// 11 -> 01011
// 12 -> 01100
// 13 -> 01101
// 14 -> 01110
// 15 -> 01111 2 ** 4 - 1
// 16 -> 10000

// [2 ** n,  2 ** (n+1) - 1]   此范围内二进制第n-1位为1， 其他情况所有位都为0

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == 0 {
            return 0;
        }
        if left == right {
            return left;
        }
        for n in 0..31 {
            if left >= 2_i32.pow(n) && right <= 2_i32.pow(n + 1) - 1 {
                return 1 << n | Solution::range_bitwise_and(left - (1 << n), right - (1 << n));
            }
        }
        return 0;
    }
}
fn main() {
    println!("{}", Solution::range_bitwise_and(5, 7));
}
