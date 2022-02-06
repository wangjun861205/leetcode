struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let actual = k % nums.len() as i32;
        let mut right: Vec<i32> = nums.drain(nums.len() - actual as usize..).collect();
        right.append(nums);
        *nums = right;
    }
}

fn main() {
    println!("Hello, world!");
}
