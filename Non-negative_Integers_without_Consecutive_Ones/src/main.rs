struct Solution;

impl Solution {
    pub fn find_integers(mut n: i32) -> i32 {
        let (mut x, mut y) = (1, 2);
        let mut res = 0;
        n += 1;
        while n > 0 {
            if (n & 1 > 0) && (n & 2 > 0) {
                res = 0;
            }
            res += x * (n & 1);
            n >>= 1;
            let temp_x = x;
            x = y;
            y = temp_x + y;
        }
        return res;
    }
}
fn main() {
    println!("{}", Solution::find_integers(8));
}
