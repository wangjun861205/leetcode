struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 5001;
        }
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return nums[0].min(nums[1]);
        }
        let mid = nums[nums.len() / 2];
        let first = nums[0];
        let last = nums[nums.len() - 1];
        if mid > first && mid < last {
            return first;
        }
        let left = Solution::find_min(nums[..nums.len() / 2].to_vec());
        let right = Solution::find_min(nums[nums.len() / 2..].to_vec());
        left.min(right)
    }
}

fn main() {
    println!("{}", Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]));
}
