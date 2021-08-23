struct Solution;

impl Solution {
    pub fn reach_number(mut target: i32) -> i32 {
        target = target.abs();
        let mut k = 0;
        while target > 0 {
            k += 1;
            target -= k;
        }
        if target == 0 {
            return k;
        }
        if target % 2 == 0 {
            return k;
        } else {
            return (k + 1) + k % 2;
        }
    }
}

fn main() {
    println!("{}", Solution::reach_number(4));
}
