struct Solution;

impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for i in 0..nums.len() {
            if i == 0 {
                ans.push(*nums.last().unwrap() - nums[0]);
            } else if i == nums.len() - 1 {
                ans.push()
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
