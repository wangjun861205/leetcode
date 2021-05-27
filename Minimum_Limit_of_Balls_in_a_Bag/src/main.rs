struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let mut left = 1;
        let mut right = 1e9 as i32;
        while left < right {
            let mid = (right + left) / 2;
            if nums.iter().map(|v| (*v - 1) / mid).sum::<i32>() > max_operations {
                left = mid + 1;
            } else {
                right = mid;
            }
        }
        left
    }
}
fn main() {
    println!("Hello, world!");
}
