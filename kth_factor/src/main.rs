struct Solution;

impl Solution {
    pub fn kth_factor(n: i32, k: i32) -> i32 {
        let mut count = 0;
        for i in 1..=n {
            if n % i == 0 {
                count += 1;
                if count == k {
                    return i
                }
            }
        }   
        -1
    }
}

fn main() {
    println!("{}", Solution::kth_factor(12, 3));
}
