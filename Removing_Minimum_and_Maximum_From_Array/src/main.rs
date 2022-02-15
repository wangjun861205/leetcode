struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let mut min_idx = -1;
        let mut max_idx = -1;
        let mut min = i32::MAX;
        let mut max = i32::MIN;
        let length = nums.len() as i32;
        for (i, n) in nums.into_iter().enumerate() {
            if n < min {
                min_idx = i as i32;
                min = n;
            }
            if n > max {
                max_idx = i as i32;
                max = n;
            }
        }
        let both = (min_idx.min(max_idx)) + 1 + length - (min_idx.max(max_idx));
        let left = min_idx.max(max_idx) + 1;
        let right = length - min_idx.min(max_idx);
        both.min(left).min(right)
    }
}
fn main() {
    println!(
        "{}",
        Solution::minimum_deletions(vec![0, -4, 19, 1, 8, -2, -3, 5])
    );
}
