struct Solution;

impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut ans = 0;
        for i in 0..31 {
            if num == 0 {
                break;
            }
            let b = num & 1;
            if b == 0 {
                ans += 2_i32.pow(i);
            }
            num >>= 1;
        }
        ans
    }
}

fn main() {
    println!("{}", Solution::find_complement(5));
}
