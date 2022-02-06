struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>) -> i32 {
        let mut count = 0;
        if nums.iter().all(|v| v == &0) {
            return 0;
        }
        for i in 0..nums.len() {
            if nums[i] > 0 {
                if nums[i] % 2 == 1 {
                    nums[i] = (nums[i] - 1) / 2;
                    count += 1;
                } else {
                    nums[i] = nums[i] / 2;
                }
            }
        }
        if nums.iter().all(|v| v == &0) {
            return count;
        }
        count += 1;
        Solution::min_operations(nums) + count
    }
}

fn main() {
    println!("{}", Solution::min_operations(vec![1, 5]));
}
