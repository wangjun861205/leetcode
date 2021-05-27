struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn is_happy(mut n: i32) -> bool {
        let mut set = HashSet::new();
        set.insert(n);
        loop {
            let mut num = n;
            let mut sum = 0;
            while num > 0 {
                let d = num % 10;
                sum += d.pow(2);
                num = num / 10;
            }
            if sum == 1 {
                return true;
            }
            if set.contains(&sum) {
                return false;
            }
            set.insert(sum);
            n = sum;
        }
    }
}
fn main() {
    println!("{}", Solution::is_happy(19));
}
