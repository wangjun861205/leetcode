struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut presum: Vec<i32> = nums
            .iter()
            .scan(0, |s, v| {
                *s += *v;
                Some(*s)
            })
            .collect();
        presum.insert(0, 0);
        let mut result = vec![0; nums.len()];
        for i in 0..nums.len() {
            let left = nums[i] * i as i32 - presum[i];
            let right = presum[nums.len()] - presum[i + 1] - nums[i] * (nums.len() - i - 1) as i32;
            result[i] = left + right;
        }
        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10])
    );
}
