struct Solution;

impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        for i in 0..32 {
            let mut ones = 0;
            let mut zeros = 0;
            let mask = 1 << i;
            for j in 0..nums.len() {
                if nums[j] & mask > 0 {
                    ones += 1;
                } else {
                    zeros += 1;
                }
            }
            total += ones * zeros;
        }
        total
    }
}

fn main() {
    println!("Hello, world!");
}
