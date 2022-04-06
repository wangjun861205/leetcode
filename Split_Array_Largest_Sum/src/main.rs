struct Solution;

impl Solution {
    pub fn split_array(nums: Vec<i32>, m: i32) -> i32 {
        let mut sum = 0;
        let mut maximum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            maximum = maximum.max(nums[i]);
        }
        let mut min = (sum / m).max(maximum);
        let mut max = sum;
        while min < max {
            let mid = (min + max) / 2;
            let mut count = 0;
            let mut curr = 0;
            for i in 0..nums.len() {
                if curr + nums[i] > mid {
                    count += 1;
                    curr = nums[i];
                } else {
                    curr += nums[i];
                }
            }
            count += 1;
            if count > m {
                min = mid + 1;
            } else {
                max = mid;
            }
        }
        min
    }
}
fn main() {
    println!("{}", Solution::split_array(vec![7, 2, 5, 10, 8], 2));
}
