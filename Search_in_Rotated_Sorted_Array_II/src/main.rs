struct Solution;

impl Solution {
    fn binary_search(nums: &Vec<i32>, target: i32, left: usize, right: usize) -> bool {
        if left == right {
            return nums[left] == target;
        }
        if nums[left] < nums[right] {
            if target < nums[left] || target > nums[right] {
                return false;
            }
        }
        Solution::binary_search(nums, target, left, (left + right) / 2)
            || Solution::binary_search(nums, target, (left + right) / 2 + 1, right)
    }
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Solution::binary_search(&nums, target, 0, nums.len() - 1)
    }
}

fn main() {
    println!("{}", Solution::search(vec![2, 5, 6, 0, 0, 1, 2], 3));
}
