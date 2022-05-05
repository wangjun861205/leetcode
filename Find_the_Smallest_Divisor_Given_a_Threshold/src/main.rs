struct Solution;

impl Solution {
    pub fn smallest_divisor(nums: Vec<i32>, threshold: i32) -> i32 {
        let sum = nums.iter().sum::<i32>() as i64;
        let mut low = (sum / threshold as i64).max(1);
        let mut high = *nums.iter().max().unwrap() as i64;
        while low < high {
            let mid = (low + high) / 2;
            let total = nums.iter().fold(0, |mut s, v| {
                s += if *v as i64 % mid == 0 {
                    *v as i64 / mid
                } else {
                    *v as i64 / mid + 1
                };
                s
            });
            if total <= threshold as i64 {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        low as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::smallest_divisor(vec![21212, 10101, 12121], 1000000)
    );
}
