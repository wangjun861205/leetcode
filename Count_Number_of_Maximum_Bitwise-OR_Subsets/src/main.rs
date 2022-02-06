struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max = nums.iter().fold(0, |mut s, v| {
            s |= *v;
            s
        });
        let mut count = 0;
        for i in 1..2_i32.pow(nums.len() as u32) {
            let mut result = 0;
            for j in 0..nums.len() {
                if (i >> j) & 1 == 1 {
                    result |= nums[j];
                }
            }
            if result == max {
                count += 1;
            }
        }
        count
    }
}

fn main() {
    println!("Hello, world!");
}
