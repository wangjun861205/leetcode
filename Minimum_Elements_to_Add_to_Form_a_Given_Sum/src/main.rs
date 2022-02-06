struct Solution;

impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let sum: i128 = nums.into_iter().map(|v| v as i128).sum();
        let diff = goal as i128 - sum;
        let m = diff.abs() / limit as i128;
        let r = if diff.abs() % limit as i128 == 0 { 0 } else { 1 };
        (m + r) as i32
    }
}


fn main() {
}
