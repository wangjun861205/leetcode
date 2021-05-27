struct Solution;

impl Solution {
    fn is_one_byte(data: i32) -> bool {
        data & 0b10000000 == 0b00000000
    }
    fn is_two_bytes(data: i32) -> bool {
        data & 0b11100000 == 0b11000000
    }
    fn is_three_bytes(data: i32) -> bool {
        data & 0b11110000 == 0b11100000
    }
    fn is_four_bytes(data: i32) -> bool {
        data & 0b11111000 == 0b11110000
    }
    fn is_valid_multi(data: i32) -> bool {
        data & 0b11000000 == 0b10000000
    }
    pub fn valid_utf8(data: Vec<i32>) -> bool {
        let mut is_multi = false;
        let mut curr_byte = 0;
        for d in data {
            if !is_multi {
                if Solution::is_one_byte(d) {
                    continue;
                } else if Solution::is_two_bytes(d) {
                    is_multi = true;
                    curr_byte = 1;
                } else if Solution::is_three_bytes(d) {
                    is_multi = true;
                    curr_byte = 2;
                } else if Solution::is_four_bytes(d) {
                    is_multi = true;
                    curr_byte = 3;
                } else {
                    return false;
                }
            } else {
                if !Solution::is_valid_multi(d) {
                    return false;
                }
                curr_byte -= 1;
                if curr_byte == 0 {
                    is_multi = false;
                }
            }
        }
        return !is_multi;
    }
}
fn main() {
    println!("Hello, world!");
}
