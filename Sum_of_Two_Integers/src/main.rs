struct Solution;

impl Solution {
    pub fn get_sum(a: i32, b: i32) -> i32 {
        let mut ans = 0;
        let mut sup = 0;
        for i in 0..31 {
            let aa = a & (1 << i);
            let bb = b & (1 << i);
            if aa > 0 && bb > 0 {
                if sup == 0 {
                    sup = 1;
                } else {
                    ans |= 1 << i;
                }
                continue;
            }
            if aa == 0 && bb == 0 {
                if sup == 1 {
                    sup = 0;
                    ans |= 1 << i;
                }
                continue;
            }
            if sup == 0 {
                ans |= 1 << i;
            }
        }

        ans
    }
}

fn main() {
    println!("{}", Solution::get_sum(2, 100));
}
