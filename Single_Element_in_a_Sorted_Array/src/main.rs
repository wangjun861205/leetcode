struct Solution;

impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let m = (l + r) / 2;
            if m % 2 == 1 && nums[m - 1] == nums[m] || m % 2 == 0 && nums[m + 1] == nums[m] {
                l = m + 1;
            } else {
                r = m
            }
        }
        nums[l]
    }
}
fn main() {
    println!(
        "{}",
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11])
    );
}
