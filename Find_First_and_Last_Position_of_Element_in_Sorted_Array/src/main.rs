struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![-1, -1];
        }
        if nums.len() == 1 {
            if nums[0] == target {
                return vec![0, 0];
            } else {
                return vec![-1, -1];
            }
        }
        let mut left = 0;
        let mut right = nums.len() - 1;
        while left <= right {
            if nums[left] == target && nums[right] == target {
                return vec![left as i32, right as i32];
            } else {
                if nums[left] != target {
                    left += 1;
                }
                if nums[right] != target {
                    right -= 1;
                }
            }
        }
        vec![-1, -1]
    }
}
fn main() {
    println!("Hello, world!");
}
